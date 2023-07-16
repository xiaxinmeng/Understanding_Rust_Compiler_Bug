 rust
fn main() {
    let f = |: _| {};
    let g: Box<FnMut(&())> = Box::new(f) as Box<FnMut(&())>;
}
