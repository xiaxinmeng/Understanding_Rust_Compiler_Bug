
thread 'main' panicked at 'region_obligations not empty: [
    (
        NodeId(
            3805
        ),
        RegionObligation(sub_region=ReStatic, sup_type=U)
    ),
    (
        NodeId(
            3805
        ),
        RegionObligation(sub_region=ReStatic, sup_type=U)
    )
]', librustc/infer/mod.rs:1057:9
note: Run with `RUST_BACKTRACE=1` for a backtrace.

error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports

note: rustc 1.28.0-nightly (757cd050f 2018-06-09) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z borrowck=mir -C debuginfo=2 --crate-type lib

note: some of the compiler flags provided by cargo are hidden
