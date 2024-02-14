use ready_set_boole::arithmetic::gray_code;

pub fn main() {
    for arg in std::env::args().skip(1) {
        let a = arg.parse().expect("Invalid number argument");
        println!("{a:8b} -> {:8b}", gray_code(a));
    }
}

