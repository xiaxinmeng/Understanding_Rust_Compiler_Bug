
#![no_implicit_prelude]
#![feature(specialization)]

extern crate core;
extern crate std;

use core::marker::Sized;
use core::ops::FnMut;
use core::option::Option;
use std::panic;
use std::unimplemented;

pub trait Iterator {
    type Item;
    fn next(&mut self) -> Option<Self::Item>;
    fn fuse(self) -> Fuse<Self>
    where
        Self: Sized,
    {
        unimplemented!()
    }
}

trait FusedIterator: Iterator {}
pub struct Fuse<I>(I);
impl<I> Iterator for Fuse<I>
where
    I: Iterator,
{
    type Item = I::Item;
    fn next(&mut self) -> Option<Self::Item> {
        unimplemented!()
    }
}

impl<I> Iterator for Fuse<I> where I: FusedIterator {}

impl<'a, I> FusedIterator for Fuse<I> where I: Iterator {}

pub fn f(s: &Mystr) {
    let mut it = s.split(|c: char| c == ',').fuse();
    let t = it.next();
    let u = t.unwrap();
    let v = u.parse::<usize>().unwrap();
}

pub struct Mystr;

impl Mystr {
    pub fn split<'a, P>(&'a self, pat: P) -> Split<'a, P>
    where
        P: Pattern<'a>,
    {
        Split(pat.into_searcher(self))
    }
}

pub trait Pattern<'a> {
    type Searcher: Searcher<'a>;

    fn into_searcher(self, haystack: &'a Mystr) -> Self::Searcher;
}

impl<'a, F> Pattern<'a> for F
where
    F: FnMut(char) -> bool,
{
    type Searcher = CharPredicateSearcher<'a, F>;

    fn into_searcher(self, haystack: &'a Mystr) -> Self::Searcher {
        unimplemented!()
    }
}

pub struct CharPredicateSearcher<'a, F>(F, core::marker::PhantomData<&'a ()>)
where
    F: FnMut(char) -> bool;

unsafe impl<'a, F> Searcher<'a> for CharPredicateSearcher<'a, F> where F: FnMut(char) -> bool {}

pub unsafe trait Searcher<'a> {}

pub struct Split<'a, P>(<P as Pattern<'a>>::Searcher)
where
    P: Pattern<'a>;

impl<'a, P> Iterator for Split<'a, P>
where
    P: Pattern<'a>,
{
    type Item = &'a str;
    fn next(&mut self) -> Option<Self::Item> {
        unimplemented!()
    }
}

impl<'a, P> FusedIterator for Split<'a, P> where P: Pattern<'a> {}

fn main() {}
