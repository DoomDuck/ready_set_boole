use ready_set_boole::arithmetic::multiplier;

pub fn main() {
    let mut product = 1;
    for arg in std::env::args().skip(1) {
        let a = arg.parse().expect("Invalid number argument");
        product = multiplier(product, a);
    }
    println!("Here is the product: {product}");
}
 