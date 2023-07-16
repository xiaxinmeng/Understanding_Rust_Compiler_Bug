 
error: internal compiler error: compiler\rustc_const_eval\src\interpret\operand.rs:438:18:
    invalid field access on immediate (0x0001, 0x00): (u16, bool), layout TyAndLayout {
...
                                        fields: Arbitrary {
                                            offsets: [
                                                Size(2 bytes),
                                                Size(3 bytes),
                                            ],
...
                                        abi: Aggregate {
                                            sized: true,
                                        },
...
