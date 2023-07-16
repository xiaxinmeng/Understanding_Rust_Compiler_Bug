
error[E0599]: no method named `write_all_vectored` found for struct `sys::wasi::stdio::Stdout` in the current scope
    --> src/libstd/io/stdio.rs:136:16
     |
136  |         self.0.write_all_vectored(bufs)
     |                ^^^^^^^^^^^^^^^^^^ help: there is an associated function with a similar name: `write_vectored`
     | 
    ::: src/libstd/sys/wasi/stdio.rs:6:1
     |
6    | pub struct Stdout;
     | ------------------ method `write_all_vectored` not found for this
     |
     = help: items from traits can only be used if the trait is implemented and in scope
note: `io::Write` defines an item `write_all_vectored`, perhaps you need to implement it
    --> src/libstd/io/mod.rs:1264:1
     |
1264 | pub trait Write {
     | ^^^^^^^^^^^^^^^

error[E0599]: no method named `write_fmt` found for struct `sys::wasi::stdio::Stdout` in the current scope
   --> src/libstd/io/stdio.rs:140:16
    |
140 |         self.0.write_fmt(fmt)
    |                ^^^^^^^^^ method not found in `sys::wasi::stdio::Stdout`
    | 
   ::: src/libstd/sys/wasi/stdio.rs:6:1
    |
6   | pub struct Stdout;
    | ------------------ method `write_fmt` not found for this
    |
    = help: items from traits can only be used if the trait is implemented and in scope
    = note: the following traits define an item `write_fmt`, perhaps you need to implement one of them:
            candidate #1: `io::Write`
            candidate #2: `core::fmt::Write`

error: aborting due to 6 previous errors; 1 warning emitted

For more information about this error, try `rustc --explain E0599`.
[RUSTC-TIMING] std test:false 2.411
error: could not compile `std`.
