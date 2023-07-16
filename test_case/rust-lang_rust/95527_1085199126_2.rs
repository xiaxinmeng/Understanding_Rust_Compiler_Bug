rust
struct T<'scope, 'up : 'scope> (
    &'scope mut (&'scope (), &'up ()), // in practice `'up` can have whatever variance, even contra; it's the non-covariance of `'scope` which scares Rust into thinking it may have to deal with too big of a `'scope`
);

impl<'scope, 'up : 'scope> T<'scope, 'up> {
    fn set<U> (self, _: &'scope U) {}
}

fn with_higher_order<'up, F : for<'scope> FnOnce(T<'scope, 'up>)> (_: F)
{}

fn foo<'at_x, 'x> (at_x: &'at_x &'x ())
{
    with_higher_order/*::<'at_x>*/(move |it| {
        it.set(at_x);
    })
}
