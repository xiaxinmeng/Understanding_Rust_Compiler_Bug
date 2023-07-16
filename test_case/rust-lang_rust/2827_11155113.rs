
pub trait ExtendedIter<A>: BaseIter<A> {
    pure fn eachi(blk: fn(uint, v: &A) -> bool) {
        eachi(&self, blk)
    }
   // etc.
