use std::io::{stdin, stdout, Write};

use ready_set_boole::eval_formula;

pub fn main() {
    let mut formula = String::new();
    loop {
        print!(">> ");
        stdout().flush().unwrap();
        formula.clear();
        if stdin().read_line(&mut formula).unwrap() == 0 {
            break;
        }

        println!("=> {}", eval_formula(formula.trim()));
    }
}