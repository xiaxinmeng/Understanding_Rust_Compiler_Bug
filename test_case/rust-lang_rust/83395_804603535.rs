rust
_4 = move _5 as &dyn Pin (Pointer(Unsize)); // scope 1 at src/main.rs:16:22: 16:25
StorageDead(_5);                 // scope 1 at src/main.rs:16:24: 16:25
_3 = <dyn Pin as Configure>::make_output(move _4) -> bb1; // scope 1 at src/main.rs:16:5: 16:26
