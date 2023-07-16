
fn use_unsized_arg(_1: dyn Unsized) -> () {
    debug arg => _1;                     // in scope 0 at [src/lib.rs:16:20: 16:23](https://play.rust-lang.org/?version=nightly&mode=debug&edition=2021#)
    let mut _0: ();                      // return place in scope 0 at [src/lib.rs:16:38: 16:38](https://play.rust-lang.org/?version=nightly&mode=debug&edition=2021#)
    let _2: ();                          // in scope 0 at [src/lib.rs:17:5: 17:26](https://play.rust-lang.org/?version=nightly&mode=debug&edition=2021#)
    let mut _3: dyn Unsized;             // in scope 0 at [src/lib.rs:17:22: 17:25](https://play.rust-lang.org/?version=nightly&mode=debug&edition=2021#)

    bb0: {
        _3 = move _1;                    // <----- scope 0 at [src/lib.rs:17:22: 17:25](https://play.rust-lang.org/?version=nightly&mode=debug&edition=2021#)
        _2 = take_unsized_arg(move _3) -> bb1; // scope 0 at [src/lib.rs:17:5: 17:26](https://play.rust-lang.org/?version=nightly&mode=debug&edition=2021#)
                                         // mir::Constant
                                         // + span: [src/lib.rs:17:5: 17:21](https://play.rust-lang.org/?version=nightly&mode=debug&edition=2021#)
                                         // + literal: Const { ty: fn((dyn Unsized + 'static)) {take_unsized_arg}, val: Value(<ZST>) }
    }

    bb1: {
        return;                          // scope 0 at [src/lib.rs:18:2: 18:2](https://play.rust-lang.org/?version=nightly&mode=debug&edition=2021#)
    }
}
