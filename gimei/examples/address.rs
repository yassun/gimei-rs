use gimei::address::Address;

use rand::rngs::StdRng;
use rand::SeedableRng;

// cargo run --example address
fn main() {
    let a = Address::new();
    println!("value {:#?}", a);

    let seed = [
        1, 0, 0, 0, 23, 0, 0, 0, 200, 1, 0, 0, 210, 30, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0,
    ];
    let r = &mut StdRng::from_seed(seed);

    for _ in 0..5 {
        let v = Address::new_with_rand(r);
        println!("value from fixed seed {:#?}", v);
    }
}
