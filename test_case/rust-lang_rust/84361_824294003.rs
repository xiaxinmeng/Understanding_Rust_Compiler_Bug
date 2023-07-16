
    bb0: {
        StorageLive(_2);                 // this is the variable we'll be matching on, the function result
        _3 = &mut (*_1);
        _2 = Test::test(move _3) -> bb1;
    }

    bb1: {
        _4 = discriminant(_2);
        switchInt(move _4) -> [0_isize: bb3, 1_isize: bb4, otherwise: unreachable];
    }

    bb3: {
        // The None branch
        discriminant(_0) = 0;
        goto -> bb8;
    }

    bb4: {
        _6 = &((_2 as Some).0: &str);
        _8 = (*_6);
        _7 = core::str::<impl str>::len(move _8) -> bb5;
    }

    bb5: {
        switchInt(move _7) -> [1_usize: bb6, otherwise: bb7];
    }

    bb6: {
        // Some(_) if ... branch
        _5 = ((_2 as Some).0: &str);
        _9 = _5;
        ((_0 as Some).0: &str) = move _9;
        discriminant(_0) = 1;
        goto -> bb8;
    }

    bb7: {
        // this is the last Some(_) branch...
        goto -> bb8;
    }

    bb8: {
        StorageDead(_2);                 // matched variable finally dead here
        return;
    }
