rust
    bb1: {
        StorageDead(_3);                 // bb1[0]: scope 1 at src/lib.rs:5:11: 5:12
        StorageDead(_2);                 // bb1[1]: scope 1 at src/lib.rs:5:12: 5:13
        StorageLive(_4);                 // bb1[2]: scope 1 at src/lib.rs:6:9: 6:10
        _4 = _1;                         // bb1[3]: scope 1 at src/lib.rs:6:13: 6:14
        StorageDead(_4);                 // bb1[4]: scope 1 at src/lib.rs:7:1: 7:2
        StorageDead(_1);                 // bb1[5]: scope 0 at src/lib.rs:7:1: 7:2
        return;                          // bb1[6]: scope 0 at src/lib.rs:7:2: 7:2
    }
