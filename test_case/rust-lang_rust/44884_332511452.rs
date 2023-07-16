
[00:34:19] error[E0133]: borrow of packed field requires unsafe function or block
[00:34:19]    --> /cargo/registry/src/github.com-1ecc6299db9ec823/tendril-0.3.1/src/tendril.rs:486:26
[00:34:19]     |
[00:34:19] 486 |         let kind = match self.ptr.get().get() {
[00:34:19]     |                          ^^^^^^^^ borrow of packed field
[00:34:19] 
[00:34:19] error[E0133]: borrow of packed field requires unsafe function or block
[00:34:19]    --> /cargo/registry/src/github.com-1ecc6299db9ec823/tendril-0.3.1/src/tendril.rs:562:15
[00:34:19]     |
[00:34:19] 562 |         match self.ptr.get().get() {
[00:34:19]     |               ^^^^^^^^ borrow of packed field
[00:34:19] 
[00:34:19] error[E0133]: borrow of packed field requires unsafe function or block
[00:34:19]    --> /cargo/registry/src/github.com-1ecc6299db9ec823/tendril-0.3.1/src/tendril.rs:572:17
[00:34:19]     |
[00:34:19] 572 |         let n = self.ptr.get().get();
[00:34:19]     |                 ^^^^^^^^ borrow of packed field
[00:34:19] 
[00:34:19] error[E0133]: borrow of packed field requires unsafe function or block
[00:34:19]    --> /cargo/registry/src/github.com-1ecc6299db9ec823/tendril-0.3.1/src/tendril.rs:580:17
[00:34:19]     |
[00:34:19] 580 |         let n = self.ptr.get().get();
[00:34:19]     |                 ^^^^^^^^ borrow of packed field
[00:34:19] 
[00:34:19] error[E0133]: borrow of packed field requires unsafe function or block
[00:34:19]    --> /cargo/registry/src/github.com-1ecc6299db9ec823/tendril-0.3.1/src/tendril.rs:582:39
[00:34:19]     |
[00:34:19] 582 |         (n > MAX_INLINE_TAG) && (n == other.ptr.get().get())
[00:34:19]     |                                       ^^^^^^^^^ borrow of packed field
[00:34:19] 
[00:34:19] error[E0133]: borrow of packed field requires unsafe function or block
[00:34:19]    --> /cargo/registry/src/github.com-1ecc6299db9ec823/tendril-0.3.1/src/tendril.rs:588:12
[00:34:19]     |
[00:34:19] 588 |         if self.ptr.get().get() <= MAX_INLINE_TAG {
[00:34:19]     |            ^^^^^^^^ borrow of packed field
[00:34:19] 
[00:34:19] error[E0133]: borrow of packed field requires unsafe function or block
[00:34:19]    --> /cargo/registry/src/github.com-1ecc6299db9ec823/tendril-0.3.1/src/tendril.rs:589:13
[00:34:19]     |
[00:34:19] 589 |             self.ptr.set(unsafe { NonZeroUsize::new(EMPTY_TAG) });
[00:34:19]     |             ^^^^^^^^ borrow of packed field
[00:34:19] 
[00:34:19] error: aborting due to 7 previous errors
