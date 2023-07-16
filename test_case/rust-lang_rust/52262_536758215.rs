
$ cargo build
!! master-installed (default)
!! Executing '/home/user/.cargo/bin/cargo' in pwd='/tmp/two' with args(1): 'build'
   Compiling two v0.1.0 (/tmp/two)
error[E0507]: cannot move out of a raw pointer
  --> src/main.rs:52:35
   |
52 |             let header = unsafe { *(self.pointer as *const EntryHeader) };
   |                                   ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
   |                                   |
   |                                   move occurs because value has type `EntryHeader`, which does not implement the `Copy` trait
   |                                   help: consider borrowing here: `&*(self.pointer as *const EntryHeader)`

error: internal compiler error: src/librustc_mir/borrow_check/mod.rs:1949: Accessing `(*_8)` with the kind `Write(Move)` shouldn't be possible
  --> src/main.rs:52:35
   |
52 |             let header = unsafe { *(self.pointer as *const EntryHeader) };
   |                                   ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

thread '<unnamed>' panicked at 'Box<Any>', src/librustc_errors/lib.rs:871:9
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports

note: rustc 1.40.0-dev (f3c8eba64 2019-09-28) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z treat-err-as-bug=5 -Z external-macro-backtrace -C debuginfo=2 -C incremental -C target-cpu=native --crate-type bin

note: some of the compiler flags provided by cargo are hidden

error: aborting due to 2 previous errors

For more information about this error, try `rustc --explain E0507`.
error: could not compile `two`.

To learn more, run the command again with --verbose.
