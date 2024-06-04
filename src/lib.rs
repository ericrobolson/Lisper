mod error;
mod location;
mod node;
mod parser;
mod tokenizer;

pub use location::*;
pub use node::*;
pub use tokenizer::*;

/// Parses the given contents into a vec of nodes.
pub fn parse_str<'a>(contents: &'a str) -> Result<Vec<Node>, Error> {
    parse_optional_path(contents, None)
}

/// Parse the given contents from a file into a vec of nodes.
pub fn parse_file<'a>(contents: &'a str, path: std::path::PathBuf) -> Result<Vec<Node>, Error> {
    parse_optional_path(contents, Some(path))
}

fn parse_optional_path<'a>(
    contents: &'a str,
    path: Option<std::path::PathBuf>,
) -> Result<Vec<Node>, Error> {
    let tokens = tokenizer::Tokenizer::tokenize(contents, path)?;
    let nodes = parser::Parser::parse(tokens)?;

    Ok(nodes)
}

/// Errors that may occur during parsing.
#[derive(Debug, Clone, PartialEq)]
pub enum Error {
    Tokenizer(tokenizer::Err),
    Parser(parser::Err),
}
impl ToString for Error {
    fn to_string(&self) -> String {
        match self {
            Self::Tokenizer(err) => err.clone().to_string(),
            Self::Parser(err) => err.clone().to_string(),
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
