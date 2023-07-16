rust
// FAIL
mod test_lifetime_param {
    type Ty<'a> = impl Sized;
    fn defining(a: &str) -> Ty<'_> { a }
    fn assert_static<'a: 'static>() {}
    fn test<'a>() where Ty<'a>: 'static { assert_static::<'a>() }
}

// PASS
mod test_type_param {
    type Ty<A> = impl Sized;
    fn defining<A>(s: A) -> Ty<A> { s }
    fn assert_static<A: 'static>() {}
    fn test<A>() where Ty<A>: 'static { assert_static::<A>() }
}
