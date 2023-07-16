plain
    Checking alloc v0.0.0 (/checkout/library/alloc)
    Checking cfg-if v0.1.10
    Checking adler v0.2.3
    Checking rustc-demangle v0.1.21
error[E0520]: `is_zero` specializes an item from a parent `impl`, but that item is not marked `default`
   --> library/alloc/src/vec/is_zero.rs:170:5
    |
150 |   unsafe impl<T: IsZero> IsZero for Wrapping<T> {
    |   --------------------------------------------- parent `impl` is here
...
170 | /     fn is_zero(&self) -> bool {
171 | |         self.0 == 0
172 | |     }
    | |_____^ cannot specialize default item `is_zero`
    |
    = note: to specialize, `is_zero` in the parent `impl` must be marked `default`

error[E0520]: `is_zero` specializes an item from a parent `impl`, but that item is not marked `default`
   --> library/alloc/src/vec/is_zero.rs:177:5
    |
150 |   unsafe impl<T: IsZero> IsZero for Wrapping<T> {
    |   --------------------------------------------- parent `impl` is here
...
177 | /     fn is_zero(&self) -> bool {
178 | |         self.0 == 0
179 | |     }
    | |_____^ cannot specialize default item `is_zero`
    |
    = note: to specialize, `is_zero` in the parent `impl` must be marked `default`

error[E0520]: `is_zero` specializes an item from a parent `impl`, but that item is not marked `default`
   --> library/alloc/src/vec/is_zero.rs:184:5
    |
157 |   unsafe impl<T: IsZero> IsZero for Saturating<T> {
    |   ----------------------------------------------- parent `impl` is here
...
184 | /     fn is_zero(&self) -> bool {
185 | |         self.0 == 0
186 | |     }
    | |_____^ cannot specialize default item `is_zero`
    |
    = note: to specialize, `is_zero` in the parent `impl` must be marked `default`

error[E0520]: `is_zero` specializes an item from a parent `impl`, but that item is not marked `default`
   --> library/alloc/src/vec/is_zero.rs:191:5
    |
157 |   unsafe impl<T: IsZero> IsZero for Saturating<T> {
    |   ----------------------------------------------- parent `impl` is here
...
191 | /     fn is_zero(&self) -> bool {
192 | |         self.0 == 0
193 | |     }
    | |_____^ cannot specialize default item `is_zero`
    |
    = note: to specialize, `is_zero` in the parent `impl` must be marked `default`
For more information about this error, try `rustc --explain E0520`.
error: could not compile `alloc` due to 4 previous errors
Build completed unsuccessfully in 0:01:18
