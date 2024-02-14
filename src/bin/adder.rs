use ready_set_boole::arithmetic::adder;

pub fn main() {
    let mut sum = 0;
    for arg in std::env::args().skip(1) {
        let a = arg.parse().expect("Invalid number argument");
        sum = adder(sum, a);
    }
    println!("Here is the sum: {sum}");
}
