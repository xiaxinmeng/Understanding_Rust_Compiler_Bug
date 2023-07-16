    bb2: {
        StorageLive(_3);                 // bb2[0]: scope 0 at src/main.rs:12:17: 12:18
        _3 = move ((_1 as Some).0: Empty); // bb2[1]: scope 0 at src/main.rs:12:17: 12:18
        StorageLive(_4);                 // bb2[2]: scope 1 at src/main.rs:13:9: 13:19
        StorageLive(_5);                 // bb2[3]: scope 1 at src/main.rs:13:17: 13:18
        _5 = move _3;                    // bb2[4]: scope 1 at src/main.rs:13:17: 13:18
        ((_4 as B).0: Empty) = move _5;  // bb2[5]: scope 1 at src/main.rs:13:9: 13:19
        discriminant(_4) = 1;            // bb2[6]: scope 1 at src/main.rs:13:9: 13:19
        StorageDead(_5);                 // bb2[7]: scope 1 at src/main.rs:13:18: 13:19
        StorageDead(_4);                 // bb2[8]: scope 1 at src/main.rs:13:19: 13:20
        StorageDead(_3);                 // bb2[9]: scope 0 at src/main.rs:14:5: 14:6
        goto -> bb3;                     // bb2[10]: scope 0 at src/main.rs:12:5: 14:6
    }
