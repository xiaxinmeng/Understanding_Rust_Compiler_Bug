rust
fn <impl at opti.rs:10:1: 23:2>::mut_update(_1: &mut Type, _2: u32) -> &mut Type {
    let mut _0: &mut Type;               // return pointer

    bb0: {
        ((*_1).1: u32) = _2;             // scope 1 at opti.rs:14:9: 14:29
        _0 = _1;                         // scope 1 at opti.rs:13:57: 16:6
        return;                          // scope 1 at opti.rs:16:6: 16:6
    }
}

fn <impl at opti.rs:10:1: 23:2>::func_update(_1: Type, _2: u32) -> Type {
    let mut _0: Type;                    // return pointer

    bb0: {
        _0 = Type { field1: (_1.0: u64), field2: _2 }; // scope 1 at opti.rs:21:9: 21:42
        return;                          // scope 1 at opti.rs:22:6: 22:6
    }
}
