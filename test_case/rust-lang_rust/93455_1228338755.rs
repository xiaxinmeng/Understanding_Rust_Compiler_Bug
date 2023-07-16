plain
[RUSTC-TIMING] adler test:false 0.102
[RUSTC-TIMING] libc test:false 0.573
[RUSTC-TIMING] compiler_builtins test:false 0.604
[RUSTC-TIMING] unwind test:false 0.050
error[E0520]: `is_zero` specializes an item from a parent `impl`, but that item is not marked `default`
  --> library/alloc/src/vec/is_zero.rs:48:5
   |
48 | /     fn is_zero(&self) -> bool {
49 | |         self.0 == 0
50 | |     }
   | |_____^ cannot specialize default item `is_zero`
...
59 |   unsafe impl<T: IsZero> IsZero for Wrapping<T> {
   |   --------------------------------------------- parent `impl` is here
   |
   = note: to specialize, `is_zero` in the parent `impl` must be marked `default`

error[E0520]: `is_zero` specializes an item from a parent `impl`, but that item is not marked `default`
  --> library/alloc/src/vec/is_zero.rs:54:5
   |
54 | /     fn is_zero(&self) -> bool {
55 | |         self.0 == 0
56 | |     }
   | |_____^ cannot specialize default item `is_zero`
...
59 |   unsafe impl<T: IsZero> IsZero for Wrapping<T> {
   |   --------------------------------------------- parent `impl` is here
   |
   = note: to specialize, `is_zero` in the parent `impl` must be marked `default`

error[E0520]: `is_zero` specializes an item from a parent `impl`, but that item is not marked `default`
  --> library/alloc/src/vec/is_zero.rs:66:5
   |
66 | /     fn is_zero(&self) -> bool {
67 | |         self.0 == 0
68 | |     }
   | |_____^ cannot specialize default item `is_zero`
...
77 |   unsafe impl<T: IsZero> IsZero for Saturating<T> {
   |   ----------------------------------------------- parent `impl` is here
   |
   = note: to specialize, `is_zero` in the parent `impl` must be marked `default`

error[E0520]: `is_zero` specializes an item from a parent `impl`, but that item is not marked `default`
  --> library/alloc/src/vec/is_zero.rs:72:5
   |
72 | /     fn is_zero(&self) -> bool {
73 | |         self.0 == 0
74 | |     }
   | |_____^ cannot specialize default item `is_zero`
...
77 |   unsafe impl<T: IsZero> IsZero for Saturating<T> {
   |   ----------------------------------------------- parent `impl` is here
   |
   = note: to specialize, `is_zero` in the parent `impl` must be marked `default`
For more information about this error, try `rustc --explain E0520`.
[RUSTC-TIMING] alloc test:false 0.858
error: could not compile `alloc` due to 4 previous errors
warning: build failed, waiting for other jobs to finish...
