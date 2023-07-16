rust
layout_of(FixedResult<[u64; 0], Packed<std::num::NonZeroU16>>) = Layout {
    // ...
    variants: Multiple {
        tag: Initialized {
            value: Int(I8, false),
            valid_range: 0..=1,
        },
        tag_encoding: Direct,
        // ...
    },
    // ...
    align: AbiAndPrefAlign {
        abi: Align(8 bytes),
        pref: Align(8 bytes),
    },
    size: Size(8 bytes),
}
layout_of(FixedResult<[u64; 0], Packed<U16IsZero>>) = Layout {
    // ...
    variants: Multiple {
        tag: Initialized {
            value: Int(I16, false),
            valid_range: 0..=1,
        },
        tag_encoding: Niche {
            dataful_variant: 1,
            niche_variants: 0..=0,
            niche_start: 1,
        },
        // ..
    },
    // ...
    align: AbiAndPrefAlign {
        abi: Align(8 bytes),
        pref: Align(8 bytes),
    },
    size: Size(8 bytes),
}
