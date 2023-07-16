
thread 'rustc' panicked at 'range start index 1 out of range for slice of length 0', library\core\src\slice\index.rs:52:5
stack backtrace:
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.

error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.65.0-nightly (9ba169a73 2022-09-02) running on x86_64-pc-windows-msvc

note: compiler flags: --crate-type dylib --crate-type rlib -C opt-level=3 -C embed-bitcode=no -C debuginfo=0 -C debug-assertions=on -C debug-assertions=off -C overflow-checks=on -Z force-unstable-if-unmarked -Z tls-model=initial-exec

note: some of the compiler flags provided by cargo are hidden

query stack during panic:
#0 [mir_drops_elaborated_and_const_checked] elaborating drops for `sys::windows::fs::<impl at D:\a\rust\rust\library\std\src\sys\windows\fs.rs:278:1: 278:10>::file_attr`
#1 [optimized_mir] optimizing MIR for `sys::windows::fs::<impl at D:\a\rust\rust\library\std\src\sys\windows\fs.rs:278:1: 278:10>::file_attr`
end of query stack
[RUSTC-TIMING] std test:false 5.705
error: could not compile `std`
