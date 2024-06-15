mod error;
mod list;
mod location;
mod node;
pub mod parser;
pub mod tokenizer;

pub use list::*;
pub use location::*;
pub use node::*;

pub mod parse {
    use super::*;

    /// Parses the given contents into a vec of nodes.
    pub fn parse_str<'a>(contents: &'a str, ignore_comments: bool) -> Result<Vec<List>, Error> {
        parse_optional_path(contents, "list", ignore_comments, None)
    }

    /// Parse the given contents from a file into a vec of nodes.
    pub fn parse_file<'a>(
        contents: &'a str,
        msg: &str,
        ignore_comments: bool,
        path: std::path::PathBuf,
    ) -> Result<Vec<List>, Error> {
        parse_optional_path(contents, msg, ignore_comments, Some(path))
    }

    fn parse_optional_path<'a>(
        contents: &'a str,
        msg: &str,
        ignore_comments: bool,
        path: Option<std::path::PathBuf>,
    ) -> Result<Vec<List>, Error> {
        let tokens = tokenizer::Tokenizer::tokenize(contents, path)?;
        let nodes = parser::Parser::parse(tokens)?;

        let mut lists = vec![];
        for node in nodes {
            match list::list(&node, msg, ignore_comments) {
                Ok(l) => lists.push(l),
                Err(e) => return Err(Error::Invalid(e)),
            }
        }

        Ok(lists)
    }
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
