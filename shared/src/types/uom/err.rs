use std::fmt;

#[derive(Debug)]
pub enum UcreError {
    String(String),
    Str(&'static str),
}

impl UcreError {
    pub fn new(s: String) -> Self {
        UcreError::String(s)
    }

    pub const fn from_str(s: &'static str) -> Self {
        UcreError::Str(s)
    }
}

impl fmt::Display for UcreError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::String(string) => write!(f, "{}", string),
            Self::Str(str) => write!(f, "{}", str),
        }
    }
}
