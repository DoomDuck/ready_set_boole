use std::io::{stdin, stdout, Write};

pub fn main() {
    let mut line = String::new();
    loop {
        print!("n = ");
        stdout().flush().unwrap();
        line.clear();
        if stdin().read_line(&mut line).unwrap() == 0 {
            break;
        }

        let n = match line.trim().parse() {
            Ok(n) => n,
            Err(err) => {
                eprintln!("Invalid value: {err}");
                continue;
            },
        };

        let (x, y) = ready_set_boole::reverse_map(n);
        println!("=> x = {x}, y = {y}");
    }
}
