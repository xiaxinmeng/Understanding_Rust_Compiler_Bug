plain
   Compiling term v0.0.0 (/checkout/library/term)
   Compiling proc_macro v0.0.0 (/checkout/library/proc_macro)
   Compiling unicode-width v0.1.8
   Compiling getopts v0.2.21
thread 'rustc' panicked at 'assertion failed: output.ends_with(\'>\')', compiler/rustc_codegen_ssa/src/debuginfo/type_names.rs:591:5

error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.
note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.55.0-nightly (4ae522214 2021-07-15) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z macro-backtrace -Z save-analysis -Z crate-attr=doc(html_root_url="https://doc.rust-lang.org/nightly/") -Z binary-dep-depinfo -Z force-unstable-if-unmarked -C opt-level=3 -C embed-bitcode=no -C codegen-units=1 -C debuginfo=1 -C linker=clang -C link-args=-Wl,-rpath,$ORIGIN/../lib -C prefer-dynamic -C embed-bitcode=yes --crate-type lib
note: some of the compiler flags provided by cargo are hidden

query stack during panic:
end of query stack
end of query stack
error: could not compile `getopts`

To learn more, run the command again with --verbose.
warning: build failed, waiting for other jobs to finish...
thread 'rustc' panicked at 'assertion failed: output.ends_with(\'>\')', compiler/rustc_codegen_ssa/src/debuginfo/type_names.rs:591:5

error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.
note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.55.0-nightly (4ae522214 2021-07-15) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z macro-backtrace -Z save-analysis -Z crate-attr=doc(html_root_url="https://doc.rust-lang.org/nightly/") -Z binary-dep-depinfo -Z force-unstable-if-unmarked -C opt-level=3 -C embed-bitcode=no -C codegen-units=1 -C debuginfo=1 -C linker=clang -C link-args=-Wl,-rpath,$ORIGIN/../lib -C prefer-dynamic -C embed-bitcode=yes --crate-type lib
note: some of the compiler flags provided by cargo are hidden

query stack during panic:
end of query stack
