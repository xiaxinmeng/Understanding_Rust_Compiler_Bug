plain
   Compiling memchr v2.4.1
   Compiling std v0.0.0 (/checkout/library/std)
   Compiling compiler_builtins v0.1.55
   Compiling unwind v0.0.0 (/checkout/library/unwind)
error: use of associated function `array::iter::IntoIter::<T, N>::new` that will be deprecated in future version 1.59.0: use `IntoIterator::into_iter` instead
    |
601 |         let mut iter = IntoIter::new(self);
    |                                  ^^^
    |
    |
    = note: `-D deprecated-in-future` implied by `-D warnings`
error: could not compile `core` due to previous error
Build completed unsuccessfully in 0:00:11
