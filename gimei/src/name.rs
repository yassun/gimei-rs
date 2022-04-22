use once_cell::sync::Lazy;
use rand::{self, prelude::SliceRandom, Rng};
use yaml_rust::YamlLoader;

use crate::gender::Gender;

pub static NAME_DATA: Lazy<Vec<yaml_rust::Yaml>> =
    Lazy::new(|| YamlLoader::load_from_str(include_str!("data/names.yml")).unwrap());

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

    pub fn kanji(&self) -> String {
        format!("{} {}", self.last.kanji, self.first.kanji)
    }

    pub fn katakana(&self) -> String {
        format!("{} {}", self.last.katakana, self.first.katakana)
    }

    pub fn hiragana(&self) -> String {
        format!("{} {}", self.last.hiragana, self.first.hiragana)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_kanji() {
        let name = Name::new();

        assert_eq!(
            name.kanji(),
            format!("{} {}", name.last.kanji, name.first.kanji)
        );
    }

    #[test]
    fn test_katakana() {
        let name = Name::new();

        assert_eq!(
            name.katakana(),
            format!("{} {}", name.last.katakana, name.first.katakana)
        );
    }

    #[test]
    fn test_hiragana() {
        let name = Name::new();

        assert_eq!(
            name.hiragana(),
            format!("{} {}", name.last.hiragana, name.first.hiragana)
        );
    }
}
