rust
use std::marker::PhantomData;

pub struct Punctuated<T, P> {
    t: PhantomData<T>,
    p: PhantomData<P>,
}

impl<'a, T, P> IntoIterator for &'a Punctuated<T, P> {
    type Item = T;
    type IntoIter = Iter<'a, T, P>;

    fn into_iter(self) -> Self::IntoIter {
        unimplemented!()
    }
}

pub type Iter<'a, T, P = ()> = <&'a Punctuated<T, P> as private::IntoIterator>::IntoIter;

mod private {
    use super::Punctuated;
    use std::marker::PhantomData;

    pub trait IntoIterator {
        type IntoIter;
    }

    impl<'a, T, P> IntoIterator for &'a Punctuated<T, P> {
        type IntoIter = Iter<'a, T>;
    }

    pub struct Iter<'a, T: 'a> {
        t: PhantomData<&'a T>,
    }

    impl<'a, T> Iterator for Iter<'a, T> {
        type Item = &'a T;

        fn next(&mut self) -> Option<Self::Item> {
            unimplemented!()
        }
    }
}

struct A;
struct B;
struct C;
fn f(arg: Iter<A, B>) -> Iter<A, C> {
    // SCARY assignment
    arg
}
