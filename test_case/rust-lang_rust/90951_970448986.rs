plain
[RUSTC-TIMING] build_script_build test:false 0.356
[RUSTC-TIMING] build_script_build test:false 0.383
[RUSTC-TIMING] build_script_build test:false 0.627
thread 'rustc' panicked at 'assertion failed: `(left == right)`
  left: `&str`,
 right: `str`', /rustc/43984a53f55c0c3d3071d63a0de7d5ea04454ef1/compiler/rustc_middle/src/ty/consts.rs:227:9

error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.
note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.58.0-nightly (43984a53f 2021-11-16) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z symbol-mangling-version=legacy -Z macro-backtrace -Z save-analysis -Z crate-attr=doc(html_root_url="https://doc.rust-lang.org/nightly/") -Z binary-dep-depinfo -Z force-unstable-if-unmarked -C opt-level=3 -C embed-bitcode=no -C codegen-units=1 -C debuginfo=1 -C linker=clang -C link-args=-Wl,-rpath,$ORIGIN/../lib -C prefer-dynamic -C embed-bitcode=yes --crate-type lib
note: some of the compiler flags provided by cargo are hidden

query stack during panic:
query stack during panic:
#0 [check_match] match-checking `str::traits::<impl at library/core/src/str/traits.rs:557:1: 590:2>::from_str`
#1 [analysis] running analysis passes on this crate
[RUSTC-TIMING] core test:false 30.416
error: could not compile `core`
Build completed unsuccessfully in 0:12:49
