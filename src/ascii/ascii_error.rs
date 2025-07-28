use std::fmt::Display;

#[derive(Debug)]
pub enum ASCIIError {
    ReadFileError(String),
}

impl Display for ASCIIError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ASCIIError::ReadFileError(err) => {
                write!(f, "{}", err)
            }
        }
    }
}