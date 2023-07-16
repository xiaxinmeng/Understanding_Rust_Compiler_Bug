plain
   Compiling memchr v2.4.1
   Compiling std v0.0.0 (/checkout/library/std)
   Compiling compiler_builtins v0.1.49
   Compiling unwind v0.0.0 (/checkout/library/unwind)
error[E0425]: cannot find function `memcmp` in this scope
  --> library/core/src/slice/cmp.rs:96:13
   |
96 |             memcmp(self.as_ptr() as *const u8, other.as_ptr() as *const u8, size) == 0


error[E0425]: cannot find function `memcmp` in this scope
   --> library/core/src/slice/cmp.rs:200:22
    |
200 |             unsafe { memcmp(left.as_ptr(), right.as_ptr(), cmp::min(left.len(), right.len())) };

error[E0308]: mismatched types
  --> library/core/src/slice/cmp.rs:96:86
   |
   |
83 | impl<A, B> SlicePartialEq<B> for [A]
   |         - this type parameter
...
96 |             memcmp(self.as_ptr() as *const u8, other.as_ptr() as *const u8, size) == 0
   |                                                                                      ^ expected type parameter `B`, found integer
   = note: expected type parameter `B`
   = note: expected type parameter `B`
                        found type `{integer}`
Some errors have detailed explanations: E0308, E0425.
For more information about an error, try `rustc --explain E0308`.
error: could not compile `core` due to 3 previous errors
Build completed unsuccessfully in 0:01:14
