
error[E0401]: can't use type parameters from outer function
  --> liballoc/raw_vec.rs:63:52
   |
56 | impl<T, A: Alloc> RawVec<T, A> {
   |      - type variable from outer function
...
59 |     pub const fn new_in(a: A) -> Self {
   |                  ------ try adding a local type parameter in this method instead
...
63 |         const CAP: usize = [0, !0][(mem::size_of::<T>() == 0) as usize];
   |                                                    ^ use of type variable from outer function
