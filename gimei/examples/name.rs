use gimei::gender::Gender;
use gimei::name::Name;
use rand::rngs::StdRng;
use rand::SeedableRng;

// cargo run --example name
fn main() {
    let n = Name::new();
    println!("value {:#?}", n);

    let seed = [
        1, 0, 0, 0, 23, 0, 0, 0, 200, 1, 0, 0, 210, 30, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0,
    ];
    let r = &mut StdRng::from_seed(seed);
    for _ in 0..5 {
        let v = Name::new_with_rand(r);
        println!("from fixed seed {:#?}", v);
    }

    let g = Gender::sample();
    println!("value {:#?}", g);

    let gr = Gender::with_rand(r);
    println!("from fixed seed  {:#?}", gr);

    let nrg = Name::new_with_rand_gender(r, g);
    println!("from fixed seed and gender  {:#?}", nrg);
}
