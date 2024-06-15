mod error;
mod list;
mod location;
mod node;
pub mod parser;
pub mod tokenizer;

pub use list::*;
pub use location::*;
pub use node::*;

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
    let tokens = tokenizer::Tokenizer::tokenize(contents, path)?;
    let nodes = parser::Parser::parse(tokens)?;
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
            Err(e) => return Err(Error::Invalid(e)),
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

/// Errors that may occur during parsing.
#[derive(Debug, Clone, PartialEq)]
pub enum Error {
    Invalid(String),
    Tokenizer(tokenizer::Err),
    Parser(parser::Err),
}
impl ToString for Error {
    fn to_string(&self) -> String {
        match self {
            Self::Tokenizer(err) => err.clone().to_string(),
            Self::Parser(err) => err.clone().to_string(),
            Self::Invalid(msg) => msg.clone(),
        }
    }
}

impl From<parser::Err> for Error {
    fn from(err: parser::Err) -> Self {
        Self::Parser(err)
    }
}

impl From<tokenizer::Err> for Error {
    fn from(err: tokenizer::Err) -> Self {
        Self::Tokenizer(err)
    }
}
