
bb0: {                              
        StorageLive(_1);                 // bb0[0]: scope 0 at src/main.rs:9:9: 9:14
        _1 = Xyz::A;                     // bb0[1]: scope 0 at src/main.rs:9:17: 9:23
        StorageLive(_2);                 // bb0[2]: scope 1 at src/main.rs:10:9: 10:10
        _2 = &mut _1;                    // bb0[3]: scope 1 at src/main.rs:10:13: 10:19
        _4 = discriminant(_1);           // bb0[4]: scope 3 at src/main.rs:12:9: 12:15
        switchInt(move _4) -> [0isize: bb1, 1isize: bb2, otherwise: bb3]; // bb0[5]: scope 3 at src/main.rs:12:9: 12:15
    }
