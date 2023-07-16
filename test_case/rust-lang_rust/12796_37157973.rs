
use std::iter;

struct Struct {
    data: i32,
}

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
    fn item(&self, index: uint) -> Struct;
    fn num_items(&self) -> uint;

    fn items<'a>(&'a self) -> MapIterator<uint,Struct,&'a Self,iter::Range<uint>> {
        fn map_fn<'a>(index: uint, df: & &'a Self) -> Struct {
            df.item(index)
        }
        MapIterator { data: self, iterator: range(0, self.num_items()), map_fn: map_fn }
    }
}

fn main() { }
