rust
fn f(_1: [u64; 32], _2: fn([u64; 32], [u64; 32])) -> () {
    debug a => _1;                       // in scope 0 at a.rs:3:10: 3:11
    debug b => _2;                       // in scope 0 at a.rs:3:16: 3:17
    let mut _0: ();                      // return place in scope 0 at a.rs:3:35: 3:35

    bb0: {
        _0 = move _2(_1, _1) -> bb1;     // scope 0 at a.rs:4:5: 4:12
    }

    bb1: {
        return;                          // scope 0 at a.rs:5:2: 5:2
    }
}
