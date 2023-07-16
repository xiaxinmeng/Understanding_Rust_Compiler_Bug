plain
   Compiling core v0.0.0 (/checkout/library/core)
   Compiling libc v0.2.99
   Compiling memchr v2.4.1
   Compiling std v0.0.0 (/checkout/library/std)
error: expected one of `::`, `;`, or `as`, found doc comment `/// A wrapper type to construct uninitialized instances of `T`.`
  |
7 | use crate::slice::IterMut
7 | use crate::slice::IterMut
  |                          - expected one of `::`, `;`, or `as`
8 | 
9 | /// A wrapper type to construct uninitialized instances of `T`.

   Compiling compiler_builtins v0.1.49
   Compiling unwind v0.0.0 (/checkout/library/unwind)
error[E0432]: unresolved import `maybe_uninit::MaybeUninit`
---
   |
13 | pub struct IntoIter<T, const N: usize> {
   |                     ^ unused parameter
   |
   = help: consider removing `T`, referring to it in a field, or using a marker such as `PhantomData`
   = help: if you intended `T` to be a const parameter, use `const T: usize` instead
Some errors have detailed explanations: E0392, E0432.
For more information about an error, try `rustc --explain E0392`.
error: could not compile `core` due to 3 previous errors
warning: build failed, waiting for other jobs to finish...
