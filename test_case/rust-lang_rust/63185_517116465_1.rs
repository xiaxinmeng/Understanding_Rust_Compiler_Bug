rust
        _4 = &_3;                        // bb1[3]: scope 0 at src/lib.rs:9:14: 9:23
        ...
        switchInt(move _5) -> [false: bb3, otherwise: bb2]; // bb1[9]: scope 0 at src/lib.rs:9:28: 9:35
    }

    bb2: {
        ...
        StorageLive(_7);                 // bb2[1]: scope 1 at src/lib.rs:9:46: 9:47
        _7 = _3;                         // bb2[2]: scope 1 at src/lib.rs:9:46: 9:47
        ((_0 as A).0: &mut u8) = move _7; // bb2[3]: scope 1 at src/lib.rs:9:39: 9:48
