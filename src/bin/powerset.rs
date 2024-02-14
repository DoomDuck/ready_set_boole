use std::io::{stdin, stdout, Write};

use ready_set_boole::set::Set;

pub fn main() {
    let mut line = String::new();
    loop {
        print!(">> ");
        stdout().flush().unwrap();
        line.clear();
        if stdin().read_line(&mut line).unwrap() == 0 {
            break;
        }

        let numbers: Vec<i32> = match line
            .split_whitespace()
            .map(|part| part.parse())
            .collect()
        {
            Ok(numbers) => numbers,
            Err(err) => {
                eprintln!("Could not parse numbers: {err}");
                continue;
            }
        };

        let Ok(set) = Set::try_from(numbers) else {
            eprintln!("Invalid set: duplicate found");
            continue;
        };

        for set in set.powerset() {
            println!("{set}");
        }
    }
}
