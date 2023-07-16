
fn bar(_1: Void) -> usize {
    debug v => _1;                       // in scope 0 at t.rs:3:12: 3:13
    let mut _0: usize;                   // return place in scope 0 at t.rs:3:24: 3:29
    let mut _2: Void;                    // in scope 0 at t.rs:4:5: 4:6

    bb0: {
        StorageLive(_2);                 // scope 0 at t.rs:4:5: 4:6
        _2 = move _1;                    // scope 0 at t.rs:4:5: 4:6
        _0 = move _2 as usize (Misc);    // scope 0 at t.rs:4:5: 4:15
        StorageDead(_2);                 // scope 0 at t.rs:4:14: 4:15
        return;                          // scope 0 at t.rs:5:2: 5:2
    }
}
