
error: internal compiler error: compiler/rustc_middle/src/ty/subst.rs:529:17: type parameter `T/#0` (T/0) out of range when substituting, substs=[]

thread 'rustc' panicked at 'Box<Any>', /rustc/368275062fb655c1f36e0398f88b15379a1f3c93/compiler/rustc_errors/src/lib.rs:904:9
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.51.0-nightly (368275062 2021-02-02) running on x86_64-unknown-linux-gnu

note: compiler flags: -C embed-bitcode=no -C codegen-units=1 -C debuginfo=2 --crate-type bin

note: some of the compiler flags provided by cargo are hidden

query stack during panic:
#0 [typeck] type-checking `main`
#1 [typeck_item_bodies] type-checking all item bodies
end of query stack
error: aborting due to previous error

error: could not compile `playground`
