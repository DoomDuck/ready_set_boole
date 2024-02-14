use ready_set_boole::sat;
use std::io::{stdin, stdout, Write};

pub fn main() {
    let mut formula = String::new();
    loop {
        print!(">> ");
        stdout().flush().unwrap();
        formula.clear();
        if stdin().read_line(&mut formula).unwrap() == 0 {
            break;
        }

        println!("=> {}", sat(&formula.trim().to_uppercase()));
    }
}

