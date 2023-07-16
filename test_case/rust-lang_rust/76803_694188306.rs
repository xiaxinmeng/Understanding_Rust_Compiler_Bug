
    bb0: {
        _2 = discriminant(_1);           // scope 0 at src/test/mir-opt/issue-76803.rs:12:9: 12:16
        switchInt(move _2) -> [0_isize: bb2, otherwise: bb1]; // scope 0 at src/test/mir-opt/issue-76803.rs:12:9: 12:16
        //                     ^^^^^^^ not the same value!
    }

    bb1: {
        _0 = move _1;                    // scope 0 at src/test/mir-opt/issue-76803.rs:13:14: 13:15
        goto -> bb3;                     // scope 0 at src/test/mir-opt/issue-76803.rs:11:5: 14:6
    }

    bb2: {
        discriminant(_0) = 1;            // scope 0 at src/test/mir-opt/issue-76803.rs:12:20: 12:27
        //                 ^ not the same value!
        goto -> bb3;                     // scope 0 at src/test/mir-opt/issue-76803.rs:11:5: 14:6
    }
