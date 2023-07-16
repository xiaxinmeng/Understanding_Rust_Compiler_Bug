rust
fn absurd<T>() -> T { todo!() }
fn assert_static<T: 'static>(_: T) {}
fn one<'a: 'a>() -> *mut impl Sized {
    absurd::<*mut ()>()
}
fn two<'a>() {
    assert_static(one::<'a>());
    //~^ ERROR lifetime may not live long enough
}
