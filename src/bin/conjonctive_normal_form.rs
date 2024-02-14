use std::io::{stdin, stdout, Write};
use ready_set_boole::conjonctive_normal_form;

pub fn main() {
    let mut formula = String::new();
    loop {
        print!(">> ");
        stdout().flush().unwrap();
        formula.clear();
        if stdin().read_line(&mut formula).unwrap() == 0 {
            break;
        }

        println!("{}", conjonctive_normal_form(&formula.trim().to_uppercase()));
    }
}
