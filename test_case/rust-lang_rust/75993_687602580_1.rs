rust
// MIR for `f` after SimplifyComparisonIntegral                                                                                                     
                                                                                                                                                    
fn f(_1: i8) -> i8 {                                                                                                                                
...
    bb0: {
        StorageLive(_2);                 // scope 0 at switchInt.rs:4:9: 4:10
        _2 = _1;                         // scope 0 at switchInt.rs:4:13: 4:14
        StorageLive(_3);                 // scope 1 at switchInt.rs:5:8: 5:15
        StorageLive(_4);                 // scope 1 at switchInt.rs:5:8: 5:9
        _4 = _2;                         // scope 1 at switchInt.rs:5:8: 5:9
        _3 = Eq(_4, const 42_i8);        // scope 1 at switchInt.rs:5:8: 5:15
        nop;                             // scope 1 at switchInt.rs:5:14: 5:15
        switchInt(move _4) -> [42_i8: bb2, otherwise: bb1]; // scope 1 at switchInt.rs:5:5: 5:32
    }
...
    bb1: {
        StorageDead(_4);                 // scope 1 at switchInt.rs:5:5: 5:32
        ...
    }
    bb2: {
        StorageDead(_4);                 // scope 1 at switchInt.rs:5:5: 5:32
        ...
   }
...
}
