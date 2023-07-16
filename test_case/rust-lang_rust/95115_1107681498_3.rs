rust
        FakeRead(ForLet(None), _1);      // scope 0 at test.rs:6:9: 6:10
        StorageDead(_4);                 // scope 0 at test.rs:6:38: 6:39
        StorageLive(_5);                 // scope 1 at test.rs:8:9: 8:10
        _5 = move (_1.0: std::string::String); // scope 1 at test.rs:8:13: 8:16
        FakeRead(ForLet(None), _5);      // scope 1 at test.rs:8:9: 8:10
