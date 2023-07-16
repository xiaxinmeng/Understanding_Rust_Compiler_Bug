
bb0: {
    switchInt(_1) -> [0_i32: bb2, 1_i32: bb4, 2_i32: bb6, otherwise: bb1]; // scope 0 at ./example.rs:10:13: 10:14
}

bb1: {
    return;                          // scope 0 at ./example.rs:16:2: 16:2
}

bb2: {
    _4 = const "0";                  // scope 0 at ./example.rs:10:26: 10:29
    _3 = _4;                         // scope 0 at ./example.rs:10:26: 10:29
    _2 = print(move _3) -> bb3;      // scope 0 at ./example.rs:10:20: 10:30
}

bb3: {
    _1 = const 2_i32;                // scope 0 at ./example.rs:10:32: 10:37
    goto -> bb0;                     // scope 0 at ./example.rs:9:9: 14:10
}

bb4: {
    _7 = const "1";                  // scope 0 at ./example.rs:11:26: 11:29
    _6 = _7;                         // scope 0 at ./example.rs:11:26: 11:29
    _5 = print(move _6) -> bb5;      // scope 0 at ./example.rs:11:20: 11:30
}

bb5: {
    _1 = const 3_i32;                // scope 0 at ./example.rs:11:32: 11:37
    goto -> bb0;                     // scope 0 at ./example.rs:9:9: 14:10
}

bb6: {
    _10 = const "2";                 // scope 0 at ./example.rs:12:26: 12:29
    _9 = _10;                        // scope 0 at ./example.rs:12:26: 12:29
    _8 = print(move _9) -> bb7;      // scope 0 at ./example.rs:12:20: 12:30
}

bb7: {
    _1 = const 1_i32;                // scope 0 at ./example.rs:12:32: 12:37
    goto -> bb0;                     // scope 0 at ./example.rs:9:9: 14:10
}
