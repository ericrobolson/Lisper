use crate::{Ast, Error, Location, Node};

#[derive(Debug, Clone, PartialEq)]
pub struct List {
    nodes: Vec<Node>,
    location: Location,
}

impl std::fmt::Display for List {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let contents = self
            .nodes
            .iter()
            .map(|node| format!("{}", node))
            .collect::<Vec<String>>()
            .join(" ");

        write!(f, "({})", contents)
    }
}

impl List {
    pub fn location(&self) -> Location {
        self.location.clone()
    }

    pub fn peek_front(&self) -> Option<&Node> {
        if self.nodes.is_empty() {
            None
        } else {
            Some(&self.nodes[0])
        }
    }

    pub fn front_is_list(&self) -> bool {
        if let Some(n) = self.peek_front() {
            match n.ast {
                Ast::List(_) => true,
                _ => false,
            }
        } else {
            false
        }
    }

    pub fn pop_front(&mut self, msg: &str) -> Result<Node, Error> {
        match self.nodes.is_empty() {
            true => err(&format!("Expected {msg}"), &self.location),
            false => Ok(self.nodes.remove(0)),
        }
    }

    pub fn is_empty(&self) -> bool {
        self.nodes.is_empty()
    }

    pub fn assert_empty(&self, msg: &str) -> Result<(), Error> {
        match self.peek_front() {
            Some(n) => err(
                &format!("Expected no more values for {msg}",),
                &n.first_location(),
            ),
            None => Ok(()),
        }
    }

    pub fn pop_bool(&mut self, msg: &str) -> Result<(bool, Location), Error> {
        let node = self.pop_front(&msg)?;
        match &node.ast {
            Ast::Bool(b) => Ok((*b, node.first_location())),
            _ => err(&format!("Expected {msg}"), &node.first_location()),
        }
    }

    pub fn pop_comment(&mut self, msg: &str) -> Result<String, Error> {
        let node = self.pop_front(&msg)?;
        match &node.ast {
            Ast::Comment(s) => Ok(s.clone()),
            _ => err(&format!("Expected {msg}"), &node.first_location()),
        }
    }

    pub fn pop_identifier(&mut self, msg: &str) -> Result<(String, Location), Error> {
        let node = self.pop_front(&msg)?;
        match &node.ast {
            Ast::Identifier(s) => Ok((s.clone(), node.first_location())),
            _ => err(&format!("Expected {msg}"), &node.first_location()),
        }
    }

    pub fn pop_list(&mut self, msg: &str) -> Result<List, Error> {
        let node = self.pop_front(&msg)?;
        list(&node, msg)
    }

    pub fn pop_float(&mut self, msg: &str) -> Result<(f64, Location), Error> {
        let node = self.pop_front(&msg)?;
        match &node.ast {
            Ast::Number(n) => Ok((*n, node.first_location())),
            _ => err(&format!("Expected {msg}"), &node.first_location()),
        }
    }

    pub fn pop_integer(&mut self, msg: &str) -> Result<(i64, Location), Error> {
        let node = self.pop_front(&msg)?;
        match &node.ast {
            Ast::Number(n) => {
                // If unable to cast to an int, return an error
                if n.fract() != 0.0 {
                    return err(
                        &format!("Expected an int for {msg}"),
                        &node.first_location(),
                    );
                }
                Ok((*n as i64, node.first_location()))
            }
            _ => err(&format!("Expected {msg}"), &node.first_location()),
        }
    }

    pub fn pop_string(&mut self, msg: &str) -> Result<(String, Location), Error> {
        let node = self.pop_front(&msg)?;
        match &node.ast {
            Ast::String(s) => Ok((s.clone(), node.first_location())),
            _ => err(&format!("Expected {msg}"), &node.first_location()),
        }
    }

    pub fn maybe_pop_bool(&mut self, msg: &str) -> Result<Option<(bool, Location)>, Error> {
        let is_bool = if let Some(n) = self.peek_front() {
            match n.ast {
                Ast::Bool(_) => true,
                _ => false,
            }
        } else {
            false
        };
        if is_bool {
            Ok(Some(self.pop_bool(msg)?))
        } else {
            Ok(None)
        }
    }

