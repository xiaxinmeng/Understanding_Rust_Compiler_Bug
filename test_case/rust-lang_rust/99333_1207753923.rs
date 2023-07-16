rust
fn f(v: Vec<u32>) -> Option<Result<(), Error>> {
    let x = try_something()?; // turns Err into Some(Err)
    let y = v.last()?; // propagates None
    let z = u8::try_from(y).ok()?; // turns Err into None
    ...
}
