use crate::{Ast, Location, Node};

#[derive(Debug)]
pub struct List {
    nodes: Vec<Node>,
    ignore_comments: bool,
    location: Location,
}
impl List {
    pub fn peek_front(&self) -> Option<&Node> {
        if self.nodes.is_empty() {
            None
        } else {
            for (i, node) in self.nodes.iter().enumerate() {
                match node.ast {
                    Ast::Comment(_) => continue,
                    _ => return Some(&self.nodes[i]),
                }
            }

            return None;
        }
    }

    pub fn assert_empty(&self, msg: &str) -> Result<(), String> {
        match self.peek_front() {
            Some(n) => err(
                &format!("Expected no more values for {msg}",),
                &n.first_location(),
            ),
            None => Ok(()),
        }
    }

    pub fn maybe_pop_string(&mut self, msg: &str) -> Result<Option<(String, Location)>, String> {
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

    pub fn pop_front_absolute(&mut self, msg: &str) -> Result<Node, String> {
        match self.nodes.is_empty() {
            true => err(&format!("Expected {msg}"), &self.location),
            false => Ok(self.nodes.remove(0)),
        }
    }

    pub fn pop_front(&mut self, msg: &str) -> Result<Node, String> {
        match self.nodes.is_empty() {
            true => err(&format!("Expected {msg}"), &self.location),
            false => {
                // Skip comments
                let node = self.nodes.remove(0);
                match &node.ast {
                    Ast::Comment(_) => self.pop_front(msg),
                    _ => Ok(node),
                }
            }
        }
    }

    pub fn is_empty(&self) -> bool {
        self.nodes.is_empty()
    }

    pub fn pop_list(&mut self, msg: &str) -> Result<List, String> {
        let node = self.pop_front(&msg)?;
        list(&node, msg, self.ignore_comments)
    }

    pub fn pop_integer(&mut self, msg: &str) -> Result<(i64, Location), String> {
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

    pub fn pop_identifier(&mut self, msg: &str) -> Result<(String, Location), String> {
        let node = self.pop_front(&msg)?;
        match &node.ast {
            Ast::Identifier(s) => Ok((s.clone(), node.first_location())),
            _ => err(&format!("Expected {msg}"), &node.first_location()),
        }
    }

    pub fn pop_string(&mut self, msg: &str) -> Result<(String, Location), String> {
        let node = self.pop_front(&msg)?;
        match &node.ast {
            Ast::String(s) => Ok((s.clone(), node.first_location())),
            _ => err(&format!("Expected {msg}"), &node.first_location()),
        }
    }
}

pub fn list(node: &Node, msg: &str, ignore_comments: bool) -> Result<List, String> {
    let l = match &node.ast {
        Ast::List(l) => l.clone(),
        _ => return err(&format!("Expected {msg}"), &node.first_location()),
    };
    Ok(List {
        ignore_comments,
        nodes: l,
        location: node.first_location(),
    })
}

pub fn err<T>(contents: &str, l: &Location) -> Result<T, String> {
    let loc_err = match &l.path {
        Some(p) => format!("{} {}:{}", p.display(), l.line, l.column + 1),
        None => format!("{}:{}", l.line, l.column + 1),
    };
    Err(format!("{}: {}", loc_err, contents))
}
