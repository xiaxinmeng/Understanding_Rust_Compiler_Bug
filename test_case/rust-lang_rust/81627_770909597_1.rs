`
error: internal compiler error: compiler/rustc_middle/src/ty/subst.rs:529:17: type parameter `T/#0` (T/0) out of range when substituting, substs=[]

thread 'rustc' panicked at 'Box<Any>', /rustc/e0d9f793990d20f8f640097e28556886ba5362f0/compiler/rustc_errors/src/lib.rs:904:9
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.51.0-nightly (e0d9f7939 2021-02-01) running on x86_64-unknown-linux-gnu

query stack during panic:
#0 [typeck] type-checking `run`
#1 [typeck_item_bodies] type-checking all item bodies
end of query stack
error: aborting due to previous error
