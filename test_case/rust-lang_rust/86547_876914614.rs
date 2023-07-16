
fn foo(x: Option<i32>) -> Result<(), &'static str> {
    let _ = x.ok_or_else(|| Err(      &         "nope"))?;
    Ok(())
}

fn main() {
    println!("{:?}", foo(Some(1)));
}
