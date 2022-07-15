mod error;
mod location;
mod parser;
mod tokenizer;

pub use location::Location;
pub use parser::Node;

/// Parse the given contents into a vec of nodes.
pub fn parse<'a>(contents: &'a str, path: std::path::PathBuf) -> Result<Vec<Node>, Error> {
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
