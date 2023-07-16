rust
fn root() -> u32 {
    let mut _0: u32;                     // return place in scope 0 at src/lib.rs:6:18: 6:21
    scope 1 (inlined inline_me) {        // at src/lib.rs:7:5: 7:16
    }

    bb0: {
        _0 = const 42_u32;               // scope 1 at src/lib.rs:3:5: 3:7
        return;                          // scope 0 at src/lib.rs:8:2: 8:2
    }
}
