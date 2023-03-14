#[derive(Debug)]
pub enum CalculatorInput {
    Add,
    Subtract,
    Multiply,
    Divide,
    Value(i32),
}

pub fn evaluate(inputs: &[CalculatorInput]) -> Option<i32> {
    // unimplemented!(
    //     "Given the inputs: {inputs:?}, evaluate them as though they were a Reverse Polish notation expression"
    // );
    let mut stack = Vec::<i32>::new();
    for element in inputs {
        if let CalculatorInput::Value(num) = element {
            stack.push(*num);
        } else {
            match element {
                CalculatorInput::Add => {
                    let rhs_operands = stack.pop()?;
                    let lhs_operands = stack.pop()?;
                    stack.push(lhs_operands + rhs_operands);
                },
                CalculatorInput::Subtract => {
                    let rhs_operands = stack.pop()?;
                    let lhs_operands = stack.pop()?;
                    stack.push(lhs_operands - rhs_operands);
                },
                CalculatorInput::Multiply => {
                    let rhs_operands = stack.pop()?;
                    let lhs_operands = stack.pop()?;
                    stack.push(lhs_operands * rhs_operands);
                },
                CalculatorInput::Divide => {
                    let rhs_operands = stack.pop()?;
                    let lhs_operands = stack.pop()?;
                    stack.push(lhs_operands / rhs_operands);
                },
                _ => ()
            }
        }
    }
    if stack.is_empty() || stack.len() == 2 {
        None
    } else{
        stack.pop()
    }
}

