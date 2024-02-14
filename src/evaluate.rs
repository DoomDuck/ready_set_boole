#[derive(Debug, PartialEq, Eq)]
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

#[cfg(test)]
mod tests {
    #[test]
    fn try_evaluate() {
        assert_eq!(super::try_evaluate("10&"), Ok(false));
        assert_eq!(super::try_evaluate("10|"), Ok(true));
        assert_eq!(super::try_evaluate("10|1&"), Ok(true));
        assert_eq!(super::try_evaluate("101|&"), Ok(true));
        assert_eq!(super::try_evaluate("11>"), Ok(true));
        assert_eq!(super::try_evaluate("10="), Ok(false));
        assert_eq!(super::try_evaluate("1011||="), Ok(true));
    }
}
