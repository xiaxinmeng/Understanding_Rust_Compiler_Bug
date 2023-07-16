plain
   Compiling cc v1.0.60
   Compiling core v0.0.0 (/checkout/library/core)
   Compiling libc v0.2.93
   Compiling std v0.0.0 (/checkout/library/std)
error: macros that expand to items must be delimited with braces or followed by a semicolon
   |
32 | #[macro_export]
   |  ^^^^^^^^^^^^^^
   |
   |
help: change the delimiters to curly braces
   |
32 | #{macro_export}
   |  ^            ^
help: add a semicolon
32 | #[macro_export];
   |                ^


error: expected `]`, found `#`
   |
   |
33 | #[stable(feature = "rust1", since = "1.0.0")]
   |                                             - expected `]`
34 | #[allow_internal_unstable(core_panic)]


error: expected `]`, found `macro_rules`
   |
   |
80 | #[unstable(feature = "std_internals", issue = "none")]
   |                                                      - expected `]`
81 | macro_rules! impl_fn_for_zst {


error: internal compiler error: attempted to bump the parser past EOF (may be stuck in a loop)
   |
43 |     )
   |     ^

---
note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.54.0-nightly (dd77224a8 2021-05-12) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z macro-backtrace -Z binary-dep-depinfo -Z force-unstable-if-unmarked -C opt-level=3 -C embed-bitcode=no -C codegen-units=1 -C debuginfo=0 -C debug-assertions=on -C overflow-checks=off -C link-args=-Wl,-rpath,$ORIGIN/../lib -C prefer-dynamic -C llvm-args=-import-instr-limit=10 -C embed-bitcode=yes --crate-type lib
note: some of the compiler flags provided by cargo are hidden

query stack during panic:
end of query stack
