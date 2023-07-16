
use std::num::{Zero, One};
use std::ops::Div;

fn inverse<T: Eq + Div<T,T> + One + Zero>(x: T) -> Result<T, String> {
    if x == Zero::zero() { return Err("x cannot be zero!".to_string()); }

    Ok(One::one() / x)
}
