
thread 'rustc' panicked at 'assertion failed: `(left == right)`
  left: `std::ops::FnMut<(&u8,)>`,
 right: `std::ops::FnOnce<(&u8,)>`', compiler/rustc_symbol_mangling/src/v0.rs:507:21
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace

error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.53.0-dev running on x86_64-apple-darwin

note: compiler flags: -Z symbol-mangling-version=v0

query stack during panic:
#0 [symbol_name] computing the symbol for `f::<&dyn for<'r> std::ops::FnMut(&'r u8)>`
#1 [collect_and_partition_mono_items] collect_and_partition_mono_items
end of query stack
