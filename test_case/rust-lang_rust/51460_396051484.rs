
thread 'main' panicked at 'region_obligations not empty: [
    (
        NodeId(
            18904
        ),
        RegionObligation(sub_region='_#16r, sup_type=M)
    ),
    (
        NodeId(
            18904
        ),
        RegionObligation(sub_region='_#17r, sup_type=M)
    ),
    (
        NodeId(
            18904
        ),
        RegionObligation(sub_region='_#18r, sup_type=M)
    ),
    (
        NodeId(
            18904
        ),
        RegionObligation(sub_region='_#19r, sup_type=M)
    ),
    (
        NodeId(
            18904
        ),
        RegionObligation(sub_region='_#20r, sup_type=M)
    ),
    (
        NodeId(
            18904
        ),
        RegionObligation(sub_region='_#21r, sup_type=M)
    ),
    (
        NodeId(
            18904
        ),
        RegionObligation(sub_region='_#22r, sup_type=M)
    ),
    (
        NodeId(
            18904
        ),
        RegionObligation(sub_region='_#23r, sup_type=M)
    ),
    (
        NodeId(
            18904
        ),
        RegionObligation(sub_region='_#24r, sup_type=M)
    )
]', librustc/infer/mod.rs:1057:9
note: Run with `RUST_BACKTRACE=1` for a backtrace.

error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports

note: rustc 1.28.0-nightly (757cd050f 2018-06-09) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z borrowck=mir -C debuginfo=2 --crate-type lib

note: some of the compiler flags provided by cargo are hidden

thread 'main' panicked at 'assertion failed: cmd.status().expect("failed to spawn").success()', collector/src/bin/rustc-fake.rs:33:17
