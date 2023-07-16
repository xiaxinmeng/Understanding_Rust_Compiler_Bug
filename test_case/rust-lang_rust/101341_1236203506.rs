plain
    Checking alloc v0.0.0 (/checkout/library/alloc)
error: variable does not need to be mutable
  --> library/alloc/src/raw_vec/tests.rs:43:9
   |
43 |     let mut v: RawVec<u8, _> = RawVec::with_capacity_in(50, a);
   |         |
   |         help: remove this `mut`
   |
   |
   = note: `-D unused-mut` implied by `-D warnings`
error: could not compile `alloc` due to previous error
warning: build failed, waiting for other jobs to finish...
Build completed unsuccessfully in 0:01:51
