#[derive(Debug)]
pub enum CalculatorInput {
    Add,
    Subtract,
    Multiply,
    Divide,
    Value(i32),
}

pub fn evaluate(inputs: &[CalculatorInput]) -> Option<i32> {
    use CalculatorInput::*;

    let mut stack: Vec<i32> = vec![];

    for entry in inputs {
        match entry {
            Value(num) => stack.push(*num),
            _ => {
                let operation = match entry {
                    Add => |x, y| x + y,
                    Subtract => |x, y| x - y,
                    Multiply => |x, y| x * y,
                    Divide => |x, y| x / y,
                    _ => panic!("This can't happen"),
                };

                let b = stack.pop();
                let a = stack.pop();
                match a.zip(b) {
                    Some((first, second)) => stack.push(operation(first, second)),
                    None => (break),
                }
            }
        }
    }

    match stack.len() {
        1 => Some(stack[0]),
        _ => None,
    }
}
