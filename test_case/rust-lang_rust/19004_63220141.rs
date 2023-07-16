 rust
fn main() {
    let mut f = Foo { b: 0, v: vec![1, 2, 3] };
    let b = &mut f.b;

    f.v.iter().map(|s| {
        incr(b);
        println!("{}: {}", b, s);
    }).last();
}
