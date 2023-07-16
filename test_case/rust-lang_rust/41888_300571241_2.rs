rust
// WARNING: This output format is intended for human consumers only
// and is subject to change without notice. Knock yourself out.
fn X::A(_1: std::boxed::Box<X>) -> X {
    let mut _0: X;                       // return pointer

    bb0: {
        _0 = X::A(_1,);                  // scope 0 at 1.rs:12:5: 12:14
        return;                          // scope 0 at 1.rs:12:5: 12:14
    }
}

fn E::G(_1: [X; 2]) -> E {
    let mut _0: E;                       // return pointer

    bb0: {
        _0 = E::G(_1,);                  // scope 0 at 1.rs:24:5: 24:14
        return;                          // scope 0 at 1.rs:24:5: 24:14
    }
}

fn X::B(_1: std::boxed::Box<X>) -> X {
    let mut _0: X;                       // return pointer

    bb0: {
        _0 = X::B(_1,);                  // scope 0 at 1.rs:13:5: 13:14
        return;                          // scope 0 at 1.rs:13:5: 13:14
    }
}

const E::{{initializer}}: usize = {
    let mut _0: usize;                   // return pointer

    bb0: {
        _0 = const 2usize;               // scope 0 at 1.rs:24:11: 24:12
        return;                          // scope 0 at 1.rs:24:11: 24:12
    }
}

