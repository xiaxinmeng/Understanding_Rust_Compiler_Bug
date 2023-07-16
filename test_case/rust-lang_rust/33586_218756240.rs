 rust


trait ParallelIterator {
    type Item;
    fn drive_unindexed<C>(self, consumer: C) -> C::Result
        where C: Consumer<Self::Item>;
}

pub trait Consumer<ITEM> {
    type Result;
}

fn main() { }
