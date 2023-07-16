rust
let this = Cell::new(1u8);
this.with(|inner: &u8| {
    // This changes the value of inner despite it being behind an immutable reference.
    // This is a violation of the aliasing rules.
    this.set(2);
});
