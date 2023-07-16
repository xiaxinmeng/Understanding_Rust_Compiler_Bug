
bb0: {
    switchInt(_1) -> [0_i32: bb2, 1_i32: bb3, 2_i32: bb4, otherwise: bb1]; // scope 0 at ./example.rs:10:13: 10:14
}

bb1: {
    return;                          // scope 0 at ./example.rs:16:2: 16:2
}

bb2: {
    _1 = const 2_i32;                // scope 0 at ./example.rs:10:20: 10:25
    _4 = const "0";                  // scope 0 at ./example.rs:10:33: 10:36
    _3 = _4;                         // scope 0 at ./example.rs:10:33: 10:36
    _2 = print(move _3) -> bb0;      // scope 0 at ./example.rs:10:27: 10:37
}

bb3: {
    _1 = const 3_i32;                // scope 0 at ./example.rs:11:20: 11:25
    _7 = const "1";                  // scope 0 at ./example.rs:11:33: 11:36
    _6 = _7;                         // scope 0 at ./example.rs:11:33: 11:36
    _5 = print(move _6) -> bb0;      // scope 0 at ./example.rs:11:27: 11:37
}

bb4: {
    _1 = const 1_i32;                // scope 0 at ./example.rs:12:20: 12:25
    _10 = const "2";                 // scope 0 at ./example.rs:12:33: 12:36
    _9 = _10;                        // scope 0 at ./example.rs:12:33: 12:36
    _8 = print(move _9) -> bb0;      // scope 0 at ./example.rs:12:27: 12:37
}
