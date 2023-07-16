rust
[...]
    bb0: {
        _2 = &(promoted[0]: [u8; 4]);
        _1 = move _2 as &[u8] (Pointer(Unsize));
        _6 = &(promoted[0]: [u8; 4]);
        _3 = move _6 as &[u8] (Pointer(Unsize));
        StorageLive(_4);
        _4 = const assert_1234(move _3) -> bb1;
    }

    bb1: {
        StorageDead(_4);
        StorageLive(_5);
        _5 = const assert_0000(move _1) -> bb2;
    }
[...]
