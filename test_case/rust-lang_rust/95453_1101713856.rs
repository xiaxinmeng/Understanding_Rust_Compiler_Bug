
error: internal compiler error: /rustc/ec77f252434a532fdb5699ae4f21a3072d211edd/compiler/rustc_const_eval/src/interpret/place.rs:775:68: write_immediate_to_mplace: invalid ScalarPair layout: TyAndLayout {
                                    ty: *mut &[i32; 2],
                                    layout: Layout {
                                        fields: Primitive,
                                        variants: Single {
                                            index: 0,
                                        },
                                        abi: Scalar(
                                            Initialized {
                                                value: Pointer,
                                                valid_range: 0..=18446744073709551615,
                                            },
                                        ),
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
                                            raw: 8,
                                        },
                                    },
                                }
   --> /home/autumn/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/alloc/src/boxed.rs:363:13
    |
363 |             boxed.as_mut_ptr().write(x);
    |             ^^^^^^^^^^^^^^^^^^
