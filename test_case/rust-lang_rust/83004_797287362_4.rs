
error: associated constant is never used: `BAR`
  --> $DIR/associated-const-dead-code.rs:6:5
   |
LL |     const BAR: u32 = 1;
   |     ^^^^^^^^^^^^^^^^^^^ help: if this is intentional, prefix it with an underscore: `_BAR`
   |
   = note: the leading underscore helps signal to the reader that the associated constant may still serve
           a purpose even if it isn't used in a way that we can detect (e.g. the associated constant
           is only used through FFI or used only for its effect when dropped)
