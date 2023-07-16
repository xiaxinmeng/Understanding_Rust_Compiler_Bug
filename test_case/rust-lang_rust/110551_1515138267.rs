rust
// WARNING: This output format is intended for human consumers only
// and is subject to change without notice. Knock yourself out.
fn f(_1: Option<u32>) -> u32 {
    debug a => _1;                       // in scope 0 at t.rs:1:10: 1:11
    let mut _0: u32;                     // return place in scope 0 at t.rs:1:29: 1:32
    let mut _2: isize;                   // in scope 0 at t.rs:3:9: 3:13
    scope 1 {
        scope 2 (inlined unreachable_unchecked) { // at t.rs:4:28: 4:62
            scope 3 {
                scope 4 (inlined unreachable_unchecked::runtime) { // at /rustc/c609da59d9fc05b1c7dc879d79700ccd8140b5fc/library/core/src/intrinsics.rs:2495:13: 2495:80
                }
            }
        }
    }
    scope 5 (inlined #[track_caller] Option::<u32>::unwrap) { // at t.rs:7:7: 7:15
        debug self => _1;                // in scope 5 at /rustc/c609da59d9fc05b1c7dc879d79700ccd8140b5fc/library/core/src/option.rs:947:25: 947:29
        let mut _3: isize;               // in scope 5 at /rustc/c609da59d9fc05b1c7dc879d79700ccd8140b5fc/library/core/src/option.rs:949:13: 949:22
        let mut _4: !;                   // in scope 5 at /rustc/c609da59d9fc05b1c7dc879d79700ccd8140b5fc/library/core/src/option.rs:950:21: 950:73
        scope 6 {
            debug val => _0;             // in scope 6 at /rustc/c609da59d9fc05b1c7dc879d79700ccd8140b5fc/library/core/src/option.rs:949:18: 949:21
        }
    }

    bb0: {
        _2 = discriminant(_1);           // scope 0 at t.rs:2:11: 2:12
        switchInt(move _2) -> [0: bb2, 1: bb1, otherwise: bb1]; // scope 0 at t.rs:2:5: 2:12
    }

    bb1: {
        unreachable;                     // scope 3 at /rustc/c609da59d9fc05b1c7dc879d79700ccd8140b5fc/library/core/src/intrinsics.rs:2480:9: 2496:10
    }

    bb2: {
        _3 = discriminant(_1);           // scope 5 at /rustc/c609da59d9fc05b1c7dc879d79700ccd8140b5fc/library/core/src/option.rs:948:15: 948:19
        switchInt(move _3) -> [0: bb3, 1: bb4, otherwise: bb1]; // scope 5 at /rustc/c609da59d9fc05b1c7dc879d79700ccd8140b5fc/library/core/src/option.rs:948:9: 948:19
    }

    bb3: {
        _4 = core::panicking::panic(const "called `Option::unwrap()` on a `None` value"); // scope 5 at /rustc/c609da59d9fc05b1c7dc879d79700ccd8140b5fc/library/core/src/option.rs:950:21: 950:73
                                         // mir::Constant
                                         // + span: /rustc/c609da59d9fc05b1c7dc879d79700ccd8140b5fc/library/core/src/option.rs:950:21: 950:26
                                         // + literal: Const { ty: fn(&'static str) -> ! {core::panicking::panic}, val: Value(<ZST>) }
                                         // mir::Constant
                                         // + span: /rustc/c609da59d9fc05b1c7dc879d79700ccd8140b5fc/library/core/src/option.rs:950:27: 950:72
                                         // + literal: Const { ty: &str, val: Value(Slice(..)) }
    }

    bb4: {
        _0 = ((_1 as Some).0: u32);      // scope 5 at /rustc/c609da59d9fc05b1c7dc879d79700ccd8140b5fc/library/core/src/option.rs:949:18: 949:21
        return;                          // scope 0 at t.rs:8:2: 8:2
    }
}
