
// WARNING: This output format is intended for human consumers only
// and is subject to change without notice. Knock yourself out.
fn main() -> () {
    let mut _0: ();                      // return pointer

    bb0: {                              
        _0 = ();                         // scope 0 at src/main.rs:7:11: 8:2
        return;                          // scope 0 at src/main.rs:8:2: 8:2
    }
}

fn map_or(_1: std::option::Option<T>, _2: U, _3: F) -> U {
    let mut _0: U;                       // return pointer
    scope 1 {
        let _4: T;                       // "t" in scope 1 at src/main.rs:3:18: 3:19
    }
    let mut _5: isize;
    let mut _6: F;
    let mut _7: (T,);
    let mut _8: T;
    let mut _9: U;
    let mut _10: bool;
    let mut _11: bool;
    let mut _12: bool;
    let mut _13: isize;
    let mut _14: isize;

    bb0: {                              
        _12 = const false;               // scope 1 at src/main.rs:3:13: 3:20
        _10 = const false;               // scope 1 at src/main.rs:3:13: 3:20
        _11 = const false;               // scope 1 at src/main.rs:3:13: 3:20
        _10 = const true;                // scope 1 at src/main.rs:3:13: 3:20
        _11 = const true;                // scope 1 at src/main.rs:3:13: 3:20
        _12 = const true;                // scope 1 at src/main.rs:3:13: 3:20
        _5 = discriminant(_1);           // scope 1 at src/main.rs:3:13: 3:20
        switchInt(_5) -> [0isize: bb1, 1isize: bb3, otherwise: bb2]; // scope 1 at src/main.rs:3:13: 3:20
    }

    bb1: {                              
        StorageLive(_9);                 // scope 1 at src/main.rs:4:21: 4:28
        _11 = const false;               // scope 1 at src/main.rs:4:21: 4:28
        _9 = _2;                         // scope 1 at src/main.rs:4:21: 4:28
        _0 = _9;                         // scope 1 at src/main.rs:4:21: 4:28
        StorageDead(_9);                 // scope 1 at src/main.rs:4:28: 4:28
        goto -> bb4;                     // scope 1 at src/main.rs:2:9: 5:10
    }

    bb2: {                              
        unreachable;                     // scope 0 at src/main.rs:6:6: 6:6
    }

    bb3: {                              
        StorageLive(_4);                 // scope 1 at src/main.rs:3:18: 3:19
        _10 = const false;               // scope 1 at src/main.rs:3:18: 3:19
        _4 = ((_1 as Some).0: T);        // scope 1 at src/main.rs:3:18: 3:19
        StorageLive(_6);                 // scope 1 at src/main.rs:3:24: 3:25
        _12 = const false;               // scope 1 at src/main.rs:3:24: 3:25
        _6 = _3;                         // scope 1 at src/main.rs:3:24: 3:25
        StorageLive(_7);                 // scope 1 at src/main.rs:3:24: 3:28
        StorageLive(_8);                 // scope 1 at src/main.rs:3:26: 3:27
        _8 = _4;                         // scope 1 at src/main.rs:3:26: 3:27
        _7 = (_8,);                      // scope 1 at src/main.rs:3:24: 3:28
        _0 = const std::ops::FnOnce::call_once(_6, _7) -> [return: bb7, unwind: bb14]; // scope 1 at src/main.rs:3:24: 3:28
    }

    bb4: {                              
        StorageDead(_4);                 // scope 0 at src/main.rs:5:10: 5:10
        switchInt(_12) -> [0u8: bb8, otherwise: bb15]; // scope 0 at src/main.rs:6:6: 6:6
    }

    bb5: {                               // cleanup
        resume;                          // scope 0 at src/main.rs:1:5: 6:6
    }

    bb6: {                               // cleanup
        _13 = discriminant(_1);          // scope 0 at src/main.rs:6:6: 6:6
        switchInt(_13) -> [1isize: bb10, otherwise: bb12]; // scope 0 at src/main.rs:6:6: 6:6
    }

    bb7: {                              
        StorageDead(_7);                 // scope 1 at src/main.rs:3:28: 3:28
        StorageDead(_8);                 // scope 1 at src/main.rs:3:28: 3:28
        StorageDead(_6);                 // scope 1 at src/main.rs:3:28: 3:28
        goto -> bb4;                     // scope 1 at src/main.rs:2:9: 5:10
    }

    bb8: {                              
        switchInt(_11) -> [0u8: bb9, otherwise: bb16]; // scope 0 at src/main.rs:6:6: 6:6
    }

    bb9: {                              
        _14 = discriminant(_1);          // scope 0 at src/main.rs:6:6: 6:6
        switchInt(_14) -> [1isize: bb18, otherwise: bb20]; // scope 0 at src/main.rs:6:6: 6:6
    }

    bb10: {                              // cleanup
        switchInt(_10) -> [0u8: bb5, otherwise: bb11]; // scope 0 at src/main.rs:6:6: 6:6
    }

    bb11: {                              // cleanup
        _10 = const false;               // scope 0 at src/main.rs:6:6: 6:6
        drop(((_1 as Some).0: T)) -> bb5; // scope 0 at src/main.rs:6:6: 6:6
    }

    bb12: {                              // cleanup
        drop(_1) -> bb5;                 // scope 0 at src/main.rs:6:6: 6:6
    }

    bb13: {                              // cleanup
        _11 = const false;               // scope 0 at src/main.rs:6:6: 6:6
        drop(_2) -> bb6;                 // scope 0 at src/main.rs:6:6: 6:6
    }

    bb14: {                              // cleanup
        switchInt(_11) -> [0u8: bb6, otherwise: bb13]; // scope 0 at src/main.rs:6:6: 6:6
    }

    bb15: {                             
        _12 = const false;               // scope 0 at src/main.rs:6:6: 6:6
        drop(_3) -> [return: bb8, unwind: bb14]; // scope 0 at src/main.rs:6:6: 6:6
    }

    bb16: {                             
        _11 = const false;               // scope 0 at src/main.rs:6:6: 6:6
        drop(_2) -> [return: bb9, unwind: bb6]; // scope 0 at src/main.rs:6:6: 6:6
    }

    bb17: {                             
        return;                          // scope 0 at src/main.rs:6:6: 6:6
    }

    bb18: {                             
        switchInt(_10) -> [0u8: bb17, otherwise: bb19]; // scope 0 at src/main.rs:6:6: 6:6
    }

    bb19: {                             
        _10 = const false;               // scope 0 at src/main.rs:6:6: 6:6
        drop(((_1 as Some).0: T)) -> bb17; // scope 0 at src/main.rs:6:6: 6:6
    }

    bb20: {                             
        drop(_1) -> bb17;                // scope 0 at src/main.rs:6:6: 6:6
    }
}
