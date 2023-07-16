plain
[RUSTC-TIMING] build_script_build test:false 0.300
[RUSTC-TIMING] build_script_build test:false 0.559
   Compiling rustc-std-workspace-core v1.99.0 (/checkout/library/rustc-std-workspace-core)
[RUSTC-TIMING] rustc_std_workspace_core test:false 0.029
thread 'rustc' panicked at 'attempt to add with overflow', /checkout/compiler/rustc_target/src/abi/mod.rs:795:56

error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.
note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.57.0-nightly (005d024ed 2021-09-08) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z macro-backtrace -Z crate-attr=doc(html_root_url="https://doc.rust-lang.org/nightly/") -Z binary-dep-depinfo -Z force-unstable-if-unmarked -C opt-level=3 -C embed-bitcode=no -C codegen-units=1 -C debuginfo=1 -C debug-assertions=on -C linker=clang -C link-args=-Wl,-rpath,$ORIGIN/../lib -C prefer-dynamic -C embed-bitcode=yes --crate-type lib
note: some of the compiler flags provided by cargo are hidden

query stack during panic:
end of query stack
