rust
let r: Result<core::convert::Infallible, i32> = Err(5);
match r {
    Err(five) => ...,
    // rustc demands an Ok match arm, even though it only contains an Infallible.
    // you must add:
    Ok(never) => match never {},
}
