use std::io::{stdin, stdout, Write};

pub fn main() {
    let mut line = String::new();
    loop {
        print!("x = ");
        stdout().flush().unwrap();
        line.clear();
        if stdin().read_line(&mut line).unwrap() == 0 {
            break;
        }

        let x = match line.trim().parse() {
            Ok(x) => x,
            Err(err) => {
                eprintln!("Invalid value: {err}");
                continue;
            },
        };

        print!("y = ");
        stdout().flush().unwrap();
        line.clear();
        if stdin().read_line(&mut line).unwrap() == 0 {
            break;
        }

        let y = match line.trim().parse() {
            Ok(y) => y,
            Err(err) => {
                eprintln!("Invalid value: {err}");
                continue;
            },
        };

        let result = ready_set_boole::map(x, y);
        println!("=> {result}");
    }
}

