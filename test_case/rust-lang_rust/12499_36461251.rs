 rust
fn main() {
    match Ok::<int, ()>(0) {
        Some(x) => {
            x.eq(&0); // ICE
        }
        None => {}
    }
}
