rust
#![feature(nll)]

use std::mem;

struct Node<T, U> {
    links_len: usize,
    entry: Entry<T, U>,
    links: [*mut Node<T, U>; 0],
}

impl<T, U> Node<T, U> {
    fn get_pointer(&self, index: usize) -> &*mut Node<T, U> {
        loop { }
    }
}

pub struct Entry<T, U> {
    pub key: T,
    pub value: U,
}

pub struct SkipMapIter<'a, T, U>
where
    T: 'a,
    U: 'a,
{
    current: &'a *mut Node<T, U>,
}

impl<'a, T, U> Iterator for SkipMapIter<'a, T, U>
where
    T: 'a + Ord,
    U: 'a,
{
    type Item = (&'a T, &'a U);

    fn next(&mut self) -> Option<Self::Item> {
        if self.current.is_null() {
            None
        } else {
            unsafe {
                let Entry { ref key, ref value } = (**self.current).entry;
                mem::replace(&mut self.current, &*(**self.current).get_pointer(0));
                Some((key, value))
            }
        }
    }
}

fn main() { }
