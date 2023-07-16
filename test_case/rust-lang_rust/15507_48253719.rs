 rust
use std::iter::Unfold;

// FWIW this is a great example of where an `exists` existential type operator
// would be nice, so we could avoid defining the St parameter to Unfold.
// Instead I'm just going to use a non-public type.
type IterateSt<'a,T> = (|T|: 'a -> T, Option<T>, bool);

#[allow(visible_private_types)]
pub fn iterate<'a, T: Clone>(f: |T|: 'a -> T, seed: T) -> Unfold<'a, T, IterateSt<'a, T>> {
    Unfold::new((f, Some(seed), true), |st| {
        let &(ref mut f, ref mut val, ref mut first) = st;
        if *first {
            *first = false;
        } else {
            val.mutate(|x| (*f)(x));
        }
        val.clone()
    })
}
