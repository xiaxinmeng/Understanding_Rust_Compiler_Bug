log
error: no global memory allocator found but one is required; link to std or add `#[global_allocator]` to a static item that implements the GlobalAlloc trait.

error: `#[panic_handler]` function required, but not found

error: language item required, but not found: `eh_personality`

error: `#[alloc_error_handler]` function required, but not found

error[E0599]: no method named `write_fmt` found for type `main::Writer` in the current scope
 --> src/main.rs:6:5
  |
5 |     pub struct Writer;
  |     ------------------ method `write_fmt` not found for this
6 |     write!(Writer, "");
  |     ^^^^^^^^^^^^^^^^^^^ method not found in `main::Writer`
  |
  = help: items from traits can only be used if the trait is implemented and in scope
  = note: the following trait defines an item `write_fmt`, perhaps you need to implement it:
          candidate #1: `core::fmt::Write`
  = note: this error originates in a macro outside of the current crate (in Nightly builds, run with -Z external-macro-backtrace for more info)

error: aborting due to 5 previous errors
