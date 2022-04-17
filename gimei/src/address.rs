use once_cell::sync::Lazy;
use rand::{self, prelude::SliceRandom, Rng};

use crate::load_yaml;

pub static ADDRESS_DATA: Lazy<Vec<yaml_rust::Yaml>> =
    Lazy::new(|| load_yaml("gimei/src/data/addresses.yml"));

#[derive(Debug, Default)]
pub struct Address {
    pub prefecture: Prefecture,
    pub city: City,
    pub town: Town,
}

#[derive(Debug, Default)]
pub struct Prefecture {
    pub kanji: String,
    pub hiragana: String,
    pub katakana: String,
}

#[derive(Debug, Default)]
pub struct City {
    pub kanji: String,
    pub hiragana: String,
    pub katakana: String,
}

#[derive(Debug, Default)]
pub struct Town {
    pub kanji: String,
    pub hiragana: String,
    pub katakana: String,
}

impl Address {
    pub fn new() -> Address {
        let mut r = rand::thread_rng();

        Address::new_with_rand(&mut r)
    }

    pub fn new_with_rand<R: Rng + ?Sized>(rng: &mut R) -> Address {
        let prefecture = ADDRESS_DATA[0]["addresses"]["prefecture"]
            .as_vec()
            .unwrap()
            .choose(rng)
            .unwrap()
            .as_vec()
            .unwrap()
            .iter()
            .map(|x| x.as_str().unwrap())
            .collect::<Vec<&str>>();

        let city = ADDRESS_DATA[0]["addresses"]["city"]
            .as_vec()
            .unwrap()
            .choose(rng)
            .unwrap()
            .as_vec()
            .unwrap()
            .iter()
            .map(|x| x.as_str().unwrap())
            .collect::<Vec<&str>>();

        let town = ADDRESS_DATA[0]["addresses"]["town"]
            .as_vec()
            .unwrap()
            .choose(rng)
            .unwrap()
            .as_vec()
            .unwrap()
            .iter()
            .map(|x| x.as_str().unwrap())
            .collect::<Vec<&str>>();

        Address {
            prefecture: Prefecture {
                kanji: prefecture[0].into(),
                hiragana: prefecture[1].into(),
                katakana: prefecture[2].into(),
            },
            city: City {
                kanji: city[0].into(),
                hiragana: city[1].into(),
                katakana: city[2].into(),
            },
            town: Town {
                kanji: town[0].into(),
                hiragana: town[1].into(),
                katakana: town[2].into(),
            },
        }
    }
}
