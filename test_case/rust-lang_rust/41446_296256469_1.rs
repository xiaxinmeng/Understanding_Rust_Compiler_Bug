
fn foo(_1: !) -> u32 {
    let mut _0: u32;                     // return pointer
    scope 1 {
        let _2: !;                       // "x" in scope 1 at <anon>:2:8: 2:9
    }
    let mut _3: !;

    bb0: {
        StorageLive(_2);                 // scope 0 at <anon>:2:8: 2:9
        _2 = _1;                         // scope 0 at <anon>:2:8: 2:9
        _3 = _2;                         // scope 1 at <anon>:2:23: 2:24
        unreachable;                     // scope 1 at <anon>:2:23: 2:24
    }
}
