 rust
trait ParallelIterator {
    type Item;
    fn drive_unindexed<C>(self, consumer: C) -> <C as Consumer<Self::Item>>::Result
        where C: Consumer<Self::Item>;
}

pub trait Consumer<ITEM> {
    type Result;
}

fn main() { }
