use std::io::{stdin, stdout, Write};

use ready_set_boole::set::{Set, self};

pub fn main() {
    let mut line = String::new();

    'outer: loop {
        print!(">> ");
        stdout().flush().unwrap();
        
        line.clear();
        if stdin().read_line(&mut line).unwrap() == 0 {
            break;
        }
        let formula = line.trim().to_uppercase();

        let mut environment = Vec::new();
        loop {
            print!(">    ");
            stdout().flush().unwrap();

            line.clear();
            if stdin().read_line(&mut line).unwrap() == 0 {
                break 'outer;
            }

            if line == "\n" {
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
                    continue 'outer;
                }
            };

            let Ok(set) = Set::try_from(numbers) else {
                eprintln!("Invalid set: duplicate found");
                continue 'outer;
            };

            environment.push(set)
        }

        match set::try_evaluate(&formula, environment) {
            Ok(set) => println!("=> {set}"),
            Err(err) => eprintln!("Evaluation error: {err:?}"),
        }
    }
}
