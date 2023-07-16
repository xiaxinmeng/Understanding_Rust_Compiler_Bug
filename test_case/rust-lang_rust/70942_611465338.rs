
error[E0326]: implemented const `VALUE` has an incompatible type for trait
 --> src\main.rs:8:18
  |
2 |     const VALUE: usize;
  |                  ----- type in trait
...
8 |     const VALUE: i32 = 0;
  |                  ^^^ expected usize, found i32

thread 'rustc' panicked at 'Layout mismatch when copying!
src: OpTy {
    op: Indirect(
        MemPlace {
            ptr: Ptr(
                Pointer {
                    alloc_id: AllocId(
                        1,
                    ),
                    offset: Size {
                        raw: 0,
                    },
                    tag: (),
                },
            ),
            align: Align {
                pow2: 2,
            },
            meta: None,
        },
    ),
    layout: TyLayout {
        ty: i32,
        details: LayoutDetails {
            variants: Single {
                index: 0,
            },
            fields: Union(
                0,
            ),
            abi: Scalar(
                Scalar {
                    value: Int(
                        I32,
                        true,
                    ),
                    valid_range: 0..=4294967295,
                },
            ),
            align: AbiAndPrefAlign {
                abi: Align {
                    pow2: 2,
                },
                pref: Align {
                    pow2: 2,
                },
            },
            size: Size {
                raw: 4,
            },
        },
    },
}
dest: PlaceTy {
    place: Ptr(
        MemPlace {
            ptr: Ptr(
                Pointer {
                    alloc_id: AllocId(
                        0,
                    ),
                    offset: Size {
                        raw: 0,
                    },
                    tag: (),
                },
            ),
            align: Align {
                pow2: 3,
            },
            meta: None,
        },
    ),
    layout: TyLayout {
        ty: usize,
        details: LayoutDetails {
            variants: Single {
                index: 0,
            },
            fields: Union(
                0,
            ),
            abi: Scalar(
                Scalar {
                    value: Int(
                        I64,
                        false,
                    ),
                    valid_range: 0..=18446744073709551615,
                },
            ),
            align: AbiAndPrefAlign {
                abi: Align {
                    pow2: 3,
                },
                pref: Align {
                    pow2: 3,
                },
            },
            size: Size {
                raw: 8,
            },
        },
    },
}', src\librustc_mir\interpret\place.rs:817:9
note: Run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
error: aborting due to previous error

For more information about this error, try `rustc --explain E0326`.

error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports

note: rustc 1.36.0-nightly (e938c2b9a 2019-04-23) running on x86_64-pc-windows-msvc

note: compiler flags: -C debuginfo=2 -C incremental --crate-type bin

note: some of the compiler flags provided by cargo are hidden
