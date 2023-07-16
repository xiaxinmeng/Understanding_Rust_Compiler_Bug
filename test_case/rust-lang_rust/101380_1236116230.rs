rust
fn Foo::test1(_1: &Self, _2: F) -> T {
    debug self => _1;                    // in scope 0 at [src/lib.rs:13:34: 13:39](https://play.rust-lang.org/#)
    debug f => _2;                       // in scope 0 at [src/lib.rs:13:41: 13:46](https://play.rust-lang.org/#)
    let mut _0: T;                       // return place in scope 0 at [src/lib.rs:13:54: 13:55](https://play.rust-lang.org/#)
    let mut _3: &Self;                   // in scope 0 at [src/lib.rs:14:9: 14:25](https://play.rust-lang.org/#)
    let mut _4: &mut dyn std::ops::FnMut() -> T; // in scope 0 at [src/lib.rs:14:18: 14:24](https://play.rust-lang.org/#)
    let mut _5: &mut F;                  // in scope 0 at [src/lib.rs:14:18: 14:24](https://play.rust-lang.org/#)
    let mut _6: &mut F;                  // in scope 0 at [src/lib.rs:14:18: 14:24](https://play.rust-lang.org/#)

    bb0: {
        _3 = _1;                         // scope 0 at [src/lib.rs:14:9: 14:25](https://play.rust-lang.org/#)
        _6 = &mut _2;                    // scope 0 at [src/lib.rs:14:18: 14:24](https://play.rust-lang.org/#)
        _5 = &mut (*_6);                 // scope 0 at [src/lib.rs:14:18: 14:24](https://play.rust-lang.org/#)
        _4 = move _5 as &mut dyn std::ops::FnMut() -> T (Pointer(Unsize)); // scope 0 at [src/lib.rs:14:18: 14:24](https://play.rust-lang.org/#)
        _0 = <Self as Foo>::foo::<T>(move _3, move _4) -> [return: bb1, unwind: bb3]; // scope 0 at [src/lib.rs:14:9: 14:25](https://play.rust-lang.org/#)
                                         // mir::Constant
                                         // + span: [src/lib.rs:14:14: 14:17](https://play.rust-lang.org/#)
                                         // + literal: Const { ty: for<'r, 's> fn(&'r Self, &'s mut (dyn FnMut() -> T + 's)) -> T {<Self as Foo>::foo::<T>}, val: Value(Scalar(<ZST>)) }
    }

    bb1: {
        drop(_2) -> bb2;                 // scope 0 at [src/lib.rs:15:5: 15:6](https://play.rust-lang.org/#)
    }

    bb2: {
        return;                          // scope 0 at [src/lib.rs:15:6: 15:6](https://play.rust-lang.org/#)
    }

    bb3 (cleanup): {
        drop(_2) -> bb4;                 // scope 0 at [src/lib.rs:15:5: 15:6](https://play.rust-lang.org/#)
    }

    bb4 (cleanup): {
        resume;                          // scope 0 at [src/lib.rs:13:5: 15:6](https://play.rust-lang.org/#)
    }
}

fn Foo::test2(_1: &Self, _2: F) -> T {
    debug self => _1;                    // in scope 0 at [src/lib.rs:17:44: 17:49](https://play.rust-lang.org/#)
    debug f => _2;                       // in scope 0 at [src/lib.rs:17:51: 17:56](https://play.rust-lang.org/#)
    let mut _0: T;                       // return place in scope 0 at [src/lib.rs:17:64: 17:65](https://play.rust-lang.org/#)
    let mut _3: &Self;                   // in scope 0 at [src/lib.rs:18:9: 18:25](https://play.rust-lang.org/#)
    let mut _4: &mut dyn std::ops::FnMut() -> T; // in scope 0 at [src/lib.rs:18:18: 18:24](https://play.rust-lang.org/#)
    let mut _5: &mut F;                  // in scope 0 at [src/lib.rs:18:18: 18:24](https://play.rust-lang.org/#)
    let mut _6: &mut F;                  // in scope 0 at [src/lib.rs:18:18: 18:24](https://play.rust-lang.org/#)

    bb0: {
        _3 = _1;                         // scope 0 at [src/lib.rs:18:9: 18:25](https://play.rust-lang.org/#)
        _6 = &mut _2;                    // scope 0 at [src/lib.rs:18:18: 18:24](https://play.rust-lang.org/#)
        _5 = &mut (*_6);                 // scope 0 at [src/lib.rs:18:18: 18:24](https://play.rust-lang.org/#)
        _4 = move _5 as &mut dyn std::ops::FnMut() -> T (Pointer(Unsize)); // scope 0 at [src/lib.rs:18:18: 18:24](https://play.rust-lang.org/#)
        _0 = <Self as Foo>::bar::<T>(move _3, move _4) -> [return: bb1, unwind: bb3]; // scope 0 at [src/lib.rs:18:9: 18:25](https://play.rust-lang.org/#)
                                         // mir::Constant
                                         // + span: [src/lib.rs:18:14: 18:17](https://play.rust-lang.org/#)
                                         // + literal: Const { ty: for<'r, 's> fn(&'r Self, &'s mut (dyn FnMut() -> T + 'static)) -> T {<Self as Foo>::bar::<T>}, val: Value(Scalar(<ZST>)) }
    }

    bb1: {
        drop(_2) -> bb2;                 // scope 0 at [src/lib.rs:19:5: 19:6](https://play.rust-lang.org/#)
    }

    bb2: {
        return;                          // scope 0 at [src/lib.rs:19:6: 19:6](https://play.rust-lang.org/#)
    }

    bb3 (cleanup): {
        drop(_2) -> bb4;                 // scope 0 at [src/lib.rs:19:5: 19:6](https://play.rust-lang.org/#)
    }

    bb4 (cleanup): {
        resume;                          // scope 0 at [src/lib.rs:17:5: 19:6](https://play.rust-lang.org/#)
    }
}
