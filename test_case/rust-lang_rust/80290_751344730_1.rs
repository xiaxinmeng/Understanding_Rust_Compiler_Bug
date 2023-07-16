rust
fn write(_1: *mut T, _2: T) -> () {
    debug dst => _1;
    debug src => _2;
    let mut _0: ();
    let mut _3: *mut T;
    let mut _4: &mut T;

    bb0: {
        StorageLive(_4);
        _4 = &mut (*_1);
        _3 = &raw mut (*_4);
        (*_3) = move _2;
        StorageDead(_4);
        return;
    }
}
