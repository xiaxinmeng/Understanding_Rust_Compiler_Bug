rust
// MIR for `f` before SimplifyComparisonIntegral

fn f(_1: i8) -> i8 {
...
    bb0: {
        StorageLive(_2);                 // scope 0 at switchInt.rs:4:9: 4:10
        _2 = _1;                         // scope 0 at switchInt.rs:4:13: 4:14
        StorageLive(_3);                 // scope 1 at switchInt.rs:5:8: 5:15
        StorageLive(_4);                 // scope 1 at switchInt.rs:5:8: 5:9
        _4 = _2;                         // scope 1 at switchInt.rs:5:8: 5:9
        _3 = Eq(move _4, const 42_i8);   // scope 1 at switchInt.rs:5:8: 5:15
        StorageDead(_4);                 // scope 1 at switchInt.rs:5:14: 5:15
        switchInt(_3) -> [false: bb1, otherwise: bb2]; // scope 1 at switchInt.rs:5:5: 5:32
    }
...
}
