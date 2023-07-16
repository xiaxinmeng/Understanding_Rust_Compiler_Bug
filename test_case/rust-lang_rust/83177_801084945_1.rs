console
$ rustc --crate-type=lib z.rs -Zunpretty=mir
// WARNING: This output format is intended for human consumers only
// and is subject to change without notice. Knock yourself out.
fn f() -> () {
    let mut _0: ();                      // return place in scope 0 at z.rs:1:18: 1:18

    bb0: {
        return;                          // scope 0 at z.rs:3:2: 3:2
    }
}

// MIR FOR CTFE
fn f() -> () {
    let mut _0: ();                      // return place in scope 0 at z.rs:1:18: 1:18
    let _1: i32;                         // in scope 0 at z.rs:2:5: 2:6

    bb0: {
        StorageLive(_1);                 // scope 0 at z.rs:2:5: 2:6
        _1 = const 1_i32;                // scope 0 at z.rs:2:5: 2:6
        StorageDead(_1);                 // scope 0 at z.rs:2:6: 2:7
        _0 = const ();                   // scope 0 at z.rs:1:18: 3:2
        return;                          // scope 0 at z.rs:3:2: 3:2
    }
}
