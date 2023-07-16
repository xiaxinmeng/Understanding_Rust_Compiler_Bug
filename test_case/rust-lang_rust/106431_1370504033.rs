rust
trait Trait<'a> {}
fn test<'a>(_: &'static dyn Trait<'a>) { // implies 'a: 'static !!
    None::<&'static &'a ()>; // check 'a: 'static
}
