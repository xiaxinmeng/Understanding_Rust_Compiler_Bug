rust
fn main() {
    let f: Box<dyn FnOnce()> = Box::new(||{});
    f()
}
