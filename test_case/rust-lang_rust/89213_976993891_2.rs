
bb0: {
    StorageLive(_3);                 // scope 0 at src/main.rs:25:9: 25:20
    StorageLive(_4);                 // scope 0 at src/main.rs:25:14: 25:19
    _4 = (_1.0: u32);                // scope 0 at src/main.rs:25:14: 25:19
    _3 = std::mem::drop::<u32>(move _4) -> [return: bb1, unwind: bb4]; // scope 0 at src/main.rs:25:9: 25:20
                                     // mir::Constant
                                     // + span: src/main.rs:25:9: 25:13
                                     // + literal: Const { ty: fn(u32) {std::mem::drop::<u32>}, val: Value(Scalar(<ZST>)) }
}