fn g(_1: (u32, u8)) -> std::result::Result<(), (u32, u8)> {
    let mut _0: std::result::Result<(), (u32, u8)>; // return pointer
    scope 1 {
        let mut _2: (u32, u8);           // "pt" in scope 1 at 1.rs:27:6: 27:12
        scope 2 {
            let mut _3: std::option::Option<(u32, u8)>; // "y" in scope 2 at 1.rs:28:9: 28:14
            scope 3 {
                let _5: std::option::Option<E>; // "status" in scope 3 at 1.rs:30:13: 30:19
                scope 4 {
                    let _10: E;          // "infix_or_postfix" in scope 4 at 1.rs:38:18: 38:34
                    scope 5 {
                        let _12: K;      // "_op" in scope 5 at 1.rs:39:29: 39:32
                        scope 6 {
                            let _19: (u32, u8); // "err" in scope 6 at 1.rs:41:21: 41:28
                        }
                        scope 7 {
                            let _20: (); // "val" in scope 7 at 1.rs:41:21: 41:28
                        }
                    }
                }
                scope 8 {
                    let _29: (u32, u8);  // "err" in scope 8 at 1.rs:45:17: 45:33
                }
                scope 9 {
                    let _30: ();         // "val" in scope 9 at 1.rs:45:17: 45:33
                }
            }
        }
    }
    let mut _4: ();
    let mut _6: bool;
    let mut _7: u32;
    let mut _8: E;
    let mut _9: K;
    let mut _11: isize;
    let mut _13: isize;
    let mut _14: (u32, u8);
    let mut _15: ();
    let mut _16: std::result::Result<(), (u32, u8)>;
    let mut _17: std::result::Result<(), (u32, u8)>;
    let mut _18: ();
    let mut _21: isize;
    let mut _22: (u32, u8);
    let mut _23: (u32, u8);
    let mut _24: ();
    let mut _25: std::result::Result<(), (u32, u8)>;
    let mut _26: std::result::Result<(), (u32, u8)>;
    let mut _27: (u32, u8);
    let mut _28: std::option::Option<(u32, u8)>;
    let mut _31: isize;
    let mut _32: (u32, u8);
    let mut _33: (u32, u8);
    let mut _34: ();
    let mut _35: bool;
    let mut _36: bool;
    let mut _37: bool;
    let mut _38: bool;
    let mut _39: isize;
    let mut _40: isize;
    let mut _41: isize;
    let mut _42: isize;

    bb0: {
        _37 = const false;               // scope 0 at 1.rs:27:6: 27:12
        _36 = const false;               // scope 0 at 1.rs:27:6: 27:12
        _35 = const false;               // scope 0 at 1.rs:27:6: 27:12
        _38 = const false;               // scope 0 at 1.rs:27:6: 27:12
        StorageLive(_2);                 // scope 0 at 1.rs:27:6: 27:12
        _2 = _1;                         // scope 0 at 1.rs:27:6: 27:12
        StorageLive(_3);                 // scope 1 at 1.rs:28:9: 28:14
        _3 = std::option::Option<(u32, u8)>::None; // scope 1 at 1.rs:28:17: 28:21
        goto -> bb1;                     // scope 2 at 1.rs:29:5: 48:6
    }

    bb1: {
        StorageLive(_5);                 // scope 2 at 1.rs:30:13: 30:19
        StorageLive(_6);                 // scope 2 at 1.rs:30:25: 30:34
        StorageLive(_7);                 // scope 2 at 1.rs:30:25: 30:29
        _7 = (_2.0: u32);                // scope 2 at 1.rs:30:25: 30:29
        _6 = Eq(_7, const 0u32);         // scope 2 at 1.rs:30:25: 30:34
        StorageDead(_7);                 // scope 2 at 1.rs:30:34: 30:34
        switchInt(_6) -> [0u8: bb3, otherwise: bb2]; // scope 2 at 1.rs:30:22: 34:10
    }

    bb2: {
        StorageLive(_8);                 // scope 2 at 1.rs:31:18: 31:28
        StorageLive(_9);                 // scope 2 at 1.rs:31:23: 31:27
        _9 = K::D;                       // scope 2 at 1.rs:31:23: 31:27
        _8 = E::F(_9,);                  // scope 2 at 1.rs:31:18: 31:28
        StorageDead(_9);                 // scope 2 at 1.rs:31:28: 31:28
        _38 = const true;                // scope 2 at 1.rs:31:13: 31:29
        _5 = std::option::Option<E>::Some(_8,); // scope 2 at 1.rs:31:13: 31:29
        StorageDead(_8);                 // scope 2 at 1.rs:31:29: 31:29
        goto -> bb4;                     // scope 2 at 1.rs:30:22: 34:10
    }

    bb3: {
        _38 = const true;                // scope 2 at 1.rs:33:13: 33:17
        _5 = std::option::Option<E>::None; // scope 2 at 1.rs:33:13: 33:17
        goto -> bb4;                     // scope 2 at 1.rs:30:22: 34:10
    }

    bb4: {
        StorageDead(_6);                 // scope 2 at 1.rs:34:10: 34:10
        (_2.0: u32) = const 1u32;        // scope 3 at 1.rs:35:9: 35:17
        _11 = discriminant(_5);          // scope 3 at 1.rs:38:13: 38:35
        switchInt(_11) -> [1isize: bb6, otherwise: bb5]; // scope 3 at 1.rs:38:13: 38:35
    }

    bb5: {
        StorageLive(_25);                // scope 3 at 1.rs:45:17: 45:33
        StorageLive(_26);                // scope 3 at 1.rs:45:17: 45:32
        StorageLive(_27);                // scope 3 at 1.rs:45:21: 45:31
        StorageLive(_28);                // scope 3 at 1.rs:45:21: 45:22
        _28 = _3;                        // scope 3 at 1.rs:45:21: 45:22
        _27 = const <std::option::Option<T>>::unwrap(_28) -> bb18; // scope 3 at 1.rs:45:21: 45:31
    }

    bb6: {
        StorageLive(_10);                // scope 3 at 1.rs:38:18: 38:34
        _38 = const false;               // scope 3 at 1.rs:38:18: 38:34
        _35 = const true;                // scope 3 at 1.rs:38:18: 38:34
        _36 = const true;                // scope 3 at 1.rs:38:18: 38:34
        _37 = const true;                // scope 3 at 1.rs:38:18: 38:34
        _10 = ((_5 as Some).0: E);       // scope 3 at 1.rs:38:18: 38:34
        _13 = discriminant(_10);         // scope 4 at 1.rs:39:24: 39:33
        switchInt(_13) -> [0isize: bb9, otherwise: bb8]; // scope 4 at 1.rs:39:24: 39:33
    }

    bb7: {
        switchInt(_35) -> [0u8: bb24, otherwise: bb28]; // scope 3 at 1.rs:47:10: 47:10
    }

    bb8: {
        _4 = ();                         // scope 4 at 1.rs:39:17: 42:18
        goto -> bb10;                    // scope 4 at 1.rs:39:17: 42:18
    }

    bb9: {
        StorageLive(_12);                // scope 4 at 1.rs:39:29: 39:32
        _37 = const false;               // scope 4 at 1.rs:39:29: 39:32
        _12 = ((_10 as F).0: K);         // scope 4 at 1.rs:39:29: 39:32
        StorageLive(_14);                // scope 5 at 1.rs:40:30: 40:32
        _14 = _2;                        // scope 5 at 1.rs:40:30: 40:32
        _3 = std::option::Option<(u32, u8)>::Some(_14,); // scope 5 at 1.rs:40:21: 40:33
        StorageDead(_14);                // scope 5 at 1.rs:40:33: 40:33
        StorageLive(_16);                // scope 5 at 1.rs:41:21: 41:28
        StorageLive(_17);                // scope 5 at 1.rs:41:21: 41:27
        StorageLive(_18);                // scope 5 at 1.rs:41:24: 41:26
        _18 = ();                        // scope 5 at 1.rs:41:24: 41:26
        _17 = std::result::Result<(), (u32, u8)>::Ok(_18,); // scope 5 at 1.rs:41:21: 41:27
        StorageDead(_18);                // scope 5 at 1.rs:41:27: 41:27
        _16 = const std::ops::Carrier::translate(_17) -> bb11; // scope 5 at 1.rs:41:21: 41:28
    }

    bb10: {
        StorageDead(_12);                // scope 4 at 1.rs:42:18: 42:18
        goto -> bb7;                     // scope 3 at 1.rs:37:9: 47:10
    }

    bb11: {
        StorageDead(_17);                // scope 5 at 1.rs:41:28: 41:28
        _21 = discriminant(_16);         // scope 5 at 1.rs:41:21: 41:28
        switchInt(_21) -> [0isize: bb12, otherwise: bb13]; // scope 5 at 1.rs:41:21: 41:28
    }

    bb12: {
        StorageLive(_20);                // scope 5 at 1.rs:41:21: 41:28
        _20 = ((_16 as Ok).0: ());       // scope 5 at 1.rs:41:21: 41:28
        StorageLive(_24);                // scope 7 at 1.rs:41:21: 41:28
        _24 = _20;                       // scope 7 at 1.rs:41:21: 41:28
        _15 = _24;                       // scope 7 at 1.rs:41:21: 41:28
        StorageDead(_24);                // scope 7 at 1.rs:41:28: 41:28
        StorageDead(_19);                // scope 5 at 1.rs:41:28: 41:28
        StorageDead(_20);                // scope 5 at 1.rs:41:28: 41:28
        StorageDead(_16);                // scope 5 at 1.rs:41:29: 41:29
        _4 = ();                         // scope 5 at 1.rs:39:53: 42:18
        goto -> bb10;                    // scope 4 at 1.rs:39:17: 42:18
    }

    bb13: {
        StorageLive(_19);                // scope 5 at 1.rs:41:21: 41:28
        _19 = ((_16 as Err).0: (u32, u8)); // scope 5 at 1.rs:41:21: 41:28
        StorageLive(_22);                // scope 6 at 1.rs:41:21: 41:28
        StorageLive(_23);                // scope 6 at 1.rs:41:21: 41:28
        _23 = _19;                       // scope 6 at 1.rs:41:21: 41:28
        _22 = const std::convert::From::from(_23) -> bb14; // scope 6 at 1.rs:41:21: 41:28
    }

    bb14: {
        StorageDead(_23);                // scope 6 at 1.rs:41:28: 41:28
        _0 = const std::ops::Carrier::from_error(_22) -> bb15; // scope 6 at 1.rs:41:21: 41:28
    }

    bb15: {
        StorageDead(_22);                // scope 6 at 1.rs:41:28: 41:28
        StorageDead(_19);                // scope 5 at 1.rs:41:28: 41:28
        StorageDead(_20);                // scope 5 at 1.rs:41:28: 41:28
        StorageDead(_16);                // scope 5 at 1.rs:41:29: 41:29
        StorageDead(_12);                // scope 4 at 1.rs:42:18: 42:18
        goto -> bb32;                    // scope 3 at 1.rs:41:21: 41:28
    }

    bb16: {
        StorageDead(_10);                // scope 3 at 1.rs:47:10: 47:10
        _41 = discriminant(_5);          // scope 2 at 1.rs:48:6: 48:6
        switchInt(_41) -> [1isize: bb34, otherwise: bb33]; // scope 2 at 1.rs:48:6: 48:6
    }

    bb17: {
        StorageDead(_5);                 // scope 2 at 1.rs:48:6: 48:6
        StorageDead(_3);                 // scope 1 at 1.rs:49:2: 49:2
        StorageDead(_2);                 // scope 0 at 1.rs:49:2: 49:2
        return;                          // scope 1 at 1.rs:49:2: 49:2
    }

    bb18: {
        StorageDead(_28);                // scope 3 at 1.rs:45:31: 45:31
        _26 = std::result::Result<(), (u32, u8)>::Err(_27,); // scope 3 at 1.rs:45:17: 45:32
        StorageDead(_27);                // scope 3 at 1.rs:45:32: 45:32
        _25 = const std::ops::Carrier::translate(_26) -> bb19; // scope 3 at 1.rs:45:17: 45:33
    }

    bb19: {
        StorageDead(_26);                // scope 3 at 1.rs:45:33: 45:33
        _31 = discriminant(_25);         // scope 3 at 1.rs:45:17: 45:33
        switchInt(_31) -> [0isize: bb20, otherwise: bb21]; // scope 3 at 1.rs:45:17: 45:33
    }

    bb20: {
        StorageLive(_30);                // scope 3 at 1.rs:45:17: 45:33
        _30 = ((_25 as Ok).0: ());       // scope 3 at 1.rs:45:17: 45:33
        StorageLive(_34);                // scope 9 at 1.rs:45:17: 45:33
        _34 = _30;                       // scope 9 at 1.rs:45:17: 45:33
        _4 = _34;                        // scope 9 at 1.rs:45:17: 45:33
        StorageDead(_34);                // scope 9 at 1.rs:45:33: 45:33
        StorageDead(_29);                // scope 3 at 1.rs:45:33: 45:33
        StorageDead(_30);                // scope 3 at 1.rs:45:33: 45:33
        StorageDead(_25);                // scope 3 at 1.rs:46:14: 46:14
        goto -> bb7;                     // scope 3 at 1.rs:37:9: 47:10
    }

    bb21: {
        StorageLive(_29);                // scope 3 at 1.rs:45:17: 45:33
        _29 = ((_25 as Err).0: (u32, u8)); // scope 3 at 1.rs:45:17: 45:33
        StorageLive(_32);                // scope 8 at 1.rs:45:17: 45:33
        StorageLive(_33);                // scope 8 at 1.rs:45:17: 45:33
        _33 = _29;                       // scope 8 at 1.rs:45:17: 45:33
        _32 = const std::convert::From::from(_33) -> bb22; // scope 8 at 1.rs:45:17: 45:33
    }

    bb22: {
        StorageDead(_33);                // scope 8 at 1.rs:45:33: 45:33
        _0 = const std::ops::Carrier::from_error(_32) -> bb23; // scope 8 at 1.rs:45:17: 45:33
    }

    bb23: {
        StorageDead(_32);                // scope 8 at 1.rs:45:33: 45:33
        StorageDead(_29);                // scope 3 at 1.rs:45:33: 45:33
        StorageDead(_30);                // scope 3 at 1.rs:45:33: 45:33
        StorageDead(_25);                // scope 3 at 1.rs:46:14: 46:14
        goto -> bb32;                    // scope 3 at 1.rs:45:17: 45:33
    }

    bb24: {
        StorageDead(_10);                // scope 3 at 1.rs:47:10: 47:10
        _42 = discriminant(_5);          // scope 2 at 1.rs:48:6: 48:6
        switchInt(_42) -> [1isize: bb37, otherwise: bb36]; // scope 2 at 1.rs:48:6: 48:6
    }

    bb25: {
        StorageDead(_5);                 // scope 2 at 1.rs:48:6: 48:6
        goto -> bb1;                     // scope 2 at 1.rs:29:5: 48:6
    }

    bb26: {
        _35 = const false;               // scope 3 at 1.rs:47:10: 47:10
        drop(_10) -> bb24;               // scope 3 at 1.rs:47:10: 47:10
    }

    bb27: {
        switchInt(_35) -> [0u8: bb24, otherwise: bb26]; // scope 3 at 1.rs:47:10: 47:10
    }

    bb28: {
        _39 = discriminant(_10);         // scope 3 at 1.rs:47:10: 47:10
        switchInt(_39) -> [0isize: bb24, otherwise: bb27]; // scope 3 at 1.rs:47:10: 47:10
    }

    bb29: {
        _35 = const false;               // scope 3 at 1.rs:47:10: 47:10
        drop(_10) -> bb16;               // scope 3 at 1.rs:47:10: 47:10
    }

    bb30: {
        switchInt(_35) -> [0u8: bb16, otherwise: bb29]; // scope 3 at 1.rs:47:10: 47:10
    }

    bb31: {
        _40 = discriminant(_10);         // scope 3 at 1.rs:47:10: 47:10
        switchInt(_40) -> [0isize: bb16, otherwise: bb30]; // scope 3 at 1.rs:47:10: 47:10
    }

    bb32: {
        switchInt(_35) -> [0u8: bb16, otherwise: bb31]; // scope 3 at 1.rs:47:10: 47:10
    }

    bb33: {
        drop(_5) -> bb17;                // scope 2 at 1.rs:48:6: 48:6
    }

    bb34: {
        switchInt(_38) -> [0u8: bb17, otherwise: bb35]; // scope 2 at 1.rs:48:6: 48:6
    }

    bb35: {
        _38 = const false;               // scope 2 at 1.rs:48:6: 48:6
        drop(((_5 as Some).0: E)) -> bb17; // scope 2 at 1.rs:48:6: 48:6
    }

    bb36: {
        drop(_5) -> bb25;                // scope 2 at 1.rs:48:6: 48:6
    }

    bb37: {
        switchInt(_38) -> [0u8: bb25, otherwise: bb38]; // scope 2 at 1.rs:48:6: 48:6
    }

    bb38: {
        _38 = const false;               // scope 2 at 1.rs:48:6: 48:6
        drop(((_5 as Some).0: E)) -> bb25; // scope 2 at 1.rs:48:6: 48:6
    }
}

fn E::F(_1: K) -> E {
    let mut _0: E;                       // return pointer

    bb0: {
        _0 = E::F(_1,);                  // scope 0 at 1.rs:22:5: 22:9
        return;                          // scope 0 at 1.rs:22:5: 22:9
    }
}

fn main() -> () {
    let mut _0: ();                      // return pointer
    let mut _1: std::result::Result<(), (u32, u8)>;
    let mut _2: (u32, u8);

    bb0: {
        StorageLive(_1);                 // scope 0 at 1.rs:4:13: 4:22
        StorageLive(_2);                 // scope 0 at 1.rs:4:15: 4:21
        _2 = (const 0u32, const 0u8);    // scope 0 at 1.rs:4:15: 4:21
        _1 = const g(_2) -> bb1;         // scope 0 at 1.rs:4:13: 4:22
    }

    bb1: {
        StorageDead(_2);                 // scope 0 at 1.rs:4:22: 4:22
        StorageDead(_1);                 // scope 0 at 1.rs:4:23: 4:23
        _0 = ();                         // scope 0 at 1.rs:3:11: 5:2
        return;                          // scope 0 at 1.rs:5:2: 5:2
    }
}
