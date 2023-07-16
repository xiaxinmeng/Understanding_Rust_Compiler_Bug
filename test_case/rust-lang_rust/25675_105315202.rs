 rust
type CoalesceFn<I> where I: Iterator =
    Coalesce<I, fn(I::Item, I::Item) -> Result<I::Item, (I::Item, I::Item)>>;
