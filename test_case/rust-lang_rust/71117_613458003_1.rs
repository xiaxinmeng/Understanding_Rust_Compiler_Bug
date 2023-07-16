
fn  foo(_1: u32) -> () {
    debug x => _1;                       // in scope 0 at src/main.rs:1:8: 1:13
    let mut _0: ();                      // return place in scope 0 at src/main.rs:1:20: 1:20
    let mut _2: (u32, bool);             // in scope 0 at src/main.rs:2:5: 2:11

    bb0: {
        _2 = CheckedAdd(_1, const 1u32); // bb0[0]: scope 0 at src/main.rs:2:5: 2:11
        assert(!move (_2.1: bool), "attempt to add with overflow") -> bb1; // bb0[1]: scope 0 at src/main.rs:2:5: 2:11
    }

    bb1: {
        _1 = move (_2.0: u32);           // bb1[0]: scope 0 at src/main.rs:2:5: 2:11
        return;                          // bb1[1]: scope 0 at src/main.rs:3:2: 3:2
    }
}
