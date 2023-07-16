mir
// MIR for `main` after PreCodegen

fn main() -> () {
    let mut _0: ();                      // return place in scope 0 at 27925-reduced.rs:1:15: 1:15
    let _1: [closure@27925-reduced.rs:2:13: 2:18]; // in scope 0 at 27925-reduced.rs:2:9: 2:10
    scope 1 {
        debug c => _1;                   // in scope 1 at 27925-reduced.rs:2:9: 2:10
        scope 2 (inlined main::{closure#0}) { // at 27925-reduced.rs:3:5: 3:8
        }
    }

    bb0: {
        Coverage::Expression(4294967295) = 1 + 0 for 27925-reduced.rs:2:18 - 4:2; // scope 1 at 27925-reduced.rs:3:5: 3:8
        Coverage::Counter(1) for 27925-reduced.rs:1:1 - 2:13; // scope 1 at 27925-reduced.rs:3:5: 3:8
        StorageLive(_1);                 // scope 0 at 27925-reduced.rs:2:9: 2:10
        Coverage::Counter(1) for 27925-reduced.rs:2:16 - 2:18; // scope 2 at 27925-reduced.rs:3:5: 3:8
        _0 = const ();                   // scope 0 at 27925-reduced.rs:1:15: 4:2
        StorageDead(_1);                 // scope 0 at 27925-reduced.rs:4:1: 4:2
        return;                          // scope 0 at 27925-reduced.rs:4:2: 4:2
    }
}
