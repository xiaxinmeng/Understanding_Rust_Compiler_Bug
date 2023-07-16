
error: internal compiler error: compiler/rustc_metadata/src/rmeta/decoder.rs:900:18: get_adt_def called on a non-ADT DefId(33:288 ~ syntax[8264]::ptr::SyntaxNodePtr)

thread 'rustc' panicked at 'Box<dyn Any>', compiler/rustc_errors/src/lib.rs:1146:9
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.57.0 (f1edd0429 2021-11-29) running on x86_64-unknown-linux-gnu

note: compiler flags: -C opt-level=3 -C embed-bitcode=no -C debuginfo=2 -C incremental --crate-type lib

note: some of the compiler flags provided by cargo are hidden

query stack during panic:
#0 [adt_def] computing ADT definition for `syntax::ptr::SyntaxNodePtr`
#1 [analysis] running analysis passes on this crate
end of query stack
error: could not compile `ide_diagnostics`
