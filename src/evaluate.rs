#[derive(Debug)]
pub enum EvaluationError {
    MissingArgument,
    IncompleteComputation,
    UnknownSymbol,
}

pub fn try_evaluate(expression: &str) -> Result<bool, EvaluationError> {
    use EvaluationError::*;
    let mut stack = Vec::<bool>::new();
    for symbol in expression.bytes() {
        let result = match symbol {
            b'0' => false,
            b'1' => true,
            b'!' => !(stack.pop().ok_or(MissingArgument)?),
            b'|' => {
                let b = stack.pop().ok_or(MissingArgument)?;
                let a = stack.pop().ok_or(MissingArgument)?;
                a || b
            }
            b'&' => {
                let b = stack.pop().ok_or(MissingArgument)?;
                let a = stack.pop().ok_or(MissingArgument)?;
                a && b
            }
            b'^' => {
                let b = stack.pop().ok_or(MissingArgument)?;
                let a = stack.pop().ok_or(MissingArgument)?;
                a ^ b
            }
            b'>' => {
                let b = stack.pop().ok_or(MissingArgument)?;
                let a = stack.pop().ok_or(MissingArgument)?;
                a <= b
            }
            b'=' => {
                let b = stack.pop().ok_or(MissingArgument)?;
                let a = stack.pop().ok_or(MissingArgument)?;
                a == b
            }
            _ => return Err(UnknownSymbol),
        };
        stack.push(result)
    }
    if stack.len() != 1 {
        return Err(IncompleteComputation);
    }
    Ok(stack.pop().unwrap())
}


fn evaluate(expression: &str) -> bool {
    match try_evaluate(expression) {
        Ok(value) => value,
        Err(err) => {
            eprintln!("An error occured while evaluting: {err:?}");
            false
        }
    }
}

pub fn eval_formula(formula: &str) -> bool {
    evaluate(formula)
}

