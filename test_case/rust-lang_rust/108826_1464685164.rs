rs
trait LendingIterator {
    type Item<'a>
    where
        Self: 'a;
}

struct Filter<S, P>(S, P);

impl<S, P> LendingIterator for Filter<S, P>
where
    S: LendingIterator,
    P: FnMut(&Self::Item<'_>),
{
    type Item<'a> = S::Item<'a> where S: 'a;
}

