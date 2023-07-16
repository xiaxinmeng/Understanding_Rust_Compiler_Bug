rust
fn <impl at opti.rs:10:1: 23:2>::mut_update(_1: &mut Type, _2: u32) -> &mut Type {
    let mut _0: &mut Type;               // return pointer
    scope 1 {
        let _3: &mut Type;               // "self" in scope 1 at opti.rs:13:19: 13:28
        let _4: u32;                     // "field2" in scope 1 at opti.rs:13:30: 13:36
    }
    let mut _5: &mut Type;
    let mut _6: u32;

    bb0: {
        StorageLive(_3);                 // scope 0 at opti.rs:13:19: 13:28
        _3 = _1;                         // scope 0 at opti.rs:13:19: 13:28
        StorageLive(_4);                 // scope 0 at opti.rs:13:30: 13:36
        _4 = _2;                         // scope 0 at opti.rs:13:30: 13:36
        StorageLive(_5);                 // scope 1 at opti.rs:13:57: 16:6
        StorageLive(_6);                 // scope 1 at opti.rs:14:23: 14:29
        _6 = _4;                         // scope 1 at opti.rs:14:23: 14:29
        ((*_3).1: u32) = _6;             // scope 1 at opti.rs:14:9: 14:29
        StorageDead(_6);                 // scope 1 at opti.rs:14:29: 14:29
        _5 = _3;                         // scope 1 at opti.rs:15:9: 15:13
        _0 = _5;                         // scope 1 at opti.rs:13:57: 16:6
        StorageDead(_5);                 // scope 1 at opti.rs:16:6: 16:6
        StorageDead(_4);                 // scope 0 at opti.rs:16:6: 16:6
        StorageDead(_3);                 // scope 0 at opti.rs:16:6: 16:6
        return;                          // scope 1 at opti.rs:16:6: 16:6
    }
}

fn <impl at opti.rs:10:1: 23:2>::func_update(_1: Type, _2: u32) -> Type {
    let mut _0: Type;                    // return pointer
    scope 1 {
        let _3: Type;                    // "self" in scope 1 at opti.rs:20:20: 20:24
        let _4: u32;                     // "field2" in scope 1 at opti.rs:20:26: 20:32
    }
    let mut _5: u32;

    bb0: {
        StorageLive(_3);                 // scope 0 at opti.rs:20:20: 20:24
        _3 = _1;                         // scope 0 at opti.rs:20:20: 20:24
        StorageLive(_4);                 // scope 0 at opti.rs:20:26: 20:32
        _4 = _2;                         // scope 0 at opti.rs:20:26: 20:32
        StorageLive(_5);                 // scope 1 at opti.rs:21:25: 21:31
        _5 = _4;                         // scope 1 at opti.rs:21:25: 21:31
        _0 = Type { field1: (_3.0: u64), field2: _5 }; // scope 1 at opti.rs:21:9: 21:42
        StorageDead(_5);                 // scope 1 at opti.rs:21:42: 21:42
        StorageDead(_4);                 // scope 0 at opti.rs:22:6: 22:6
        StorageDead(_3);                 // scope 0 at opti.rs:22:6: 22:6
        return;                          // scope 1 at opti.rs:22:6: 22:6
    }
}
