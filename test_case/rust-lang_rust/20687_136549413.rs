 rust
fn foo<F: FnMut(isize) -> isize>(mut f: F) {
    println!("{}", f(42));
}

fn bar<F: FnOnce(isize) -> isize>(f: F) {
    foo(f)
}

fn main() {
    foo(|x| x*2)
}
