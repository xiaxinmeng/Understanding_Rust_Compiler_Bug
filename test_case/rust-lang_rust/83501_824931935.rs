
error: layout_of(X) = Layout {
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
    variants: Multiple {
        tag: Scalar {
            value: Int(
                I8,
                false,
            ),
            valid_range: 0..=1,
        },
        tag_encoding: Direct,
        tag_field: 0,
        variants: [
            Layout {
                fields: Arbitrary {
                    offsets: [
                        Size {
                            raw: 1,
                        },
                        Size {
                            raw: 2,
                        },
                        Size {
                            raw: 3,
                        },
                    ],
                    memory_index: [
                        0,
                        1,
                        2,
                    ],
                },
                variants: Single {
                    index: 0,
                },
                abi: Aggregate {
                    sized: true,
                },
                largest_niche: None,
                align: AbiAndPrefAlign {
                    abi: Align {
                        pow2: 0,
                    },
                    pref: Align {
                        pow2: 3,
                    },
                },
                size: Size {
                    raw: 4,
                },
            },
            Layout {
                fields: Arbitrary {
                    offsets: [
                        Size {
                            raw: 8,
                        },
                    ],
                    memory_index: [
                        0,
                    ],
                },
                variants: Single {
                    index: 1,
                },
                abi: Aggregate {
                    sized: true,
                },
                largest_niche: None,
                align: AbiAndPrefAlign {
                    abi: Align {
                        pow2: 3,
                    },
                    pref: Align {
                        pow2: 3,
                    },
                },
                size: Size {
                    raw: 16,
                },
            },
        ],
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
                valid_range: 0..=1,
            },
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
        raw: 16,
    },
}
