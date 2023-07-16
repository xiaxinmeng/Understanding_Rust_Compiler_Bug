
thread 'rustc' panicked at 'assertion failed: `(left == right)`
  left: `UserFacing`,
 right: `All`', /checkout/compiler/rustc_middle/src/ty/normalize_erasing_regions.rs:30:9
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace

error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.49.0-nightly (db0665ce8 2020-10-10) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z macro-backtrace -Z binary-dep-depinfo -Z force-unstable-if-unmarked -C opt-level=3 -C embed-bitcode=no -C codegen-units=1 -C debuginfo=0 -C debug-assertions=on -C overflow-checks=off -C link-args=-Wl,-rpath,$ORIGIN/../lib -C prefer-dynamic -C embed-bitcode=yes --crate-type lib

note: some of the compiler flags provided by cargo are hidden

[RUSTC-TIMING] core test:false 6.187
error: could not compile `core`
