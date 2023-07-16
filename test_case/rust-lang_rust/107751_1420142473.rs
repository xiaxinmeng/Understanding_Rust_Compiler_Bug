rust
trait Trait {
    type AsIter<'a>: for<'x> AsIterator<Item<'x> = &'x u32> where Self: 'a;
}
