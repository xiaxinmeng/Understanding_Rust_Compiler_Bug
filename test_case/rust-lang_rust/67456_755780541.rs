text
error: any use of this value will cause an error
  --> src/main.rs:13:27
   |
13 |         !ptr.is_null() && ptr as usize % mem::align_of::<T>() == 0
   |                           ^^^^^^^^^^^^
   |                           |
   |                           "pointer-to-integer cast" needs an rfc before being allowed inside constants
   |                           inside `is_aligned_and_not_null::<usize>` at src/main.rs:13:27
   |                           inside `slice::from_raw_parts::<usize>` at src/main.rs:18:13
   |                           inside `slice::from_ref::<usize>` at src/main.rs:31:13
   |                           inside `X` at src/main.rs:36:21
...
36 | const X: &[usize] = slice::from_ref(&1);
   | ----------------------------------------
   |
   = note: `#[deny(const_err)]` on by default
