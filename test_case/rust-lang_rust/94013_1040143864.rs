rust
// WARNING: This output format is intended for human consumers only
// and is subject to change without notice. Knock yourself out.
fn foo(_1: u8, _2: u8) -> u8 {
    debug a => _1;                       // in scope 0 at [src/lib.rs:1:8: 1:9](https://play.rust-lang.org/#)
    debug b => _2;                       // in scope 0 at [src/lib.rs:1:15: 1:16](https://play.rust-lang.org/#)
    let mut _0: u8;                      // return place in scope 0 at [src/lib.rs:1:25: 1:27](https://play.rust-lang.org/#)
    let mut _3: u8;                      // in scope 0 at [src/lib.rs:2:5: 2:6](https://play.rust-lang.org/#)
    let mut _4: u8;                      // in scope 0 at [src/lib.rs:2:9: 2:10](https://play.rust-lang.org/#)
    let mut _5: bool;                    // in scope 0 at [src/lib.rs:2:5: 2:10](https://play.rust-lang.org/#)

    bb0: {
        _3 = _1;                         // scope 0 at [src/lib.rs:2:5: 2:6](https://play.rust-lang.org/#)
        _4 = _2;                         // scope 0 at [src/lib.rs:2:9: 2:10](https://play.rust-lang.org/#)
        _5 = Eq(_4, const 0_u8);         // scope 0 at [src/lib.rs:2:5: 2:10](https://play.rust-lang.org/#)
        assert(!move _5, "attempt to divide `{}` by zero", _3) -> bb1; // scope 0 at [src/lib.rs:2:5: 2:10](https://play.rust-lang.org/#)
    }

    bb1: {
        _0 = Div(move _3, move _4);      // scope 0 at [src/lib.rs:2:5: 2:10](https://play.rust-lang.org/#)
        return;                          // scope 0 at [src/lib.rs:3:2: 3:2](https://play.rust-lang.org/#)
    }
}
