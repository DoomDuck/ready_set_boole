pub mod arithmetic;
pub mod curve;
pub mod evaluate;
pub mod expression;
pub mod set;

use std::io::stdout;

use expression::Expression;

pub use arithmetic::{adder, gray_code, multiplier};

pub fn eval_formula(formula: &str) -> bool {
    match evaluate::try_evaluate(formula) {
        Ok(value) => value,
        Err(err) => {
            eprintln!("An error occured while evaluting: {err:?}");
            false
        }
    }
}

pub fn print_truth_table(formula: &str) {
    let expression = match formula.parse::<Expression>() {
        Ok(expression) => expression,
        Err(err) => {
            eprintln!("Could not parse formula: {err:?}");
            return;
        }
    };
    if let Err(err) = expression.write_truth_table(&mut stdout()) {
        eprintln!("Could not parse formula: {err:?}");
    }
}

pub fn negation_normal_form(formula: &str) -> String {
    match formula.parse::<Expression>() {
        Ok(expression) => expression.negation_normal().to_string(),
        Err(err) => {
            eprintln!("Could not parse formula: {err:?}");
            return "".to_owned();
        }
    }
}

pub fn conjonctive_normal_form(formula: &str) -> String {
    match formula.parse::<Expression>() {
        Ok(expression) => expression.conjonctive_normal().to_string(),
        Err(err) => {
            eprintln!("Could not parse formula: {err:?}");
            return "".to_owned();
        }
    }
}

pub fn sat(formula: &str) -> bool {
    match formula.parse::<Expression>() {
        Ok(expression) => expression.sat(),
        Err(err) => {
            eprintln!("Could not parse formula: {err:?}");
            return false;
        }
    }
}

pub fn powerset(set: Vec<i32>) -> Vec<Vec<i32>> {
    match set::Set::try_from(set) {
        Ok(set) => set.powerset().map(|s| s.cloned().into()).collect(),
        Err(_) => {
            eprintln!("Invalid set: has duplicate");
            return Vec::new();
        }
    }
}

pub fn eval_set(formula: &str, sets: Vec<Vec<i32>>) -> Vec<i32> {
    let Ok(environment) = sets.into_iter().map(set::Set::try_from).collect() else {
        eprintln!("Invalid set found: has duplicate");
        return Vec::new();
    };
    match set::try_evaluate(formula, environment) {
        Ok(set) => set.into(),
        Err(err) => {
            eprintln!("Could not evaluate set expression: {err:?}");
            Vec::new()
        }
    }
}

pub use curve::{map, reverse_map};

#[cfg(test)]
mod correction_tests {
    #[test]
    fn adder() {
        assert_eq!(super::adder(0, 0), 0);
        assert_eq!(super::adder(1, 0), 1);
        assert_eq!(super::adder(0, 1), 1);
        assert_eq!(super::adder(1, 1), 2);
        assert_eq!(super::adder(1, 2), 3);
        assert_eq!(super::adder(2, 2), 4);
    }

    #[test]
    fn multiplier() {
        assert_eq!(super::multiplier(0, 0), 0);
        assert_eq!(super::multiplier(1, 0), 0);
        assert_eq!(super::multiplier(0, 1), 0);
        assert_eq!(super::multiplier(1, 1), 1);
        assert_eq!(super::multiplier(1, 2), 2);
        assert_eq!(super::multiplier(2, 2), 4);
    }

    #[test]
    fn eval_formula_basic() {
        assert_eq!(super::eval_formula("0!"), true);
        assert_eq!(super::eval_formula("1!"), false);
        assert_eq!(super::eval_formula("00|"), false);
        assert_eq!(super::eval_formula("10|"), true);
        assert_eq!(super::eval_formula("01|"), true);
        assert_eq!(super::eval_formula("11|"), true);
        assert_eq!(super::eval_formula("10&"), false);
        assert_eq!(super::eval_formula("11&"), true);
        assert_eq!(super::eval_formula("11^"), false);
        assert_eq!(super::eval_formula("10^"), true);
        assert_eq!(super::eval_formula("00>"), true);
        assert_eq!(super::eval_formula("01>"), true);
        assert_eq!(super::eval_formula("10>"), false);
        assert_eq!(super::eval_formula("00="), true);
        assert_eq!(super::eval_formula("11="), true);
        assert_eq!(super::eval_formula("10="), false);
        assert_eq!(super::eval_formula("01="), false);
    }

    #[test]
    fn eval_formula_compistion() {
        assert_eq!(super::eval_formula("11&0|"), true);
        assert_eq!(super::eval_formula("10&1|"), true);
        assert_eq!(super::eval_formula("11&1|"), true);
        assert_eq!(super::eval_formula("11&1|1^"), false);
        assert_eq!(super::eval_formula("01&1|1="), true);
        assert_eq!(super::eval_formula("01&1&1&"), false);
        assert_eq!(super::eval_formula("0111&&&"), false);
    }

