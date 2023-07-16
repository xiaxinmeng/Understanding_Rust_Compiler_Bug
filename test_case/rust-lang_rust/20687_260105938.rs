 rust
fn foo<F: FnMut(usize) -> usize>(mut f: F) {
    println!("{}", f(42));
}

fn bar<F: FnOnce(usize) -> usize>(f: F) {
    foo(f)
}

fn main() {
    foo(|x| x*2)
}