    pub fn maybe_pop_comment(&mut self, msg: &str) -> Result<Option<String>, Error> {
        let is_comment = if let Some(n) = self.peek_front() {
            match n.ast {
                Ast::Comment(_) => true,
                _ => false,
            }
        } else {
            false
        };
        if is_comment {
            Ok(Some(self.pop_comment(msg)?))
        } else {
            Ok(None)
        }
    }

    pub fn maybe_pop_identifier(&mut self, msg: &str) -> Result<Option<(String, Location)>, Error> {
        let is_identifier = if let Some(n) = self.peek_front() {
            match n.ast {
                Ast::Identifier(_) => true,
                _ => false,
            }
        } else {
            false
        };
        if is_identifier {
            Ok(Some(self.pop_identifier(msg)?))
        } else {
            Ok(None)
        }
    }

    pub fn maybe_pop_list(&mut self, msg: &str) -> Result<Option<List>, Error> {
        let is_list = if let Some(n) = self.peek_front() {
            match n.ast {
                Ast::List(_) => true,
                _ => false,
            }
        } else {
            false
        };
        if is_list {
            Ok(Some(self.pop_list(msg)?))
        } else {
            Ok(None)
        }
    }

    pub fn maybe_pop_float(&mut self, msg: &str) -> Result<Option<(f64, Location)>, Error> {
        let is_float = if let Some(n) = self.peek_front() {
            match n.ast {
                Ast::Number(_) => true,
                _ => false,
            }
        } else {
            false
        };
        if is_float {
            Ok(Some(self.pop_float(msg)?))
        } else {
            Ok(None)
        }
    }

    pub fn maybe_pop_integer(&mut self, msg: &str) -> Result<Option<(i64, Location)>, Error> {
        let is_integer = if let Some(n) = self.peek_front() {
            match n.ast {
                Ast::Number(f) => f.fract() == 0.0,
                _ => false,
            }
        } else {
            false
        };
        if is_integer {
            Ok(Some(self.pop_integer(msg)?))
        } else {
            Ok(None)
        }
    }

    pub fn maybe_pop_string(&mut self, msg: &str) -> Result<Option<(String, Location)>, Error> {
        let is_string = if let Some(n) = self.peek_front() {
            match n.ast {
                Ast::String(_) => true,
                _ => false,
            }
        } else {
            false
        };
        if is_string {
            Ok(Some(self.pop_string(msg)?))
        } else {
            Ok(None)
        }
    }
}

pub fn list(node: &Node, msg: &str) -> Result<List, Error> {
    let l = match &node.ast {
        Ast::List(l) => l.clone(),
        _ => return err(&format!("Expected {msg}"), &node.first_location()),
    };
    Ok(List {
        nodes: l,
        location: node.first_location(),
    })
}

/// Create an error message with a location.
pub fn err<T>(contents: &str, l: &Location) -> Result<T, Error> {
    Err(Error {
        message: contents.to_string(),
        location: l.clone(),
    })
}

#[cfg(test)]
mod tests {
    #[test]
    fn display_list_empty() {
        let contents = "()";
        let list = crate::parse_str(contents).unwrap().first().unwrap().clone();

        assert_eq!(format!("{}", list), "()");
    }

    #[test]
    fn display_list_flat() {
        let contents = "(+ 1 2)";
        let list = crate::parse_str(contents).unwrap().first().unwrap().clone();

        assert_eq!(format!("{}", list), "(+ 1 2)");
    }

    #[test]
    fn display_list_nested() {
        let contents = "(+ 1 (* 2 3))";
        let list = crate::parse_str(contents).unwrap().first().unwrap().clone();

        assert_eq!(format!("{}", list), "(+ 1 (* 2 3))");
    }

    #[test]
    fn display_list_comment() {
        let contents = "; Hello!\n(+ 1 (* 2 3))";
        let list = crate::parse_str(contents).unwrap().first().unwrap().clone();

        assert_eq!(format!("{}", list), "(+ 1 (* 2 3))");
    }
}