    #[test]
    fn print_truth_table() {
        fn check(formula: &str, expected_output: &str) {
            let expression: crate::expression::Expression = formula.parse().unwrap();
            let mut output = Vec::new();
            expression.write_truth_table(&mut output).unwrap();
            let text = std::str::from_utf8(&output).unwrap();
            assert_eq!(text, expected_output, "Error in `{formula}`'s truth table");
        }

        check(
            "A",
            "\
            | A | = |\n\
            |---|---|\n\
            | 0 | 0 |\n\
            | 1 | 1 |\n\
            ",
        );

        check(
            "A!",
            "\
            | A | = |\n\
            |---|---|\n\
            | 0 | 1 |\n\
            | 1 | 0 |\n\
            ",
        );

        check(
            "AB|",
            "\
            | A | B | = |\n\
            |---|---|---|\n\
            | 0 | 0 | 0 |\n\
            | 0 | 1 | 1 |\n\
            | 1 | 0 | 1 |\n\
            | 1 | 1 | 1 |\n\
            ",
        );

        check(
            "AB^",
            "\
            | A | B | = |\n\
            |---|---|---|\n\
            | 0 | 0 | 0 |\n\
            | 0 | 1 | 1 |\n\
            | 1 | 0 | 1 |\n\
            | 1 | 1 | 0 |\n\
            ",
        );

        check(
            "AB>",
            "\
            | A | B | = |\n\
            |---|---|---|\n\
            | 0 | 0 | 1 |\n\
            | 0 | 1 | 1 |\n\
            | 1 | 0 | 0 |\n\
            | 1 | 1 | 1 |\n\
            ",
        );

        check(
            "AB=",
            "\
            | A | B | = |\n\
            |---|---|---|\n\
            | 0 | 0 | 1 |\n\
            | 0 | 1 | 0 |\n\
            | 1 | 0 | 0 |\n\
            | 1 | 1 | 1 |\n\
            ",
        );

        check(
            "AA=",
            "\
            | A | = |\n\
            |---|---|\n\
            | 0 | 1 |\n\
            | 1 | 1 |\n\
            ",
        );

        check(
            "ABC==",
            "\
            | A | B | C | = |\n\
            |---|---|---|---|\n\
            | 0 | 0 | 0 | 0 |\n\
            | 0 | 0 | 1 | 1 |\n\
            | 0 | 1 | 0 | 1 |\n\
            | 0 | 1 | 1 | 0 |\n\
            | 1 | 0 | 0 | 1 |\n\
            | 1 | 0 | 1 | 0 |\n\
            | 1 | 1 | 0 | 0 |\n\
            | 1 | 1 | 1 | 1 |\n\
            ",
        );

        check(
            "AB>C>",
            "\
            | A | B | C | = |\n\
            |---|---|---|---|\n\
            | 0 | 0 | 0 | 0 |\n\
            | 0 | 0 | 1 | 1 |\n\
            | 0 | 1 | 0 | 0 |\n\
            | 0 | 1 | 1 | 1 |\n\
            | 1 | 0 | 0 | 1 |\n\
            | 1 | 0 | 1 | 1 |\n\
            | 1 | 1 | 0 | 0 |\n\
            | 1 | 1 | 1 | 1 |\n\
            ",
        );

        check(
            "AB>A>A>",
            "\
            | A | B | = |\n\
            |---|---|---|\n\
            | 0 | 0 | 1 |\n\
            | 0 | 1 | 1 |\n\
            | 1 | 0 | 1 |\n\
            | 1 | 1 | 1 |\n\
            ",
        );
    }

    #[test]
    fn negation_normal_form_basic() {
        assert_eq!(super::negation_normal_form("A"), "A");
        assert_eq!(super::negation_normal_form("A!"), "A!");
        assert_eq!(super::negation_normal_form("AB&!"), "A!B!|");
        assert_eq!(super::negation_normal_form("AB|!"), "A!B!&");
        assert_eq!(super::negation_normal_form("AB>!"), "AB!&");
        assert_eq!(super::negation_normal_form("AB=!"), "AB!&A!B&|");
    }

    #[test]
    fn negation_normal_form_composition() {
        assert_eq!(super::negation_normal_form("ABC||"), "ABC||");
        assert_eq!(super::negation_normal_form("ABC||!"), "A!B!C!&&");
        assert_eq!(super::negation_normal_form("ABC|&"), "ABC|&");
        assert_eq!(super::negation_normal_form("ABC&|"), "ABC&|");
        assert_eq!(super::negation_normal_form("ABC&|!"), "A!B!C!|&");
        assert_eq!(
            super::negation_normal_form("ABC^^"),
            "ABC&B!C!&|&A!BC!&B!C&|&|"
        );
        assert_eq!(super::negation_normal_form("ABC>>"), "A!B!C||");
    }

