
optimization-fuel-exhausted: Reorder fields of "ty::context::TyCtxt"
thread 'rustc' panicked at 'nonscalar layout for layout_scalar_valid_range type middle::region::FirstStatementIndex: LayoutDetails {
    variants: Single {
        index: 0,
    },
    fields: Arbitrary {
        offsets: [
            Size {
                raw: 0,
            },
        ],
        memory_index: [
            0,
        ],
    },
    abi: Aggregate {
        sized: true,
    },
    largest_niche: None,
    align: AbiAndPrefAlign {
        abi: Align {
            pow2: 2,
        },
        pref: Align {
            pow2: 3,
        },
    },
    size: Size {
        raw: 4,
    },
}', src/librustc/ty/layout.rs:855:30
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace

error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports

note: rustc 1.42.0-beta.5 (4e1c5f0e9 2020-02-28) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z fuel=rustc=0 -Z external-macro-backtrace -Z unstable-options -Z binary-dep-depinfo -Z force-unstable-if-unmarked -C opt-level=3 -C debuginfo=2 -C link-args=-Wl,-rpath,$ORIGIN/../lib -C prefer-dynamic -C debug-assertions=y --crate-type lib

note: some of the compiler flags provided by cargo are hidden

error: could not compile `rustc`.
warning: build failed, waiting for other jobs to finish...
error: build failed
