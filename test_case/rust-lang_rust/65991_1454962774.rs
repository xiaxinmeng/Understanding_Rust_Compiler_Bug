rust
#![feature(coerce_unsized)]
// No feature(trait_upcasting)

fn upcast_me<'a, T>(x: &'a T) -> &'a dyn Any
where
    T: ?Sized,
    &'a T: CoerceUnsized<&'a dyn Any>,
{
    x as &dyn Any
}

//...
let _ = upcast_me::<dyn Trait>(&());
