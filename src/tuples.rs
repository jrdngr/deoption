use crate::{Deoption, DeoptionError};

impl<T0, T1> Deoption<(T0, T1)> for (Option<T0>, Option<T1>) {
    fn deoption(self) -> Result<(T0, T1), DeoptionError> {
        let mut missing = Vec::new();

        if self.0.is_none() {
            missing.push("0".to_owned());
        }

        if self.1.is_none() {
            missing.push("1".to_owned());
        }

        if missing.is_empty() {
            Ok((self.0.unwrap(), self.1.unwrap()))
        } else {
            Err(DeoptionError::new(missing))
        }
    }
}

impl<T0, T1, T2> Deoption<(T0, T1, T2)> for (Option<T0>, Option<T1>, Option<T2>) {
    fn deoption(self) -> Result<(T0, T1, T2), DeoptionError> {
        let mut missing = Vec::new();

        if self.0.is_none() {
            missing.push("0".to_owned());
        }

        if self.1.is_none() {
            missing.push("1".to_owned());
        }

        if self.2.is_none() {
            missing.push("2".to_owned());
        }

        if missing.is_empty() {
            Ok((self.0.unwrap(), self.1.unwrap(), self.2.unwrap()))
        } else {
            Err(DeoptionError::new(missing))
        }
    }
}

impl<T0, T1, T2, T3> Deoption<(T0, T1, T2, T3)>
    for (Option<T0>, Option<T1>, Option<T2>, Option<T3>)
{
    fn deoption(self) -> Result<(T0, T1, T2, T3), DeoptionError> {
        let mut missing = Vec::new();

        if self.0.is_none() {
            missing.push("0".to_owned());
        }

        if self.1.is_none() {
            missing.push("1".to_owned());
        }

        if self.2.is_none() {
            missing.push("2".to_owned());
        }

        if self.3.is_none() {
            missing.push("3".to_owned());
        }

        if missing.is_empty() {
            Ok((
                self.0.unwrap(),
                self.1.unwrap(),
                self.2.unwrap(),
                self.3.unwrap(),
            ))
        } else {
            Err(DeoptionError::new(missing))
        }
    }
}

impl<T0, T1, T2, T3, T4> Deoption<(T0, T1, T2, T3, T4)>
    for (Option<T0>, Option<T1>, Option<T2>, Option<T3>, Option<T4>)
{
    fn deoption(self) -> Result<(T0, T1, T2, T3, T4), DeoptionError> {
        let mut missing = Vec::new();

        if self.0.is_none() {
            missing.push("0".to_owned());
        }

        if self.1.is_none() {
            missing.push("1".to_owned());
        }

        if self.2.is_none() {
            missing.push("2".to_owned());
        }

        if self.3.is_none() {
            missing.push("3".to_owned());
        }

        if self.4.is_none() {
            missing.push("4".to_owned());
        }

        if missing.is_empty() {
            Ok((
                self.0.unwrap(),
                self.1.unwrap(),
                self.2.unwrap(),
                self.3.unwrap(),
                self.4.unwrap(),
            ))
        } else {
            Err(DeoptionError::new(missing))
        }
    }
}

impl<T0, T1, T2, T3, T4, T5> Deoption<(T0, T1, T2, T3, T4, T5)>
    for (
        Option<T0>,
        Option<T1>,
        Option<T2>,
        Option<T3>,
        Option<T4>,
        Option<T5>,
    )
{
    fn deoption(self) -> Result<(T0, T1, T2, T3, T4, T5), DeoptionError> {
        let mut missing = Vec::new();

        if self.0.is_none() {
            missing.push("0".to_owned());
        }

        if self.1.is_none() {
            missing.push("1".to_owned());
        }

        if self.2.is_none() {
            missing.push("2".to_owned());
        }

        if self.3.is_none() {
            missing.push("3".to_owned());
        }

        if self.4.is_none() {
            missing.push("4".to_owned());
        }

        if self.5.is_none() {
            missing.push("5".to_owned());
        }

        if missing.is_empty() {
            Ok((
                self.0.unwrap(),
                self.1.unwrap(),
                self.2.unwrap(),
                self.3.unwrap(),
                self.4.unwrap(),
                self.5.unwrap(),
            ))
        } else {
            Err(DeoptionError::new(missing))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_2_ok() {
        (Some("thing"), Some(2)).deoption().unwrap();
    }

    #[test]
    #[should_panic]
    fn test_2_err() {
        let thing: (Option<f64>, Option<String>) = (Some(0.0), None);
        thing.deoption().unwrap();
    }
}
