mod traits;
mod faker;
mod engine;
mod random;
mod fixed;
mod fake;

pub use crate::faker::Faker;
pub use crate::engine::FakerValue;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
