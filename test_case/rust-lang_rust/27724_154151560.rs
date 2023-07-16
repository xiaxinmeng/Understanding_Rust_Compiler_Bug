 rust
struct Rank2Compare<F>(F);

impl<A, B, F> FnMut(A, A) -> Ordering for Rank2Compare<F> 
    where F: FnMut(A) -> B, B: Ord
{ /* ... */ }

iter.min_by(|a, b| a.field1.cmp(&b.field1));
iter.min_by(Rank2Compare(|a| &a.field1));
