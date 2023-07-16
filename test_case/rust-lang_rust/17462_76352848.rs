
use std::cmp::Eq;

enum List<T: Eq> {
    Cons {
        val: T,
        next: Box<List<T>>,
    },
    Nil,
}

impl <T: Eq> List<T> {
    fn find_mut(&mut self, query: T) -> &mut List<T> {
        use List::{Cons, Nil};
        match *self {
            Nil => self,
            Cons{ref val, ref mut next} =>
            if *val == query { self } else { next.find_mut(query) },
        }
    }
}
