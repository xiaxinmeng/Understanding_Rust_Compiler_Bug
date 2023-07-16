rust
use std::marker::PhantomData;

pub struct Punctuated<T, P> {
    t: PhantomData<T>,
    p: PhantomData<P>,
}

impl<T, P> IntoIterator for Punctuated<T, P> {
    type Item = T;
    type IntoIter = IntoIter<T, P>;

    fn into_iter(self) -> Self::IntoIter {
        unimplemented!()
    }
}

pub type IntoIter<T, P = ()> = <Punctuated<T, P> as private::IntoIterator>::IntoIter;

mod private {
    use super::Punctuated;
    use std::marker::PhantomData;

    pub trait IntoIterator {
        type IntoIter;
    }

    impl<T, P> IntoIterator for Punctuated<T, P> {
        type IntoIter = IntoIter<T>;
    }

    pub struct IntoIter<T> {
        t: PhantomData<T>,
    }

    impl<T> Iterator for IntoIter<T> {
        type Item = T;

        fn next(&mut self) -> Option<Self::Item> {
            unimplemented!()
        }
    }
}

struct A;
struct B;
struct C;
fn f(arg: IntoIter<A, B>) -> IntoIter<A, C> {
    // SCARY assignment
    arg
}
