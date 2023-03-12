mod tests;

#[derive(Debug, Clone, Copy)]
pub enum CalculatorInput {
    Add,
    Subtract,
    Multiply,
    Divide,
    Value(i32),
}

impl CalculatorInput {
    pub fn is_value(&self) -> bool {
        matches!(self, CalculatorInput::Value(_))
    }
}

pub fn evaluate(inputs: &[CalculatorInput]) -> Option<i32> {
    let mut stack = Vec::new();

    for input in inputs {
        if let CalculatorInput::Value(value) = input {
            stack.push(*value)
        } else {
            let a = stack.pop()?;
            let b = stack.pop()?;

            calculate(input, a, b).map(|result| stack.push(result))?;
        }
    }

    (stack.len() == 1).then(|| stack[0])
}

fn calculate(input: &CalculatorInput, a: i32, b: i32) -> Option<i32> {
    match input {
        CalculatorInput::Add => Some(a + b),
        CalculatorInput::Subtract => Some(b - a),
        CalculatorInput::Multiply => Some(a * b),
        CalculatorInput::Divide => Some(b / a),
        _ => None,
    }
}