    #[test]
    fn conjunctive_normal_form_basic() {
        assert_eq!(super::conjonctive_normal_form("A"), "A");
        assert_eq!(super::conjonctive_normal_form("A!"), "A!");
        assert_eq!(super::conjonctive_normal_form("AB&!"), "A!B!|");
        assert_eq!(super::conjonctive_normal_form("AB|!"), "A!B!&");
        assert_eq!(super::conjonctive_normal_form("AB>!"), "AB!&");
        assert_eq!(super::conjonctive_normal_form("AB=!"), "A!A!|AB|&");
    }

    #[test]
    fn conjunctive_normal_form_composition() {
        assert_eq!(super::conjonctive_normal_form("ABC||"), "ABC||");
        assert_eq!(super::conjonctive_normal_form("ABC||!"), "A!B!C!&&");
        assert_eq!(super::conjonctive_normal_form("ABC|&"), "ABC|&");
        assert_eq!(super::conjonctive_normal_form("ABC&|"), "AB|AC|&");
        assert_eq!(super::conjonctive_normal_form("ABC&|!"), "A!B!C!|&");
        assert_eq!(
            super::conjonctive_normal_form("ABC^^"),
            "ABC||AB!C!||A!B!C||A!BC!||&&&"
        );
        assert_eq!(super::conjonctive_normal_form("ABC>>"), "A!B!C||");
    }

    #[test]
    fn sat_basic() {
        assert_eq!(super::sat("A"), true);
        assert_eq!(super::sat("A!"), true);
        assert_eq!(super::sat("AA|"), true);
        assert_eq!(super::sat("AA&"), true);
        assert_eq!(super::sat("AA!&"), false);
        assert_eq!(super::sat("AA^"), false);
        assert_eq!(super::sat("AB^"), true);
        assert_eq!(super::sat("AB="), true);
        assert_eq!(super::sat("AA>"), true);
        assert_eq!(super::sat("AA!>"), true);
    }

    #[test]
    fn sat_composition() {
        assert_eq!(super::sat("ABC||"), true);
        assert_eq!(super::sat("AB&A!B!&&'"), false);
        assert_eq!(super::sat("ABCDE&&&&"), true);
        assert_eq!(super::sat("AAA^^"), true);
        assert_eq!(super::sat("ABCDE^^^^"), true);
    }

    #[test]
    fn powerset() {
        fn check(input: &[i32], expected: &[&[i32]]) {
            for (value, &expected) in super::powerset(input.to_owned()).into_iter().zip(expected) {
                assert_eq!(value, expected);
            }
        }

        check(&[], &[&[]]);
        check(&[1], &[&[], &[1]]);
        check(&[1, 2], &[&[], &[1], &[2], &[1, 2]]);
        check(
            &[1, 2, 3],
            &[&[], &[1], &[2], &[1, 2], &[3], &[1, 3], &[2, 3], &[1, 2, 3]],
        );
    }

    #[test]
    fn eval_set_basic() {
        fn check(formula: &str, sets: &[&[i32]], expected: &[i32]) {
            let sets = sets.into_iter().map(|&s| s.into()).collect();
            assert_eq!(super::eval_set(formula, sets), expected);
        }

        check("A", &[&[]], &[]);
        check("A!", &[&[]], &[]);
        check("A", &[&[42]], &[42]);
        check("A!", &[&[42]], &[]);
        check("A!B&", &[&[1, 2, 3], &[2, 3, 4]], &[4]);
        check("AB|", &[&[0, 1, 2], &[]], &[0, 1, 2]);
        check("AB&", &[&[0, 1, 2], &[]], &[]);
        check("AB&", &[&[0, 1, 2], &[0]], &[0]);
        check("AB&", &[&[0, 1, 2], &[42]], &[]);
        check("AB^", &[&[0, 1, 2], &[0]], &[1, 2]);
        check("AB>", &[&[0], &[1, 2]], &[1, 2]);
        check("AB>", &[&[0], &[0, 1, 2]], &[0, 1, 2]);
    }

    #[test]
    fn eval_set_composition() {
        fn check(formula: &str, sets: &[&[i32]], expected: &[i32]) {
            let sets = sets.into_iter().map(|&s| s.into()).collect();
            assert_eq!(super::eval_set(formula, sets), expected);
        }

        check("ABC||", &[&[], &[], &[]], &[]);
        check("ABC||", &[&[0], &[1], &[2]], &[0, 1, 2]);
        check("ABC||", &[&[0], &[0], &[0]], &[0]);
        check("ABC&&", &[&[0], &[0], &[]], &[]);
        check("ABC&&", &[&[0], &[0], &[0]], &[0]);
        check("ABC^^", &[&[0], &[0], &[0]], &[0]);
        check("ABC>>", &[&[0], &[0], &[0]], &[0]);
    }
}
