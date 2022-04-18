use gimei::address::Address;
use gimei::name::Name;
use gimei::{Dummy, Gimake, Gimei, Gimeiker};
use rand::rngs::StdRng;
use rand::SeedableRng;

#[allow(dead_code)]
#[derive(Debug, Gimei, Default)]
pub struct Foo {
    #[gimei(generator = "name_with_rng.last.kanji")]
    last_kanji: String,

    #[gimei(generator = "address_with_rng.prefecture.kanji")]
    prefecture_kanji: String,

    order_id: usize,
}

fn main() {
    // type derived Gimei
    let f: Foo = Gimeiker.gimake();
    println!("{:?}", f);

    // from fixed seed
    let seed = [
        1, 0, 0, 0, 23, 0, 0, 0, 200, 1, 0, 0, 210, 30, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0,
    ];

    let r = &mut StdRng::from_seed(seed);
    for _ in 0..5 {
        let v: Foo = Gimeiker.gimake_with_rng(r);
        println!("value from fixed seed {:#?}", v);
    }

    let name = Name::new();
    println!("value {:#?}", name);

    for _ in 0..5 {
        let v = Name::new_with_rand(r);
        println!("from fixed seed {:#?}", v);
    }

    let address = Address::new();
    println!("value {:#?}", address);

    for _ in 0..5 {
        let v = Address::new_with_rand(r);
        println!("from fixed seed {:#?}", v);
    }
}
