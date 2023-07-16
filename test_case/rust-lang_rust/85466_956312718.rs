plain
note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.58.0-nightly (8e3673c9b 2021-11-01) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z symbol-mangling-version=legacy -Z macro-backtrace -Z crate-attr=doc(html_root_url="https://doc.rust-lang.org/nightly/") -Z binary-dep-depinfo -Z force-unstable-if-unmarked -C opt-level=3 -C embed-bitcode=no -C codegen-units=1 -C debuginfo=0 -C debug-assertions=on -C link-args=-Wl,-rpath,$ORIGIN/../lib -C prefer-dynamic -C llvm-args=-import-instr-limit=10 -C embed-bitcode=yes --crate-type lib
note: some of the compiler flags provided by cargo are hidden

query stack during panic:
query stack during panic:
#0 [inferred_outlives_crate] computing the inferred outlives predicates for items in this crate
#1 [inferred_outlives_of] computing inferred outlives predicates of `cell::UnsafeCell`
error: could not compile `core`
warning: build failed, waiting for other jobs to finish...
error: build failed
Build completed unsuccessfully in 0:04:12
