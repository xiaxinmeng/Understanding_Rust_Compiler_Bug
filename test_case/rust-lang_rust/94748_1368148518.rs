plain
   Compiling addr2line v0.17.0
error: function cannot return without recursing
   --> library/std/src/fs.rs:796:5
    |
796 |     fn is_read_vectored(&self) -> bool {
    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ cannot return without recursing
797 |         (&*self).is_read_vectored()
    |         --------------------------- recursive call site
    |
    = help: a `loop` may express intention better if this is on purpose
    = note: `-D unconditional-recursion` implied by `-D warnings`
error: function cannot return without recursing
   --> library/std/src/fs.rs:815:5
    |
    |
815 |     fn is_write_vectored(&self) -> bool {
    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ cannot return without recursing
816 |         (&*self).is_write_vectored()
    |         ---------------------------- recursive call site
    |
    = help: a `loop` may express intention better if this is on purpose
error: could not compile `std` due to 2 previous errors
Build completed unsuccessfully in 0:00:23
