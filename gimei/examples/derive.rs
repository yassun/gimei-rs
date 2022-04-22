use gimei::address::Address;
use gimei::name::Name;
use gimei::{Dummy, Gimake, Gimeiker};
use gimei_derive::Gimei;
use rand::prelude::StdRng;
use rand::SeedableRng;

#[allow(dead_code)]
#[derive(Debug, Gimei, Default)]
struct NameExample {
    pub example: String,

    #[gimei(generator = "name_with_rng.kanji()")]
    pub kanji: String,

    #[gimei(generator = "name_with_rng.hiragana()")]
    pub hiragana: String,

    #[gimei(generator = "name_with_rng.katakana()")]
    pub katakana: String,

    #[gimei(generator = "name_with_rng.last.kanji")]
    pub last_kanji: String,

    #[gimei(generator = "name_with_rng.last.hiragana")]
    pub last_hiragana: String,

    #[gimei(generator = "name_with_rng.last.katakana")]
    pub last_katakana: String,

    #[gimei(generator = "name_with_rng.first.kanji")]
    pub first_kanji: String,

    #[gimei(generator = "name_with_rng.first.hiragana")]
    pub first_hiragana: String,

    #[gimei(generator = "name_with_rng.first.katakana")]
    pub first_katakana: String,

    #[gimei(generator = "address_with_rng.prefecture.kanji")]
    pub prefecture_kanji: String,

    #[gimei(generator = "address_with_rng.prefecture.hiragana")]
    pub prefecture_hiragana: String,

    #[gimei(generator = "address_with_rng.prefecture.katakana")]
    pub prefecture_katakana: String,

    #[gimei(generator = "address_with_rng.city.kanji")]
    pub city_kanji: String,

    #[gimei(generator = "address_with_rng.city.hiragana")]
    pub city_hiragana: String,

    #[gimei(generator = "address_with_rng.city.katakana")]
    pub city_katakana: String,

    #[gimei(generator = "address_with_rng.town.kanji")]
    pub town_kanji: String,

    #[gimei(generator = "address_with_rng.town.hiragana")]
    pub town_hiragana: String,

    #[gimei(generator = "address_with_rng.town.katakana")]
    pub town_katakana: String,
}

// cargo run --example derive
fn main() {
    let name: NameExample = Gimeiker.gimake();
    println!("value {:#?}", name);

    // from fixed seed
    let seed = [
        1, 0, 0, 0, 23, 0, 0, 0, 200, 1, 0, 0, 210, 30, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0,
    ];
    let r = &mut StdRng::from_seed(seed);
    for _ in 0..5 {
        let v: NameExample = Gimeiker.gimake_with_rng(r);
        println!("value from fixed seed {:#?}", v);
    }
}
