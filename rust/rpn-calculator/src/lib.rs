#[derive(Debug)]
pub enum CalculatorInput {
    Add,
    Subtract,
    Multiply,
    Divide,
    Value(i32),
}

pub fn evaluate(inputs: &[CalculatorInput]) -> Option<i32> {
    let mut stack = vec![];
    for input in inputs {
        match input {
            CalculatorInput::Value(value) => stack.push(*value),
            operation => {
                if stack.len() < 2 {
                    return  None;
                }

                let x = stack.pop().unwrap();
                let y = stack.pop().unwrap();

                match operation {
                    CalculatorInput::Add => stack.push( y + x ),
                    CalculatorInput::Divide => stack.push( y / x ),
                    CalculatorInput::Multiply => stack.push( y * x ),
                    CalculatorInput::Subtract => stack.push( y - x ),
                    _ => { return None; } // should never happen
                }
            }
        }
    }

    if stack.len() == 1 { stack.pop() } 
    else { None }

}
