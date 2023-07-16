
error: field is never read: `items`
  --> $DIR/field-used-in-ffi-issue-81658.rs:13:5
   |
LL |     items: Option<Vec<T>>,
   |     ^^^^^^^^^^^^^^^^^^^^^ help: if this is intentional, prefix it with an underscore: `_items`
   |
   = note: The leading underscore signals to the reader that while the field may not be read
           by any Rust code, it still serves some other purpose that isn't detected by rustc.
           (e.g. some values are used for their effect when dropped or used in FFI code
           exclusively through raw pointers)
