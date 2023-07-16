
// MIR for `foo`
// source = Fn(NodeId(7))
// pass_name = Inline
// disambiguator = before

fn foo(_1: T, _2: i32) -> i32 {
    let mut _0: i32;                     // return pointer
    scope 1 {
        let _3: [closure@klosure.rs:6:13: 6:24]; // "x" in scope 1 at klosure.rs:6:9: 6:10
    }
    scope 2 {
    }
    let mut _4: &[closure@klosure.rs:6:13: 6:24];
    let mut _5: (i32, i32);
    let mut _6: i32;
    let mut _7: i32;
    let mut _8: i32;
    let mut _9: i32;

    bb0: {                              
        StorageLive(_3);                 // scope 1 at klosure.rs:6:9: 6:10
        _3 = [closure@klosure.rs:6:13: 6:24]; // scope 1 at klosure.rs:6:13: 6:24
        StorageLive(_4);                 // scope 1 at klosure.rs:7:5: 7:6
        _4 = &_3;                        // scope 1 at klosure.rs:7:5: 7:6
        StorageLive(_5);                 // scope 1 at klosure.rs:7:5: 7:16
        StorageLive(_6);                 // scope 1 at klosure.rs:7:7: 7:10
        StorageLive(_7);                 // scope 1 at klosure.rs:7:7: 7:8
        _7 = _2;                         // scope 1 at klosure.rs:7:7: 7:8
        _6 = Mul(_7, const 2i32);        // scope 1 at klosure.rs:7:7: 7:10
        StorageDead(_7);                 // scope 1 at klosure.rs:7:10: 7:10
        StorageLive(_8);                 // scope 1 at klosure.rs:7:12: 7:15
        StorageLive(_9);                 // scope 1 at klosure.rs:7:12: 7:13
        _9 = _2;                         // scope 1 at klosure.rs:7:12: 7:13
        _8 = Mul(_9, const 3i32);        // scope 1 at klosure.rs:7:12: 7:15
        StorageDead(_9);                 // scope 1 at klosure.rs:7:15: 7:15
        _5 = (_6, _8);                   // scope 1 at klosure.rs:7:5: 7:16
        _0 = const std::ops::Fn::call(_4, _5) -> bb1; // scope 1 at klosure.rs:7:5: 7:16
    }

    bb1: {                              
        StorageDead(_5);                 // scope 1 at klosure.rs:7:16: 7:16
        StorageDead(_8);                 // scope 1 at klosure.rs:7:16: 7:16
        StorageDead(_6);                 // scope 1 at klosure.rs:7:16: 7:16
        StorageDead(_4);                 // scope 1 at klosure.rs:7:16: 7:16
        StorageDead(_3);                 // scope 0 at klosure.rs:8:2: 8:2
        return;                          // scope 0 at klosure.rs:8:2: 8:2
    }
}

// MIR for `foo::{{closure}}`
// source = Fn(NodeId(28))
// pass_name = Inline
// disambiguator = before

fn foo::{{closure}}(_1: &[closure@klosure.rs:6:13: 6:24], _2: i32, _3: i32) -> i32 {
    let mut _0: i32;                     // return pointer
    let mut _4: i32;

    bb0: {                              
        StorageLive(_4);                 // scope 0 at klosure.rs:6:22: 6:24
        _4 = _2;                         // scope 0 at klosure.rs:6:22: 6:24
        _0 = _4;                         // scope 0 at klosure.rs:6:22: 6:24
        StorageDead(_4);                 // scope 0 at klosure.rs:6:24: 6:24
        return;                          // scope 0 at klosure.rs:6:24: 6:24
    }
}
