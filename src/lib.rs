pub trait Deoption<T> {
    fn deoption(self) -> Result<T, DeoptionError>;
}

#[derive(Default, Debug, Clone)]
pub struct DeoptionError {
    missing_fields: Vec<String>,
}

impl DeoptionError {
    pub fn new<T>(missing_fields: T) -> Self
    where
        T: Into<Vec<String>>,
    {
        Self {
            missing_fields: missing_fields.into(),
        }
    }
}

impl std::error::Error for DeoptionError {
    fn description(&self) -> &str {
        "All optional fields must be set before calling deoption"
    }

    fn cause(&self) -> Option<&std::error::Error> {
        None
    }
}

impl std::fmt::Display for DeoptionError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:?}", self.missing_fields)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
