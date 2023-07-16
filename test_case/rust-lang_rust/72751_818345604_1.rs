rust
// WARNING: This output format is intended for human consumers only
// and is subject to change without notice. Knock yourself out.
fn foo(_1: bool) -> bool {
    debug x => _1;                       // in scope 0 at src/lib.rs:1:12: 1:13
    let mut _0: bool;                    // return place in scope 0 at src/lib.rs:1:24: 1:28

    bb0: {
        _0 = const false;                // scope 0 at src/lib.rs:2:5: 2:15
        return;                          // scope 0 at src/lib.rs:3:2: 3:2
    }
}
