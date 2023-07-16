plain
   Compiling memchr v2.4.1
   Compiling std v0.0.0 (/checkout/library/std)
   Compiling compiler_builtins v0.1.70
   Compiling unwind v0.0.0 (/checkout/library/unwind)
error[E0596]: cannot borrow `*self` as mutable, as it is behind a `&` reference
     |
     |
4022 |     pub fn flatten(&self) -> &[T] {
     |                    ----- help: consider changing this to be a mutable reference: `&mut self`
...
4031 |         unsafe { from_raw_parts_mut(self.as_mut_ptr().cast(), len) }
     |                                     ^^^^^^^^^^^^^^^^^ `self` is a `&` reference, so the data it refers to cannot be borrowed as mutable
For more information about this error, try `rustc --explain E0596`.
error: could not compile `core` due to previous error
Build completed unsuccessfully in 0:04:06
