
error: field is never read: `items`
  --> $DIR/field-used-in-ffi-issue-81658.rs:13:5
   |
LL |     items: Option<Vec<T>>,
   |     ^^^^^^^^^^^^^^^^^^^^^ help: if this is intentional, prefix it with an underscore: `_items`
   |
   = note: the leading underscore helps signal to the reader that the field may still serve
           a purpose even if it isn't used in a way that we can detect (e.g. the field
           is only used through FFI or used only for its effect when dropped)
