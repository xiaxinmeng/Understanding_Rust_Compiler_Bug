
$ rustc +nightly t.rs '-Zdump-mir=foo_if|foo_match' --crate-type=lib
$ cat mir_dump/t.foo_if.000-000.SimplifyCfg-initial.before.mir
// MIR for `foo_if` before SimplifyCfg-initial

fn foo_if(_1: bool) -> u32 {
    debug foo => _1;                     // in scope 0 at t.rs:1:15: 1:18
    let mut _0: u32;                     // return place in scope 0 at t.rs:1:29: 1:32
    let mut _2: bool;                    // in scope 0 at t.rs:2:8: 2:11

    bb0: {
        StorageLive(_2);                 // scope 0 at t.rs:2:8: 2:11
        _2 = _1;                         // scope 0 at t.rs:2:8: 2:11
        switchInt(move _2) -> [false: bb2, otherwise: bb1]; // scope 0 at t.rs:2:8: 2:11
    }

    bb1: {
        _0 = const 1_u32;                // scope 0 at t.rs:3:9: 3:10
        goto -> bb4;                     // scope 0 at t.rs:2:5: 6:6
    }

    bb2: {
        goto -> bb3;                     // scope 0 at t.rs:2:8: 2:11
    }

    bb3: {
        _0 = const 2_u32;                // scope 0 at t.rs:5:9: 5:10
        goto -> bb4;                     // scope 0 at t.rs:2:5: 6:6
    }

    bb4: {
        StorageDead(_2);                 // scope 0 at t.rs:6:5: 6:6
        return;                          // scope 0 at t.rs:7:2: 7:2
    }
}
$ cat mir_dump/t.foo_match.000-000.SimplifyCfg-initial.before.mir
// MIR for `foo_match` before SimplifyCfg-initial

fn foo_match(_1: bool) -> u32 {
    debug foo => _1;                     // in scope 0 at t.rs:9:18: 9:21
    let mut _0: u32;                     // return place in scope 0 at t.rs:9:32: 9:35

    bb0: {
        FakeRead(ForMatchedPlace(None), _1); // scope 0 at t.rs:10:11: 10:14
        switchInt(_1) -> [false: bb2, otherwise: bb1]; // scope 0 at t.rs:10:5: 10:14
    }

    bb1: {
        falseEdge -> [real: bb3, imaginary: bb2]; // scope 0 at t.rs:11:9: 11:13
    }

    bb2: {
        _0 = const 2_u32;                // scope 0 at t.rs:12:18: 12:19
        goto -> bb4;                     // scope 0 at t.rs:12:18: 12:19
    }

    bb3: {
        _0 = const 1_u32;                // scope 0 at t.rs:11:17: 11:18
        goto -> bb4;                     // scope 0 at t.rs:11:17: 11:18
    }

    bb4: {
        return;                          // scope 0 at t.rs:14:2: 14:2
    }
}
