pub mod expression;
pub mod arithmetic;
pub mod evaluate;
pub mod set;
pub mod curve;

use std::io::stdout;

use expression::Expression;

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
        },
    }
}

pub use curve::{map, reverse_map};
