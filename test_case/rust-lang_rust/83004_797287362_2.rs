
error: associated constant is never used: `BAR`
  --> $DIR/associated-const-dead-code.rs:6:5
   |
LL |     const BAR: u32 = 1;
   |     ^^^^^^^^^^^^^^^^^^^ help: if this is intentional, prefix it with an underscore: `_BAR`
   |
   = note: The leading underscore signals to the reader that while the associated constant may not be used
           by any Rust code, it still serves some other purpose that isn't detected by rustc.
           (e.g. some values are used for their effect when dropped or used in FFI code
           exclusively through raw pointers)
