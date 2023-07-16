
fn test(_1: [u32; 2]) -> () {
    debug dwords => _1;
    let mut _0: ();
    let _2: u32;
    scope 1 {
        debug _a => _2;
    }

    bb0: {
        switchInt(_1[0 of 2]) -> [0_u32: bb2, 1_u32: bb3, otherwise: bb4];
    }

    bb1: {
        StorageDead(_2);
        goto -> bb4;
    }

    bb2: {
        StorageLive(_2);
        _2 = _1[1 of 2];
        goto -> bb1;
    }

    bb3: {
        StorageLive(_2);
        _2 = _1[1 of 2];
        goto -> bb1;
    }

    bb4: {
        return;
    }
}
