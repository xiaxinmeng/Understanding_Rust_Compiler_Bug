rust
fn main() {
    fn foo<F: Fn()>(f: impl Fn(Box<F>, ()), x: (Box<F>, ())) {}
    foo(|f, _| (*f)(), (Box::new(|| println!("lol")), ()));
}
