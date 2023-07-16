rust
fn main() {
    #[repr(bogus_statement)]
    //~^ ERROR the `#[repr]` attribute cannot be applied to statements or expressions
    let x = 0;

    // ... make sure to test expressions as well!
}
