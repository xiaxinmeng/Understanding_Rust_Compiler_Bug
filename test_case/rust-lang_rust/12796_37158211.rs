
use std::iter;

struct MapIterator<T,U,D,I> {
    data: D,
    iterator: I,
    // `map` is already an function of an iterator, so we can't use `map` as a name here
    map_fn: fn (T, &D) -> U,
}

impl<T,U,D,I:Iterator<T>> Iterator<U> for MapIterator<T,U,D,I> {
    fn next(&mut self) -> Option<U> {
        self.iterator.next().map(|x| (self.map_fn)(x, &self.data))
    }
}

trait Trait {
    fn items<'a>(&'a self) -> MapIterator<uint,i32,&'a Self,iter::Range<uint>> {
        fn map_fn<'a>(index: uint, df: & &'a Self) -> i32 {
            0
        }
        MapIterator { data: self, iterator: range(0u, 1), map_fn: map_fn }
    }
}

fn main() { }
