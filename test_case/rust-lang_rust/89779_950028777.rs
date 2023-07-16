
fn main() {
    if false {
        stop().unwrap();
    }
    println!("Hello, world!");
}

fn stop() -> Result<std::convert::Infallible, u64> {
    Err(5)
}
