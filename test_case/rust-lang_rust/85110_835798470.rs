plain
   Compiling parking_lot v0.11.1
   Compiling rand_core v0.6.2
   Compiling cstr v0.2.8
   Compiling regex-automata v0.1.9
error: internal compiler error: /checkout/compiler/rustc_codegen_ssa/src/mir/block.rs:645:33: shuffle indices must be constant
   --> /checkout/library/core/src/../../stdarch/crates/core_arch/src/x86/avx2.rs:298:15
298 |           15 => simd_shuffle32(
    |  _______________^
299 | |             b,
300 | |             a,
---
note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.54.0-nightly (c5c405811 2021-05-09) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z macro-backtrace -Z tls-model=initial-exec -Z unstable-options -Z binary-dep-depinfo -Z force-unstable-if-unmarked -C opt-level=3 -C embed-bitcode=no -C debuginfo=0 -C debug-assertions=on -C overflow-checks=off -C link-args=-Wl,-rpath,$ORIGIN/../lib -C prefer-dynamic -C llvm-args=-import-instr-limit=10 --crate-type lib
note: some of the compiler flags provided by cargo are hidden

query stack during panic:
end of query stack
