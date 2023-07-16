
// MIR for `main::{closure#0}::{closure#0}` after CheckPackedRef

fn main::{closure#0}::{closure#0}(_1: [generator@src/main.rs:36:9: 40:10], _2: ()) -> ()
yields u8
 {
    debug start => (_1.0: u8);           // in scope 0 at src/main.rs:35:25: 35:30
    debug end => (_1.1: u8);             // in scope 0 at src/main.rs:33:9: 33:12
    let mut _0: ();                      // return place in scope 0 at src/main.rs:36:17: 36:17
    let mut _3: std::ops::Range<u8>;     // in scope 0 at src/main.rs:37:22: 37:32
    let mut _4: std::ops::Range<u8>;     // in scope 0 at src/main.rs:37:22: 37:32
    let mut _5: u8;                      // in scope 0 at src/main.rs:37:22: 37:27
    let mut _6: u8;                      // in scope 0 at src/main.rs:37:29: 37:32
    let mut _7: std::ops::Range<u8>;     // in scope 0 at src/main.rs:37:22: 37:32
    let mut _8: ();                      // in scope 0 at src/main.rs:36:9: 40:10
    let _10: ();                         // in scope 0 at src/main.rs:37:22: 37:32
    let mut _11: std::option::Option<u8>; // in scope 0 at src/main.rs:37:22: 37:32
    let mut _12: &mut std::ops::Range<u8>; // in scope 0 at src/main.rs:37:22: 37:32
    let mut _13: &mut std::ops::Range<u8>; // in scope 0 at src/main.rs:37:22: 37:32
    let mut _14: isize;                  // in scope 0 at src/main.rs:37:17: 37:18
    let mut _16: u8;                     // in scope 0 at src/main.rs:37:17: 37:18
    let mut _17: !;                      // in scope 0 at src/main.rs:37:13: 39:14
    let _19: ();                         // in scope 0 at src/main.rs:38:17: 38:24
    let mut _20: u8;                     // in scope 0 at src/main.rs:38:23: 38:24
    scope 1 {
        debug iter => _7;                // in scope 1 at src/main.rs:37:22: 37:32
        let mut _9: u8;                  // in scope 1 at src/main.rs:37:22: 37:32
        scope 2 {
            debug __next => _9;          // in scope 2 at src/main.rs:37:22: 37:32
            let _15: u8;                 // in scope 2 at src/main.rs:37:17: 37:18
            let _18: u8;                 // in scope 2 at src/main.rs:37:17: 37:18
            scope 3 {
                debug val => _15;        // in scope 3 at src/main.rs:37:17: 37:18
            }
            scope 4 {
                debug i => _18;          // in scope 4 at src/main.rs:37:17: 37:18
            }
        }
    }

    bb0: {
        StorageLive(_3);                 // scope 0 at src/main.rs:37:22: 37:32
        StorageLive(_4);                 // scope 0 at src/main.rs:37:22: 37:32
        StorageLive(_5);                 // scope 0 at src/main.rs:37:22: 37:27
        _5 = (_1.0: u8);                 // scope 0 at src/main.rs:37:22: 37:27
        StorageLive(_6);                 // scope 0 at src/main.rs:37:29: 37:32
        _6 = (_1.1: u8);                 // scope 0 at src/main.rs:37:29: 37:32
        _4 = std::ops::Range::<u8> { start: move _5, end: move _6 }; // scope 0 at src/main.rs:37:22: 37:32
        StorageDead(_6);                 // scope 0 at src/main.rs:37:31: 37:32
        StorageDead(_5);                 // scope 0 at src/main.rs:37:31: 37:32
        _3 = <std::ops::Range<u8> as IntoIterator>::into_iter(move _4) -> [return: bb1, unwind: bb18]; // scope 0 at src/main.rs:37:22: 37:32
                                         // mir::Constant
                                         // + span: src/main.rs:37:22: 37:32
                                         // + literal: Const { ty: fn(std::ops::Range<u8>) -> <std::ops::Range<u8> as std::iter::IntoIterator>::IntoIter {<std::ops::Range<u8> as std::iter::IntoIterator>::into_iter}, val: Value(Scalar(<ZST>)) }
    }
...
