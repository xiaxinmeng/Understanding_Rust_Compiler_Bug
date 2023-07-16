rust
#[repr(packed)]
pub struct PairULE<A, B>(pub A, pub B);

impl<A: ULE, B: ULE> PairULE<A, B> {
    fn foo(&self) {
       do_something_with(&self.0);
       do_something_with(&self.1);
    }
}
