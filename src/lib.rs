use deoption_derive::Deoption;

//pub mod tuples;

pub trait Deoption {
    fn deoption<T>(self) -> Result<T, DeoptionError>;
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

macro_rules! deoption {
    ($($id:ident),*) => {
        {
            let mut missing: Vec<String> = Vec::new();

            $(
                if $id.is_none() {
                    missing.push(stringify!($id).to_owned());
                }
            )*

            if missing.is_empty() {
                Ok((
                    $(
                        $id.unwrap(),
                    )*
                ))
            } else {
                Err(missing)
            }

        }
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_macro() {
        let a = Some(1);
        let b = Some("2");
        let c: Option<f64> = None;

        match deoption!(a, b, c) {
            Ok((d, e, f)) => println!("{} {} {}", d, e, f),
            Err(missing) => println!("{:?}", missing),
        }
    }

    #[derive(Deoption)]
    struct TestStruct {
        name: String,
        age: std::option::Option<i32>,
        weight: Option<f64>,
    }

    #[test]
    fn test_derive() {
        let test = TestStruct {
            name: "Bill".to_owned(),
            age: None,
            weight: None,
        };

        test.deoption::<String>();
    }

}
