
fn cmp(_1: &u8, _2: &u8) -> bool {
    debug x => _1;
    debug y => _2;
    let mut _0: bool;
    let mut _3: &&u8;
    let mut _4: &&u8;
    let _5: &u8;
    scope 1 (inlined cmp::impls::<impl PartialOrd for &u8>::lt) {
        debug self => _3;
        debug other => _4;
        let mut _6: &u8;
        let mut _7: &u8;
        scope 2 (inlined cmp::impls::<impl PartialOrd for u8>::lt) {
            debug self => _6;
            debug other => _7;
            let mut _8: u8;
            let mut _9: u8;
        }
    }

    bb0: {
        _3 = &_1;
        _5 = _2;
        _4 = &_5;
        _6 = deref_copy (*_3);
        _7 = deref_copy (*_4);
        _8 = (*_6);
        _9 = (*_7);
        _0 = Lt(move _8, move _9);
        return;
    }
}
