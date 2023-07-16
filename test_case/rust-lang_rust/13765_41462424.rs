 rust
fn main() {
    let mut x = ~1;
    let f = || { x = ~2; &*x };

    let g = &f;

    let reference = (*g)();
    (*g)();
    println!("{}", *reference);
}
