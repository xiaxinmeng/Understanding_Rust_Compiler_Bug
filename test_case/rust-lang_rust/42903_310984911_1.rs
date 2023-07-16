
fn main() -> () {
    let mut _0: ();                      // return pointer
    scope 1 {
        let mut _1: Wrapper;             // "wrapper" in scope 1 at nodep.rs:16:9: 16:20
    }
    let mut _2: DropHasLifetime;
    let mut _3: &();
    let mut _4: &();
    let mut _5: DropHasLifetime;
    let mut _6: DropHasLifetime;

    bb0: {
        _4 = &(main::STATIC: ());        // scope 0 at nodep.rs:16:47: 16:54
        _3 = _4;                         // scope 0 at nodep.rs:16:47: 16:54
        _2 = DropHasLifetime<'_>::{{constructor}}(_3,); // scope 0 at nodep.rs:16:31: 16:55
        _1 = Wrapper<'_>::{{constructor}}(_2,); // scope 0 at nodep.rs:16:23: 16:56
        _6 = (_1.0: DropHasLifetime<'_>); // scope 1 at nodep.rs:18:33: 18:42
        _5 = const move_and_return(_6) -> bb1; // scope 1 at nodep.rs:18:17: 18:43
    }

    bb1: {
        drop((_1.0: DropHasLifetime)) -> bb2; // scope 1 at nodep.rs:18:5: 18:14
    }

    bb2: {
        (_1.0: DropHasLifetime) = _5;    // scope 1 at nodep.rs:18:5: 18:14
        _0 = ();                         // scope 0 at nodep.rs:13:11: 19:2
        drop((_1.0: DropHasLifetime)) -> bb3; // scope 0 at nodep.rs:19:2: 19:2
    }

    bb3: {
        return;                          // scope 0 at nodep.rs:19:2: 19:2
    }
}
