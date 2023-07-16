rust
fn produce<T>() -> Result<&'static str, T> {
    Ok("22")
}

fn main() {
    let x: usize = produce()
        .and_then(|x| x.parse())
        .unwrap_or_else(|_| panic!());
    println!("{}", x);
}
