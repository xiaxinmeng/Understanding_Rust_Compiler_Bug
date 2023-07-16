rust
fn  foo() -> () {
    let mut _0: ();                      // return place in scope 0 at src/lib.rs:1:10: 1:10
    let _1: foo::NoCopy;                 // "x" in scope 0 at src/lib.rs:3:9: 3:10
    let _2: ();                          // in scope 0 at src/lib.rs:4:5: 4:12
    let mut _3: foo::NoCopy;             // in scope 0 at src/lib.rs:4:10: 4:11
    scope 1 {
        scope 2 {
        }
    }

    bb0: {
        StorageLive(_1);                 // bb0[0]: scope 0 at src/lib.rs:3:9: 3:10
        StorageLive(_2);                 // bb0[1]: scope 1 at src/lib.rs:4:5: 4:12
        StorageLive(_3);                 // bb0[2]: scope 1 at src/lib.rs:4:10: 4:11
        _3 = move _1;                    // bb0[3]: scope 1 at src/lib.rs:4:10: 4:11
        _2 = const std::mem::drop::<foo::NoCopy>(move _3) -> bb1; // bb0[4]: scope 1 at src/lib.rs:4:5: 4:12
                                         // ty::Const
                                         // + ty: fn(foo::NoCopy) {std::mem::drop::<foo::NoCopy>}
                                         // + val: Scalar(<ZST>)
                                         // mir::Constant
                                         // + span: src/lib.rs:4:5: 4:9
                                         // + literal: Const { ty: fn(foo::NoCopy) {std::mem::drop::<foo::NoCopy>}, val: Scalar(<ZST>) }
    }

    bb1: {
        StorageDead(_3);                 // bb1[0]: scope 1 at src/lib.rs:4:11: 4:12
        StorageDead(_2);                 // bb1[1]: scope 1 at src/lib.rs:4:12: 4:13
        StorageDead(_1);                 // bb1[2]: scope 0 at src/lib.rs:6:1: 6:2
        return;                          // bb1[3]: scope 0 at src/lib.rs:6:2: 6:2
    }
}
