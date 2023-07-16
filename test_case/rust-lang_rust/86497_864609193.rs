plain
   Compiling unwind v0.0.0 (/checkout/library/unwind)
error[E0308]: mismatched types
   --> library/core/src/str/mod.rs:249:22
    |
241 |     pub fn nearest_char_boundary_max(&self, index: usize) -> usize {
    |                                                              ----- expected `usize` because of return type
...
249 |             unsafe { index.unwrap_unchecked() }
    |                      ^^^^^^^^^^^^^^^^^^^^^^^^ expected `usize`, found `&u8`

error[E0606]: casting `&&u8` as `i8` is invalid
    |
    |
246 |             let index = self.as_bytes()[lower_bound..=index].iter().rfind(|b| (b as i8) >= -0x40);
    |
    |
    = help: cast through a raw pointer first
error[E0308]: mismatched types
   --> library/core/src/str/mod.rs:284:29
    |
    |
284 |             index.unwrap_or(upper_bound)
    |                             ^^^^^^^^^^^ expected `&u8`, found `usize`
error[E0308]: mismatched types
   --> library/core/src/str/mod.rs:284:13
    |
    |
276 |     pub fn nearest_char_boundary_min(&self, index: usize) -> usize {
    |                                                              ----- expected `usize` because of return type
...
284 |             index.unwrap_or(upper_bound)
    |             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected `usize`, found `&u8`

error[E0606]: casting `&&u8` as `i8` is invalid
    |
    |
281 |             let index = self.as_bytes()[index..upper_bound].iter().find(|b| (b as i8) >= -0x40);
    |
    |
    = help: cast through a raw pointer first
error: aborting due to 5 previous errors

Some errors have detailed explanations: E0308, E0606.
For more information about an error, try `rustc --explain E0308`.
