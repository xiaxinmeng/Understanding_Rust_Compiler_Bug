rust
fn write(_1: *mut T, _2: T) -> () {
    debug dst => _1;
    debug src => _2;
    let mut _0: ();
    let _3: ();
    let mut _4: *const T;
    let _5: &T;
    let mut _6: *mut T;
    let mut _7: bool;

    bb0: {
        _7 = const false;
        _7 = const true;
        StorageLive(_3);
        StorageLive(_4);
        StorageLive(_5);
        _5 = &_2;
        _4 = &raw const (*_5);
        StorageLive(_6);
        _6 = _1;
        _3 = copy_nonoverlapping::<T>(move _4, move _6, const 1_usize) -> [return: bb1, unwind: bb4];

    bb1: {
        StorageDead(_6);
        StorageDead(_4);
        StorageDead(_5);
        StorageDead(_3);
        _7 = const false;
        _0 = const ();
        return;
    }

    bb2 (cleanup): {
        resume;
    }

    bb3 (cleanup): {
        drop(_2) -> bb2;
    }

    bb4 (cleanup): {
        switchInt(_7) -> [false: bb2, otherwise: bb3];
    }
}
