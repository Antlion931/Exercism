enum StateCalculator {
    Addition(i32),
    Subtraction(i32),
    Multiplication(i32),
    Division(i32),
    Value(i32),
    NoValue,
}

impl StateCalculator {
    fn new() -> Self {
        Self::NoValue
    }

    fn push(&mut self, b: i32) -> Result<(), StateError> {
        match self {
            Self::NoValue => {
                *self = Self::Value(b);
                Ok(())
            }
            Self::Value(_) => Err(StateError::WrongCommand),
            Self::Addition(a) => {
                let result = a.checked_add(b).ok_or(StateError::WrongResult)?;
                *self = Self::Value(result);
                Ok(())
            }
            Self::Subtraction(a) => {
                let result = a.checked_sub(b).ok_or(StateError::WrongResult)?;
                *self = Self::Value(result);
                Ok(())
            }
            Self::Multiplication(a) => {
                let result = a.checked_mul(b).ok_or(StateError::WrongResult)?;
                *self = Self::Value(result);
                Ok(())
            }
            Self::Division(a) => {
                let result = a.checked_div(b).ok_or(StateError::WrongResult)?;
                *self = Self::Value(result);
                Ok(())
            }
        }
    }

    fn addition(&mut self) -> Result<(), StateError> {
        match self {
            Self::Value(x) => {
                *self = Self::Addition(*x);
                Ok(())
            }
            _ => Err(StateError::WrongCommand),
        }
    }

    fn subtraction(&mut self) -> Result<(), StateError> {
        match self {
            Self::Value(x) => {
                *self = Self::Subtraction(*x);
                Ok(())
            }
            _ => Err(StateError::WrongCommand),
        }
    }

    fn multiplicaton(&mut self) -> Result<(), StateError> {
        match self {
            Self::Value(x) => {
                *self = Self::Multiplication(*x);
                Ok(())
            }
            _ => Err(StateError::WrongCommand),
        }
    }

    fn division(&mut self) -> Result<(), StateError> {
        match self {
            Self::Value(x) => {
                *self = Self::Division(*x);
                Ok(())
            }
            _ => Err(StateError::WrongCommand),
        }
    }

    fn result(&self) -> Result<i32, StateError> {
        match self {
            Self::Value(x) => Ok(*x),
            _ => Err(StateError::WrongCommand),
        }
    }
}

enum StateError {
    WrongResult,
    WrongCommand,
}

pub fn answer(command: &str) -> Option<i32> {
    if let Some(command) = command
        .strip_prefix("What is ")
        .and_then(|c| c.strip_suffix('?'))
    {
        let mut calculator = StateCalculator::new();

        for word in command.split_whitespace() {
            match word {
                "plus" => calculator.addition().ok()?,
                "minus" => calculator.subtraction().ok()?,
                "multiplied" => calculator.multiplicaton().ok()?,
                "divided" => calculator.division().ok()?,
                "by" => match calculator {
                    StateCalculator::Multiplication(_) | StateCalculator::Division(_) => {}
                    _ => return None,
                },
                x => {
                    if let Ok(number) = x.parse() {
                        calculator.push(number).ok()?;
                    } else {
                        return None;
                    }
                }
            }
        }

        calculator.result().ok()
    } else {
        None
    }
}
