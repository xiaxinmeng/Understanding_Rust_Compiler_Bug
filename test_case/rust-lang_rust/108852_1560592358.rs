rust
pub trait LendingIterator {
    type Item<'a>
    where
        Self: 'a;
    fn next(&'_ mut self) -> Option<Self::Item<'_>>;
}

pub trait PeekableIterator: Iterator + Sized {
    fn peekable_iter(self) -> Peekable<Self> {
        Peekable::new(self)
    }
}

impl<T: Iterator> PeekableIterator for T {}

pub struct Peekable<T: Iterator> {
    inner: T,
    next: Option<T::Item>,
}

impl<T: Iterator> Peekable<T> {
    fn new(mut inner: T) -> Self {
        let next = inner.next();
        Peekable { inner, next }
    }
    fn next(&mut self) -> Option<T::Item> {
        std::mem::replace(&mut self.next, self.inner.next())
    }
}

impl<T: Iterator> LendingIterator for Peekable<T> {
    type Item<'a> = (T::Item, Option<&'a T::Item>) where T: 'a;
    fn next(&'_ mut self) -> Option<Self::Item<'_>> {
        self.next().map(|t| (t, self.next.as_ref()))
    }
}

pub trait LendingIteratorExt: LendingIterator {
    fn for_each<F>(self, mut f: F)
    where
        Self: Sized,
        F: for<'a> FnMut(Self::Item<'a>),
    {
        /*
        #[inline]
        fn call<T>(mut f: impl FnMut(T)) -> impl FnMut((), T) {
            move |(), item| f(item)
        }

        self.fold((), call(f));
        */
        self.fold((), move |_, item| f(item))
    }
    fn fold<B, F>(mut self, init: B, mut f: F) -> B
    where
        Self: Sized,
        F: for<'a> FnMut(B, Self::Item<'a>) -> B,
    {
        let mut accum = init;
        while let Some(x) = self.next() {
            accum = f(accum, x);
        }
        accum
    }
}

impl<T: LendingIterator> LendingIteratorExt for T {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let a = [0, 1, 2, 3, 4];
        //a.iter().peekable_iter().for_each(|(a, b)| {
        a.into_iter().peekable_iter().for_each(|(a, b)| {
            if let Some(b) = b {
                //assert_eq!(*a, *b - 1)
                assert_eq!(a, *b - 1)
            }
        });
    }
}
