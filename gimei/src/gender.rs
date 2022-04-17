use rand::Rng;

#[derive(Debug)]
pub enum Gender {
    Female,
    Male,
}

impl Default for Gender {
    fn default() -> Self {
        Gender::sample()
    }
}

impl Gender {
    pub fn sample() -> Gender {
        match rand::random::<usize>() % 2 {
            0 => Gender::Female,
            _ => Gender::Male,
        }
    }

    pub fn with_rand<R: Rng + ?Sized>(rng: &mut R) -> Gender {
        match rng.gen::<usize>() % 2 {
            0 => Gender::Female,
            _ => Gender::Male,
        }
    }

    pub fn as_str(&self) -> &str {
        match self {
            Gender::Female => "female",
            Gender::Male => "male",
        }
    }
}
