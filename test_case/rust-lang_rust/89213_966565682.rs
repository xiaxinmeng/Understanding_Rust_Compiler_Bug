
bb0: {
    StorageLive(_3);                 // scope 0 at src/main.rs:37:22: 37:32
    StorageLive(_4);                 // scope 0 at src/main.rs:37:22: 37:32
    StorageLive(_5);                 // scope 0 at src/main.rs:37:22: 37:27
    _5 = (_1.0: u8);                 // scope 0 at src/main.rs:37:22: 37:27
    StorageLive(_6);                 // scope 0 at src/main.rs:37:29: 37:32
    _6 = (_1.1: u8);                 // scope 0 at src/main.rs:37:29: 37:32
    _4 = std::ops::Range::<u8> { start: move _5, end: move _6 }; // scope 0 at src/main.rs:37:22: 37:32
    StorageDead(_6);                 // scope 0 at src/main.rs:37:31: 37:32
    StorageDead(_5);                 // scope 0 at src/main.rs:37:31: 37:32
    _3 = <std::ops::Range<u8> as IntoIterator>::into_iter(move _4) -> [return: bb1, unwind: bb18]; // scope 0 at src/main.rs:37:22: 37:32
                                     // mir::Constant
                                     // + span: src/main.rs:37:22: 37:32
                                     // + literal: Const { ty: fn(std::ops::Range<u8>) -> <std::ops::Range<u8> as std::iter::IntoIterator>::IntoIter {<std::ops::Range<u8> as std::iter::IntoIterator>::into_iter}, val: Value(Scalar(<ZST>)) }
}
