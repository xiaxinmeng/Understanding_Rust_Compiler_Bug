rust
fn main() {
    println!("Hello, world!");
}

const FN_PTR: fn(_, _) -> _ = my_fn as fn(_, _) -> _;

async fn my_fn(_a: usize, _b: usize) -> Result<(), ()> {
    Ok(())
}
