rust
// A function that just returns its argument, but serves as a type hint:
fn dummy<F>(f: F) -> F where F: for<'a> Fn(&'a u8) -> &'a u8 {
    f
}

// Doesn't work: foo(|x| x);
// Does work:
foo(dummy(|x| x));
