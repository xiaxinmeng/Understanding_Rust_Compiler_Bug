rust
fn <impl at opti.rs:8:1: 17:2>::mut_update(_1: &mut Type, _2: u32) -> &mut Type {
    let mut _0: &mut Type;               // return pointer
    scope 1 {
        let _3: &mut Type;               // "self" in scope 1 at opti.rs:9:19: 9:28
    }
    let mut _4: &mut Type;

    bb0: {
        StorageLive(_3);                 // scope 0 at opti.rs:9:19: 9:28
        _3 = _1;                         // scope 0 at opti.rs:9:19: 9:28
        nop;                             // scope 0 at opti.rs:9:30: 9:36
        nop;                             // scope 0 at opti.rs:9:30: 9:36
        StorageLive(_4);                 // scope 1 at opti.rs:9:57: 12:6
        nop;                             // scope 1 at opti.rs:10:23: 10:29
        nop;                             // scope 1 at opti.rs:10:23: 10:29
        ((*_3).1: u32) = _2;             // scope 1 at opti.rs:10:9: 10:29
        nop;                             // scope 1 at opti.rs:10:29: 10:29
        _4 = _3;                         // scope 1 at opti.rs:11:9: 11:13
        _0 = _4;                         // scope 1 at opti.rs:9:57: 12:6
        StorageDead(_4);                 // scope 1 at opti.rs:12:6: 12:6
        nop;                             // scope 0 at opti.rs:12:6: 12:6
        StorageDead(_3);                 // scope 0 at opti.rs:12:6: 12:6
        return;                          // scope 1 at opti.rs:12:6: 12:6
    }
}

fn <impl at opti.rs:8:1: 17:2>::func_update(_1: Type, _2: u32) -> Type {
    let mut _0: Type;                    // return pointer
    scope 1 {
    }

    bb0: {
        nop;                             // scope 0 at opti.rs:14:20: 14:24
        nop;                             // scope 0 at opti.rs:14:20: 14:24
        nop;                             // scope 0 at opti.rs:14:26: 14:32
        nop;                             // scope 0 at opti.rs:14:26: 14:32
        nop;                             // scope 1 at opti.rs:15:25: 15:31
        nop;                             // scope 1 at opti.rs:15:25: 15:31
        (_0.0: u64) = (_1.0: u64);       // scope 1 at opti.rs:15:9: 15:42
        (_0.1: u32) = _2;                // scope 1 at opti.rs:15:9: 15:42
        nop;                             // scope 1 at opti.rs:15:42: 15:42
        nop;                             // scope 0 at opti.rs:16:6: 16:6
        nop;                             // scope 0 at opti.rs:16:6: 16:6
        return;                          // scope 1 at opti.rs:16:6: 16:6
    }
}
