rust
fn <impl at [src/lib.rs:8:1: 8:12](https://play.rust-lang.org/?version=nightly&mode=release&edition=2021#)>::try_from_matched(_1: [u8; 4]) -> Result<MyEnum, ()> {
    debug value => _1;                   // in scope 0 at [src/lib.rs:9:29: 9:34](https://play.rust-lang.org/?version=nightly&mode=release&edition=2021#)
    let mut _0: std::result::Result<MyEnum, ()>; // return place in scope 0 at [src/lib.rs:9:48: 9:64](https://play.rust-lang.org/?version=nightly&mode=release&edition=2021#)
    let mut _2: &[u8; 4];                // in scope 0 at [src/lib.rs:10:15: 10:21](https://play.rust-lang.org/?version=nightly&mode=release&edition=2021#)

    bb0: {
        StorageLive(_2);                 // scope 0 at [src/lib.rs:10:15: 10:21](https://play.rust-lang.org/?version=nightly&mode=release&edition=2021#)
        _2 = &_1;                        // scope 0 at [src/lib.rs:10:15: 10:21](https://play.rust-lang.org/?version=nightly&mode=release&edition=2021#)
        switchInt((*_2)[0 of 4]) -> [65_u8: bb1, 69_u8: bb5, 73_u8: bb8, 77_u8: bb11, otherwise: bb4]; // scope 0 at [src/lib.rs:10:9: 10:21](https://play.rust-lang.org/?version=nightly&mode=release&edition=2021#)
    }

    bb1: {
        switchInt((*_2)[1 of 4]) -> [66_u8: bb2, otherwise: bb4]; // scope 0 at [src/lib.rs:10:9: 10:21](https://play.rust-lang.org/?version=nightly&mode=release&edition=2021#)
    }

    bb2: {
        switchInt((*_2)[2 of 4]) -> [67_u8: bb3, otherwise: bb4]; // scope 0 at [src/lib.rs:10:9: 10:21](https://play.rust-lang.org/?version=nightly&mode=release&edition=2021#)
    }

    bb3: {
        switchInt((*_2)[3 of 4]) -> [68_u8: bb14, otherwise: bb4]; // scope 0 at [src/lib.rs:10:9: 10:21](https://play.rust-lang.org/?version=nightly&mode=release&edition=2021#)
    }
