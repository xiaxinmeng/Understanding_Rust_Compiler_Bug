 rust
use std::num::{Zero, One};
use std::ops::Div;

fn inverse<T: Div<T,T> + One + Zero>(x: T) -> Result<T, String> {
    if x.is_zero() { return Err("x cannot be zero!".to_string()); }

    Ok(std::num::one::<T>() / x)
}
