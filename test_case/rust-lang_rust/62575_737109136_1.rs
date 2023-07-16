
error: layout_of(std::future::from_generator::GenFuture<[static generator@src/lib.rs:9:11: 9:13 {}]>) = Layout {
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
    variants: Single {
        index: 0,
    },
    abi: Aggregate {
        sized: true,
    },
    largest_niche: Some(
        Niche {
            offset: Size {
                raw: 0,
            },
            scalar: Scalar {
                value: Int(
                    I8,
                    false,
                ),
                valid_range: 0..=2,
            },
        },
    ),
    align: AbiAndPrefAlign {
        abi: Align {
            pow2: 0,
        },
        pref: Align {
            pow2: 3,
        },
    },
    size: Size {
        raw: 1,
    },
}
