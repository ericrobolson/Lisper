use super::location::Location;

#[derive(Debug, Clone, PartialEq)]
pub struct Error<Kind>
where
    Kind: ToString,
{
    pub kind: Kind,
    pub location: Location,
}

impl<Kind> ToString for Error<Kind>
where
    Kind: ToString,
{
    fn to_string(&self) -> String {
        let loc_error = match self.location.path {
            Some(ref path) => format!(
                "{}:{}:{}",
                path.display(),
                self.location.line,
                self.location.column
            ),
            None => format!("{}:{}", self.location.line, self.location.column),
        };
        format!("{}: {}", loc_error, self.kind.to_string())
    }
}
