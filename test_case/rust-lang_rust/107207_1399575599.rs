plain
error[E0133]: call to unsafe function is unsafe and requires unsafe block
  --> library/core/src/str/raw.rs:95:60
   |
89 |     unsafe {
   |     ------ items do not inherit unsafety from separate enclosing items
...
95 |                 && super::validations::run_utf8_validation(crate::slice::from_raw_parts(data, len)).is_ok()
   |
   = note: consult the function's documentation for information on how to avoid undefined behavior

error[E0133]: call to unsafe function is unsafe and requires unsafe block
error[E0133]: call to unsafe function is unsafe and requires unsafe block
   --> library/core/src/str/raw.rs:145:60
    |
139 |     unsafe {
    |     ------ items do not inherit unsafety from separate enclosing items
...
145 |                 && super::validations::run_utf8_validation(crate::slice::from_raw_parts(data, len)).is_ok()
    |
    = note: consult the function's documentation for information on how to avoid undefined behavior

For more information about this error, try `rustc --explain E0133`.
