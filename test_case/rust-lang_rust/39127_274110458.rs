rust
match Carrier::translate(<expr>) {
    Ok(val) => #[allow(unreachable_code)] val,
    Err(err) => #[allow(unreachable_code)] return Carrier::from_error(From::from(err)),
}
