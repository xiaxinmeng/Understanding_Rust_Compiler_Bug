rust
// explicit handling
let Ok(x): Result<u32, _> = y.try_into() else {
    return u32::MAX;
};
// remember we want people to write all that instead of this:
let x = y as u32;
// and the alternative when you have Result return type
let x: u32 = y.try_into()?;
