 rust
fn foo<T: AsSlice>(x: T) {
    let x = x.as_slice();
    ...
}

foo("foo");
foo([1, 2, 3]);
