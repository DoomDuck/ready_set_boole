use std::io::{stdin, stdout, Write};

use ready_set_boole::print_truth_table;

pub fn main() {
    let mut formula = String::new();
    loop {
        print!(">> ");
        stdout().flush().unwrap();
        formula.clear();
        if stdin().read_line(&mut formula).unwrap() == 0 {
            break;
        }

        print_truth_table(&formula.trim().to_uppercase());
    }
}
