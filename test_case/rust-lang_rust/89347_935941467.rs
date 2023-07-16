
------------------------------------------
stderr:
------------------------------------------
thread 'rustc' panicked at '`ModuleData::def_id` is called on a block module', compiler/rustc_resolve/src/lib.rs:607:27
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace

error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.57.0-nightly (86650600b 2021-10-06) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z threads=1 -Z ui-testing -Z deduplicate-diagnostics=no -Z emit-future-incompat-report -Z save-analysis -C codegen-units=1 -C prefer-dynamic -C rpath -C debuginfo=0

query stack during panic:
end of query stack

------------------------------------------



failures:
    [ui] ui/proc-macro/weird-hygiene.rs
    [ui] ui/save-analysis/issue-59134-0.rs
    [ui] ui/save-analysis/issue-59134-1.rs
