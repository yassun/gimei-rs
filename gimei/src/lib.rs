use rand::Rng;

pub mod address;
pub mod gender;
pub mod name;

// generic trait
pub struct Gimeiker;

impl<T> Gimake for T {}

pub trait Gimake: Sized {
    fn gimake<U>(&self) -> U
    where
        U: Dummy,
    {
        let mut r = rand::thread_rng();

        U::with_rng(&mut r)
    }

    fn gimake_with_rng<U, R: Rng + ?Sized>(&self, rng: &mut R) -> U
    where
        U: Dummy,
    {
        U::with_rng(rng)
    }
}

pub trait Dummy {
    fn with_rng<R: Rng + ?Sized>(rng: &mut R) -> Self;
}

#[cfg(feature = "derive")]
pub use gimei_derive::Gimei;
