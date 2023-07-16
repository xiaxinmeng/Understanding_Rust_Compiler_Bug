rust
fn absurd<T>() -> T { todo!() }
fn one<'a: 'a>() -> *mut impl Sized {
    absurd::<*mut ()>()
}
fn two<'a: 'a>() -> *mut impl Sized {
    one::<'a>()
    //~^ ERROR non-defining opaque type use in defining scope
}
