`
warning: `clippy_lints` (lib) generated 3 warnings
error: internal compiler error: compiler/rustc_monomorphize/src/collector.rs:1031:9: no MIR available for DefId(1:7715 ~ std[2afd]::process::{impl#22}::new)

thread 'rustc' panicked at 'Box<dyn Any>', /rustc/0f4bcadb46006bc484dad85616b484f93879ca4e/compiler/rustc_errors/src/lib.rs:1392:9
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust-clippy/issues/new

note: Clippy version: clippy 0.1.64 (0f4bcad 2022-07-30)

query stack during panic:
#0 [collect_and_partition_mono_items] collect_and_partition_mono_items
#1 [exported_symbols] exported_symbols
end of query stack
thread 'rustc' panicked at 'Box<dyn Any>', /rustc/0f4bcadb46006bc484dad85616b484f93879ca4e/compiler/rustc_errors/src/lib.rs:1392:9

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust-clippy/issues/new

note: Clippy version: clippy 0.1.64 (0f4bcad 2022-07-30)

query stack during panic:
#0 [collect_and_partition_mono_items] collect_and_partition_mono_items
#1 [exported_symbols] exported_symbols
end of query stack
error: internal compiler error: compiler/rustc_monomorphize/src/collector.rs:1031:9: no MIR available for DefId(1:2418 ~ std[2afd]::env::var)

thread 'rustc' panicked at 'Box<dyn Any>', /rustc/0f4bcadb46006bc484dad85616b484f93879ca4e/compiler/rustc_errors/src/lib.rs:1392:9

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust-clippy/issues/new

note: Clippy version: clippy 0.1.64 (0f4bcad 2022-07-30)

query stack during panic:
#0 [collect_and_partition_mono_items] collect_and_partition_mono_items
#1 [exported_symbols] exported_symbols
end of query stack
error: internal compiler error: compiler/rustc_monomorphize/src/collector.rs:1031:9: no MIR available for DefId(2:9044 ~ core[3fba]::option::{impl#5}::clone)

thread 'rustc' panicked at 'Box<dyn Any>', /rustc/0f4bcadb46006bc484dad85616b484f93879ca4e/compiler/rustc_errors/src/lib.rs:1392:9

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust-clippy/issues/new

note: Clippy version: clippy 0.1.64 (0f4bcad 2022-07-30)

query stack during panic:
#0 [collect_and_partition_mono_items] collect_and_partition_mono_items
#1 [exported_symbols] exported_symbols
end of query stack
error: internal compiler error: compiler/rustc_monomorphize/src/collector.rs:1031:9: no MIR available for DefId(2:48537 ~ core[3fba]::fmt::{impl#3}::new_display)

thread 'rustc' panicked at 'Box<dyn Any>', /rustc/0f4bcadb46006bc484dad85616b484f93879ca4e/compiler/rustc_errors/src/lib.rs:1392:9

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust-clippy/issues/new

note: Clippy version: clippy 0.1.64 (0f4bcad 2022-07-30)

query stack during panic:
#0 [collect_and_partition_mono_items] collect_and_partition_mono_items
#1 [exported_symbols] exported_symbols
end of query stack
error: could not compile `rustc_tools_util`
Build completed unsuccessfully in 0:00:04
