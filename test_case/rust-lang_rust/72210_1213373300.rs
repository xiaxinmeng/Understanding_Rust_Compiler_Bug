rust
fn main() {
    let s = String::new();
    let _foo: Box<dyn Fn()> = Box::new(move || { s; });
}
