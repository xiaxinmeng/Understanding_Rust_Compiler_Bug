rust
    bb3: {
        StorageLive(_4);                 // bb3[0]: scope 1 at ./example.rs:13:18: 13:19
        _4 = ((_1 as Foo).0: u8);        // bb3[1]: scope 1 at ./example.rs:13:18: 13:19
        StorageLive(_5);                 // bb3[2]: scope 3 at ./example.rs:13:33: 13:34
        _5 = _4;                         // bb3[3]: scope 3 at ./example.rs:13:33: 13:34
        ((_2 as Foo).0: u8) = move _5;   // bb3[4]: scope 3 at ./example.rs:13:24: 13:35
        discriminant(_2) = 0;            // bb3[5]: scope 3 at ./example.rs:13:24: 13:35
        StorageDead(_5);                 // bb3[6]: scope 3 at ./example.rs:13:34: 13:35
        StorageDead(_4);                 // bb3[7]: scope 1 at ./example.rs:13:35: 13:36
        goto -> bb4;                     // bb3[8]: scope 1 at ./example.rs:12:18: 15:6
    }
