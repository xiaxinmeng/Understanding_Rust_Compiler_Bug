
error[E0425]: cannot find value `signal` in this scope
  --> crates/core/src/lib.rs:36:55
   |
36 |         .each(0..num_cpus::get(), |_| block_on(ex.run(signal.listen())))
   |                                                       ^^^^^^ not found in this scope
   |
help: consider importing this function
   |
4  | use crate::chrono::month_serde::ser::net::io::sys::signal;
   |

error: internal compiler error: compiler/rustc_middle/src/ty/sty.rs:2150:18: tuple_fields called on non-tuple

thread 'rustc' panicked at 'Box<Any>', compiler/rustc_errors/src/lib.rs:945:9
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.49.0-nightly (ffa2e7ae8 2020-10-24) running on x86_64-unknown-linux-gnu

note: compiler flags: -C embed-bitcode=no -C debuginfo=2 --crate-type lib

note: some of the compiler flags provided by cargo are hidden

query stack during panic:
 #0 [typeck] type-checking `block_on`
 #1 [typeck_item_bodies] type-checking all item bodies
end of query stack
error: aborting due to 2 previous errors

For more information about this error, try `rustc --explain E0425`.
error: could not compile `yxd-auth-core`
