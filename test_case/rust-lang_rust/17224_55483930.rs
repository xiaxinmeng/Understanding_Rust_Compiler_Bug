 rust
fn inverse<T: Float>(x: T) -> Result<T, &'static str> {
    if x.is_zero() { return Err("x cannot be zero!") }

    Ok(std::num::one::<T>() / x)
}
