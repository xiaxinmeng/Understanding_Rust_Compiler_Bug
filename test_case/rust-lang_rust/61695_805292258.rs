rust
fn infallible_op() -> Result<String, bad::Never> {
    Ok("hello".into())
}

fn main() {
    let x: String = infallible_op().into_ok();
    println!("Great results: {}", x);
}
