
// MIR for `main::{closure#0}` before SimplifyCfg-early-opt

fn main::{closure#0}(_1: [generator@src/main.rs:8:17: 11:6], _2: ()) -> ()
yields ()
 {
    debug a => (*(_1.0: &mut i32));      // in scope 0 at src/main.rs:7:9: 7:14
    let mut _0: ();                      // return place in scope 0 at src/main.rs:8:20: 8:20
    let _3: ();                          // in scope 0 at src/main.rs:9:9: 9:14
    let mut _4: ();                      // in scope 0 at src/main.rs:9:9: 9:14

    bb0: {
        StorageLive(_3);                 // scope 0 at src/main.rs:9:9: 9:14
        StorageLive(_4);                 // scope 0 at src/main.rs:9:9: 9:14
        _4 = ();                         // scope 0 at src/main.rs:9:9: 9:14
        _3 = yield(move _4) -> [resume: bb1, drop: bb2]; // scope 0 at src/main.rs:9:9: 9:14
    }

    bb1: {
        StorageDead(_4);                 // scope 0 at src/main.rs:9:13: 9:14
        StorageDead(_3);                 // scope 0 at src/main.rs:9:14: 9:15
        (*(_1.0: &mut i32)) = const 1_i32; // scope 0 at src/main.rs:10:9: 10:14
        _0 = const ();                   // scope 0 at src/main.rs:8:20: 11:6
        return;                          // scope 0 at src/main.rs:11:6: 11:6
    }

    bb2: {
        StorageDead(_4);                 // scope 0 at src/main.rs:9:13: 9:14
        StorageDead(_3);                 // scope 0 at src/main.rs:9:14: 9:15
        generator_drop;                  // scope 0 at src/main.rs:8:17: 11:6
    }

    bb3 (cleanup): {
        resume;                          // scope 0 at src/main.rs:8:17: 11:6
    }
}
