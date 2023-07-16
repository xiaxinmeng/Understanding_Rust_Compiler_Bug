rust
fn main() {
    println!("{}", std::mem::align_of::<Wrapper<dyn My>>());
}
