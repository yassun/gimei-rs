use once_cell::sync::Lazy;
use rand::{self, prelude::SliceRandom, Rng};

use crate::gender::Gender;

use crate::load_yaml;

pub static NAME_DATA: Lazy<Vec<yaml_rust::Yaml>> =
    Lazy::new(|| load_yaml("gimei/src/data/names.yml"));

#[derive(Debug, Default)]
pub struct Name {
    pub first: First,
    pub last: Last,
    pub gender: Gender,
}

#[derive(Debug, Default)]
pub struct First {
    pub kanji: String,
    pub hiragana: String,
    pub katakana: String,
}

#[derive(Debug, Default)]
pub struct Last {
    pub kanji: String,
    pub hiragana: String,
    pub katakana: String,
}

impl Name {
    pub fn new() -> Name {
        let mut r = rand::thread_rng();

        Name::new_with_rand(&mut r)
    }

    pub fn new_with_gender(gender: Gender) -> Name {
        let mut r = rand::thread_rng();

        Name::new_with_rand_gender(&mut r, gender)
    }

    pub fn new_with_rand<R: Rng + ?Sized>(rng: &mut R) -> Name {
        let gender = Gender::with_rand(rng);

        Name::new_with_rand_gender(rng, gender)
    }

    pub fn new_with_rand_gender<R: Rng + ?Sized>(rng: &mut R, gender: Gender) -> Name {
        let first = NAME_DATA[0]["first_name"][gender.as_str()]
            .as_vec()
            .unwrap()
            .choose(rng)
            .unwrap()
            .as_vec()
            .unwrap()
            .iter()
            .map(|x| x.as_str().unwrap())
            .collect::<Vec<&str>>();

        let last = NAME_DATA[0]["last_name"]
            .as_vec()
            .unwrap()
            .choose(rng)
            .unwrap()
            .as_vec()
            .unwrap()
            .iter()
            .map(|x| x.as_str().unwrap())
            .collect::<Vec<&str>>();

        Name {
            first: First {
                kanji: first[0].into(),
                hiragana: first[1].into(),
                katakana: first[2].into(),
            },
            last: Last {
                kanji: last[0].into(),
                hiragana: last[1].into(),
                katakana: last[2].into(),
            },
            gender,
        }
    }
}
