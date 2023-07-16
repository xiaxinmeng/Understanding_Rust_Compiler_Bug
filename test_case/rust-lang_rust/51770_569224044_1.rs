
$ RUSTUP_TOOLCHAIN=nightly-2018-06-24 rustc issue-51770.rs
thread 'main' panicked at 'region_obligations not empty: [
    (
        NodeId(
            23
        ),
        RegionObligation(sub_region='_#1r, sup_type=T)
    )
]', librustc/infer/mod.rs:1057:9
note: Run with `RUST_BACKTRACE=1` for a backtrace.

error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports

note: rustc 1.28.0-nightly (60efbdead 2018-06-23) running on x86_64-apple-darwin
