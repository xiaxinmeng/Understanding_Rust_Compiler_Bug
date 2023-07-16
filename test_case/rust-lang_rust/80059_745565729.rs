rust
// WARNING: This output format is intended for human consumers only
// and is subject to change without notice. Knock yourself out.
fn foo(_1: *const bool) -> () {
    debug ptr => _1;                     // in scope 0 at src/lib.rs:1:8: 1:11
    let mut _0: ();                      // return place in scope 0 at src/lib.rs:1:26: 1:26
    scope 1 {
    }

    bb0: {
        _0 = const ();                   // scope 0 at src/lib.rs:1:26: 3:2
        return;                          // scope 0 at src/lib.rs:3:2: 3:2
    }
}
