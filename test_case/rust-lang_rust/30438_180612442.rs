 Rust
fn silly<'y, 'a>(s: &'y Test<'a>) -> &'y <Test<'a> as Trait>::Out {
    let x = Test { s: &s.s }; &x
}
