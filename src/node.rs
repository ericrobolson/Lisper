use crate::{tokenizer::Token, Location};

#[derive(Debug, Clone, PartialEq)]
pub enum NodeError {
    InvalidType {
        expected: AstType,
        got: AstType,
        location: Location,
    },
    InvalidLength {
        expected: usize,
        got: usize,
        location: Location,
    },
}

#[derive(Debug, Clone, PartialEq)]
pub enum Ast {
    Bool(bool),
    Comment(String),
    Identifier(String),
    List(Vec<Node>),
    Number(f64),
    String(String),
}
impl Ast {
    pub fn type_(&self) -> AstType {
        match self {
            Ast::Bool(_) => AstType::Bool,
            Ast::Comment(_) => AstType::Comment,
            Ast::Identifier(_) => AstType::Identifier,
            Ast::List(_) => AstType::List,
            Ast::Number(_) => AstType::Number,
            Ast::String(_) => AstType::String,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum AstType {
    Bool,
    Comment,
    Identifier,
    List,
    Number,
    String,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Node {
    pub ast: Ast,
    pub tokens: Vec<Token>,
}
impl Node {
    pub fn first_location(&self) -> Location {
        if self.tokens.is_empty() {
            Location::default()
        } else {
            self.tokens[0].location.clone()
        }
    }

    pub fn is_comment(&self) -> bool {
        match &self.ast {
            Ast::Comment(_) => true,
            _ => false,
        }
    }

    pub fn assert_length(&self, expected: usize) -> Result<(), NodeError> {
        let l = self.as_list()?;
        if l.len() != expected {
            return Err(NodeError::InvalidLength {
                expected,
                got: l.len(),
                location: self.first_location(),
            });
        } else {
            return Ok(());
        }
    }

    pub fn as_bool(&self) -> Result<bool, NodeError> {
        match &self.ast {
            Ast::Bool(value) => Ok(*value),
            value => Err(NodeError::InvalidType {
                expected: AstType::Bool,
                got: value.type_(),
                location: self.first_location(),
            }),
        }
    }

    pub fn as_comment(&self) -> Result<String, NodeError> {
        match &self.ast {
            Ast::Comment(value) => Ok(value.clone()),
            value => Err(NodeError::InvalidType {
                expected: AstType::Comment,
                got: value.type_(),
                location: self.first_location(),
            }),
        }
    }

    pub fn as_identifier(&self) -> Result<String, NodeError> {
        match &self.ast {
            Ast::Identifier(value) => Ok(value.clone()),
            value => Err(NodeError::InvalidType {
                expected: AstType::Identifier,
                got: value.type_(),
                location: self.first_location(),
            }),
        }
    }

    pub fn as_list(&self) -> Result<Vec<Node>, NodeError> {
        match &self.ast {
            Ast::List(nodes) => Ok(nodes.clone()),
            value => Err(NodeError::InvalidType {
                expected: AstType::List,
                got: value.type_(),
                location: self.first_location(),
            }),
        }
    }

    pub fn as_number(&self) -> Result<f64, NodeError> {
        match &self.ast {
            Ast::Number(value) => Ok(*value),
            value => Err(NodeError::InvalidType {
                expected: AstType::Number,
                got: value.type_(),
                location: self.first_location(),
            }),
        }
    }

    pub fn as_string(&self) -> Result<String, NodeError> {
        match &self.ast {
            Ast::String(value) => Ok(value.clone()),
            value => Err(NodeError::InvalidType {
                expected: AstType::String,
                got: value.type_(),
                location: self.first_location(),
            }),
        }
    }
}
