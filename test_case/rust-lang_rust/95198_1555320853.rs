plain
   Compiling memchr v2.5.0
   Compiling std v0.0.0 (/checkout/library/std)
   Compiling compiler_builtins v0.1.91
   Compiling unwind v0.0.0 (/checkout/library/unwind)
error[E0015]: cannot call non-const fn `slice::<impl [T]>::get_unchecked::<RangeFrom<usize>>` in constant functions
    |
    |
538 |             let last = unsafe { self.get_unchecked(self.len() - N..) };
    |
    = note: calls in constant functions are limited to constant functions, tuple structs and tuple variants


error[E0015]: cannot call non-const fn `slice::<impl [T]>::get_unchecked_mut::<RangeFrom<usize>>` in constant functions
    |
    |
569 |             let last = unsafe { self.get_unchecked_mut(self.len() - N..) };
    |
    = note: calls in constant functions are limited to constant functions, tuple structs and tuple variants

For more information about this error, try `rustc --explain E0015`.
