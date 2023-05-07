use std::collections::HashMap;

pub type Value = i32;
pub type Result = std::result::Result<(), Error>;

pub struct Forth {
    stack: Vec<Value>,
    defined: HashMap<String, Vec<String>>,
}

#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    DivisionByZero,
    StackUnderflow,
    UnknownWord,
    InvalidWord,
}

impl Forth {
    pub fn new() -> Forth {
        Self {
            stack: Vec::new(),
            defined: HashMap::new(),
        }
    }

    pub fn stack(&self) -> &[Value] {
        &self.stack
    }

    pub fn eval(&mut self, input: &str) -> Result {
        let mut words = input.split_whitespace().map(|x| x.to_lowercase());
        'outer: while let Some(word) = words.next() {
            match word.as_str() {
                com if self
                    .defined
                    .contains_key(&com.chars().filter(|c| !c.is_numeric()).collect::<String>()) =>
                {
                    let vec =
                        &self.defined[&com.chars().filter(|c| !c.is_numeric()).collect::<String>()];
                    let number = com
                        .chars()
                        .filter(|c| c.is_numeric())
                        .collect::<String>()
                        .parse()
                        .unwrap_or(vec.len() - 1); // last if not specify
                    self.eval(&vec[number].clone())?;
                }
                "drop" => {
                    self.stack.pop().ok_or(Error::StackUnderflow)?;
                }
                "dup" => {
                    let last = self.stack.pop().ok_or(Error::StackUnderflow)?;
                    self.stack.push(last);
                    self.stack.push(last);
                }
                "swap" => {
                    let x = self.stack.pop().ok_or(Error::StackUnderflow)?;
                    let y = self.stack.pop().ok_or(Error::StackUnderflow)?;
                    self.stack.push(x);
                    self.stack.push(y);
                }
                "over" => {
                    if self.stack.len() < 2 {
                        return Err(Error::StackUnderflow);
                    }
                    let second = self.stack[self.stack.len() - 2];
                    self.stack.push(second);
                }
                "+" => {
                    let x = self.stack.pop().ok_or(Error::StackUnderflow)?;
                    let y = self.stack.pop().ok_or(Error::StackUnderflow)?;
                    self.stack.push(y + x);
                }
                "-" => {
                    let x = self.stack.pop().ok_or(Error::StackUnderflow)?;
                    let y = self.stack.pop().ok_or(Error::StackUnderflow)?;
                    self.stack.push(y - x);
                }

                "*" => {
                    let x = self.stack.pop().ok_or(Error::StackUnderflow)?;
                    let y = self.stack.pop().ok_or(Error::StackUnderflow)?;
                    self.stack.push(y * x);
                }
                "/" => {
                    let x = self.stack.pop().ok_or(Error::StackUnderflow)?;
                    if x == 0 {
                        return Err(Error::DivisionByZero);
                    }
                    let y = self.stack.pop().ok_or(Error::StackUnderflow)?;
                    self.stack.push(y / x);
                }
                ":" => {
                    let key = words
                        .next()
                        .filter(|x| x.chars().all(|c| !c.is_numeric()))
                        .ok_or(Error::InvalidWord)?;
                    let mut value = String::new();

                    while let Some(new) = words.next() {
                        if new == ";" {
                            self.defined.entry(key).or_insert(Vec::new()).push(value);
                            continue 'outer;
                        } else if self.defined.contains_key(&new) {
                            value += &new;
                            value += &(self.defined[&new].len() - 1).to_string();
                        } else {
                            value += &new;
                        }
                        value += " ";
                    }
                    return Err(Error::InvalidWord);
                }
                x if x.chars().all(|c| c.is_numeric()) => {
                    self.stack
                        .push(x.parse().expect("contains only numeric chars"));
                }
                _ => return Err(Error::UnknownWord),
            }
        }
        Ok(())
    }
}
