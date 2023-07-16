 rust
// num.rs

pub use Int::{count_ones, leading_zeros, ... };

pub trait Int: ... {
    fn count_ones(self) -> uint;
    fn leading_zeros(self) -> uint;
    ...
}
