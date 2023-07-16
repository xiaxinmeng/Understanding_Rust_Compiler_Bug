rust
fn  e220::get_displacement() -> [i64; 2] {
    let mut _0: [i64; 2];

    bb0: {
        _0 = [const 139776i64, const 963904i64];
        return;
    }
}

fn  e220() -> (i64, i64) {
    let mut _0: (i64, i64);
    let _1: [i64; 2];
    let mut _2: (&i64, &i64);
    let mut _3: &i64;
    let _4: usize;
    let mut _5: &i64;
    let _6: usize;
    let mut _9: i64;
    let mut _10: i64;
    scope 1 {
        debug res => _1;
        let _7: &i64;
        let _8: &i64;
        scope 2 {
            debug arg0 => _7;
            debug arg1 => _8;
        }
    }

    bb0: {
        StorageLive(_1);
        _1 = const e220::get_displacement() -> bb1;
    }

    bb1: {
        StorageLive(_2);
        StorageLive(_3);
        _4 = const 0usize;
        _3 = &_1[_4];
        StorageLive(_5);
        StorageLive(_6);
        _6 = const 1usize;
        _5 = &_1[_6];
        (_2.0: &i64) = const Scalar(AllocId(15).0x0) : &i64;
        (_2.1: &i64) = const Scalar(AllocId(15).0x8) : &i64;
        StorageDead(_5);
        StorageDead(_3);
        StorageLive(_7);
        _7 = (_2.0: &i64);
        StorageLive(_8);
        _8 = (_2.1: &i64);
        StorageLive(_9);
        _9 = (*_7);
        StorageLive(_10);
        _10 = (*_8);
        (_0.0: i64) = move _9;
        (_0.1: i64) = move _10;
        StorageDead(_10);
        StorageDead(_9);
        StorageDead(_8);
        StorageDead(_7);
        StorageDead(_1);
        StorageDead(_6);
        StorageDead(_2);
        return;
    }
}
