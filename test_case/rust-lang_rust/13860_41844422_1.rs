 rust
pub trait Countable : Bounded {
    fn count() -> uint {
        let min: Self = Bounded::min_value();
        let max: Self = Bounded::max_value();
        max - min + 1
    }
}
