
fn test(_1: [u32; 2]) -> () {
    debug dwords => _1;
    let mut _0: ();
    let _2: u32;
    scope 1 {
        debug _a => _2;
    }

    bb0: {
        FakeRead(ForMatchedPlace, _1);
        switchInt(_1[0 of 2]) -> [0_u32: bb1, 1_u32: bb2, otherwise: bb3];
    }

    bb1: {
        falseEdge -> [real: bb5, imaginary: bb2];
    }

    bb2: {
        falseEdge -> [real: bb6, imaginary: bb3];
    }

    bb3: {
        _0 = ();
        goto -> bb7;
    }

    bb4: {
        _0 = ();
        StorageDead(_2);
        goto -> bb7;
    }

    bb5: {
        StorageLive(_2);
        _2 = _1[1 of 2];
        goto -> bb4;
    }

    bb6: {
        StorageLive(_2);
        _2 = _1[1 of 2];
        goto -> bb4;
    }

    bb7: {
        return;
    }
}
