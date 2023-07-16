 rust
fn main() {
    let f = || {};
    let g: Box<FnMut(&())> = Box::new(f) as Box<FnMut(&())>;
}
