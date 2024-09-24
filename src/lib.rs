mod error;
mod list;
mod location;
mod node;
pub mod parser;
pub mod tokenizer;

pub use list::*;
pub use location::*;
pub use node::*;
use parser::{ListErr, ParserErr};
use tokenizer::{IdentifierErr, TokenErr, TokenType, TypeErr};

#[derive(Debug, Clone)]
pub struct Error {
    pub message: String,
    pub location: Location,
}
impl Error {
    pub fn new(message: String, location: Location) -> Self {
        Self { message, location }
    }
}
impl PartialEq for Error {
    fn eq(&self, other: &Self) -> bool {
        self.message == other.message
    }
}
impl Into<String> for Error {
    fn into(self) -> String {
        if let Some(path) = self.location.path {
            format!(
                "{}:{}:{}: {}",
                path.display(),
                self.location.line,
                self.location.column,
                self.message
            )
        } else {
            format!(
                "{}:{}: {}",
                self.location.line, self.location.column, self.message
            )
        }
    }
}
impl From<String> for Error {
    fn from(message: String) -> Self {
        Self {
            message,
            location: Location::default(),
        }
    }
}
impl From<&str> for Error {
    fn from(message: &str) -> Self {
        Self {
            message: message.to_string(),
            location: Location::default(),
        }
    }
}

#[cfg(feature = "load_directory")]
pub fn load_directory(extension: &str, location: std::path::PathBuf) -> Result<Vec<List>, Error> {
    if !location.is_dir() {
        return Err(format!("Location '{:?}' is not a directory", location).into());
    }

    let mut files = vec![];
    // Use walkdir and traverse directory recursively, getting all files with the extension
    for entry in walkdir::WalkDir::new(location) {
        let entry = entry.unwrap();
        if entry.path().is_file() {
            if let Some(ext) = entry.path().extension() {
                if ext == extension {
                    files.push(entry.path().to_path_buf());
                }
            }
        }
    }

    // Now read each file and parse it
    let mut lists = vec![];
    for file in files {
        let contents = match std::fs::read_to_string(&file) {
            Ok(contents) => contents,
            Err(e) => return Err(format!("Error reading file '{:?}': {}", file, e).into()),
        };

        match parse_file(&contents, file) {
            Ok(mut l) => lists.append(&mut l),
            Err(e) => return Err(e),
        }
    }

    Ok(lists)
}

/// Parses the given contents into a vec of lists.
/// Will ignore comments.
pub fn parse_str<'a>(contents: &'a str) -> Result<Vec<List>, Error> {
    parse_optional_path(contents, None)
}

/// Parse the given contents from a file into a vec of lists.
/// Will ignore comments.
pub fn parse_file<'a>(contents: &'a str, path: std::path::PathBuf) -> Result<Vec<List>, Error> {
    parse_optional_path(contents, Some(path))
}

fn parse_optional_path<'a>(
    contents: &'a str,
    path: Option<std::path::PathBuf>,
) -> Result<Vec<List>, Error> {
    let tokens = match tokenizer::Tokenizer::tokenize(contents, path) {
        Ok(tokens) => tokens,
        Err(e) => {
            let msg: String = match e.kind {
                TokenErr::Comment(c) => match c {
                    tokenizer::CommentErr::NotStarted => "Comment not started".into(),
                },
                TokenErr::String(s) => match s {
                    tokenizer::StringErr::NotStarted => "String not started".into(),
                    tokenizer::StringErr::Unclosed(e) => format!("Unclosed string: {}", e.contents),
                },
                TokenErr::Type(t) => match t {
                    TypeErr::WrongType { got: _, expected } => {
                        let ty = match expected {
                            TokenType::Bool => "bool",
                            TokenType::Comment => "comment",
                            TokenType::Identifier => "identifier",
                            TokenType::Number => "number",
                            TokenType::String => "string",
                            TokenType::Symbol => "symbol",
                        };
                        format!("Expected type {}", ty)
                    }
                },
                TokenErr::Identifier(i) => match i {
                    IdentifierErr::NotStarted => "Identifier not started".into(),
                    IdentifierErr::BeginsWithNumber { got } => {
                        format!("Identifier begins with number: {}", got)
                    }
                },
                TokenErr::StackUnderflow => "Stack underflow".into(),
            };
            return err(&msg, &e.location);
        }
    };
    let nodes = match parser::Parser::parse(tokens) {
        Ok(nodes) => nodes,
        Err(e) => {
            let msg: String = match e.kind {
                ParserErr::Invalid(e) => format!("Invalid: {}", e),
                ParserErr::List(l) => match l {
                    ListErr::UnclosedList => "Unclosed list".into(),
                    ListErr::UnstartedList => "List not started".into(),
                },
                ParserErr::StackUnderflow => "Stack underflow".into(),
            };

            return err(&msg, &e.location);
        }
    };
    let nodes = nodes
        .iter()
        .filter(|n| !n.is_comment())
        .map(|n| strip_comments(n))
        .filter_map(|n| n)
        .collect::<Vec<_>>();

    let mut lists = vec![];
    for node in nodes {
        match list::list(&node, "list") {
            Ok(l) => lists.push(l),
            Err(e) => return Err(e),
        }
    }

    Ok(lists)
}

fn strip_comments(node: &Node) -> Option<Node> {
    let node = match &node.ast {
        Ast::Comment(_) => return None,
        Ast::List(nodes) => {
            let nodes = nodes.iter().filter_map(|n| strip_comments(n)).collect();

            Node {
                ast: Ast::List(nodes),
                tokens: node.tokens.clone(),
            }
        }
        Ast::Identifier(_) | Ast::Number(_) | Ast::String(_) | Ast::Bool(_) => node.clone(),
    };

    Some(node)
}
