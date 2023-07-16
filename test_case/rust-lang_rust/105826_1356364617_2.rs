rust
fn one<'a, 'b: 'b>() -> &'a impl Sized {
    &()
}
fn two<'a, 'b>() {
    one::<'a, 'b>();
    //~^ ERROR lifetime may not live long enough
}
