pub mod expression;
pub mod arithmetic;
pub mod evaluate;
pub mod set;
pub mod curve;

use expression::Expression;

// pub fn print_truth_table(formula: &str) {
//     let expression: Expression = match formula.parse() {
//         Ok(expression) => expression,
//         Err(err) => {
//             eprintln!("Could not parse formula: {err:?}");
//             return;
//         }
//     };
//     let mut environment = expression.envs();
//     print!("|");
//     let mut count = 1;
//     for symbol in environment.symbols() {
//         count += 1;
//         print!(" {} |", symbol as char);
//     }
//     print!(" = |\n|");
//     for _ in 0..count {
//         print!("---|");
//     }
//     println!("");
//     loop {
//         print!("|");
//         for value in environment.values() {
//             print!(" {} |", value as u8);
//         }
//         println!(" {} |", expression.eval(environment) as u8);

//         let Some(env) = environment.next() else {
//             break;
//         };
//         environment = env;
//     }
// }

pub fn negation_normal_form(formula: &str) -> String {
    let expr: Expression = formula.parse().unwrap();
    expr.negation_normal().to_string()
}

#[cfg(test)]
mod tests {
    use crate::evaluate;

    #[test]
    fn eval_formula() {
        assert_eq!(evaluate::eval_formula("10&"), false);
        assert_eq!(evaluate::eval_formula("10|"), true);
        assert_eq!(evaluate::eval_formula("10|1&"), true);
        assert_eq!(evaluate::eval_formula("101|&"), true);
        assert_eq!(evaluate::eval_formula("11>"), true);
        assert_eq!(evaluate::eval_formula("10="), false);
        assert_eq!(evaluate::eval_formula("1011||="), true);
    }
}
