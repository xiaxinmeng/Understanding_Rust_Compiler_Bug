
fn f() -> usize {
    let mut _0: usize;                   // return place in scope 0 at ./example.rs:2:15: 2:20
    let mut _1: usize;                   // in scope 0 at ./example.rs:3:9: 3:33

    bb0: {
        StorageLive(_1);                 // scope 0 at ./example.rs:3:9: 3:33
        _1 = const 1_usize;              // scope 0 at ./example.rs:3:19: 3:20
        _0 = Add(const 1_usize, move _1); // scope 0 at ./example.rs:3:5: 3:33
        StorageDead(_1);                 // scope 0 at ./example.rs:3:32: 3:33
        return;                          // scope 0 at ./example.rs:4:2: 4:2
    }
}
