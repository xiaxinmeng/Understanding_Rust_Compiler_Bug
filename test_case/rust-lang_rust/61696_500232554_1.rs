rust
// WARNING: This output format is intended for human consumers only
// and is subject to change without notice. Knock yourself out.
fn  main() -> () {
    let mut _0: ();                      // return place in scope 0 at src/main.rs:10:11: 10:11
    let mut _1: E1;                      // in scope 0 at src/main.rs:11:11: 11:31
    let mut _2: isize;                   // in scope 0 at src/main.rs:12:9: 12:22
    scope 1 {
    }

    bb0: {
        StorageLive(_1);                 // bb0[0]: scope 0 at src/main.rs:11:11: 11:31
        ((_1 as V1).0: bool) = const true; // bb0[1]: scope 0 at src/main.rs:11:11: 11:31
                                         // ty::Const
                                         // + ty: bool
                                         // + val: Scalar(0x01)
                                         // mir::Constant
                                         // + span: src/main.rs:11:24: 11:28
                                         // + ty: bool
                                         // + literal: Const { ty: bool, val: Scalar(0x01) }
        discriminant(_1) = 0;            // bb0[2]: scope 0 at src/main.rs:11:11: 11:31
        _2 = discriminant(_1);           // bb0[3]: scope 0 at src/main.rs:12:9: 12:22
        switchInt(move _2) -> [1isize: bb1, otherwise: bb2]; // bb0[4]: scope 0 at src/main.rs:12:9: 12:22
    }

    bb1: {
        const std::intrinsics::unreachable(); // bb1[0]: scope 1 at src/main.rs:12:35: 12:68
                                         // ty::Const
                                         // + ty: unsafe extern "rust-intrinsic" fn() -> ! {std::intrinsics::unreachable}
                                         // + val: Scalar(<ZST>)
                                         // mir::Constant
                                         // + span: src/main.rs:12:35: 12:66
                                         // + ty: unsafe extern "rust-intrinsic" fn() -> ! {std::intrinsics::unreachable}
                                         // + literal: Const { ty: unsafe extern "rust-intrinsic" fn() -> ! {std::intrinsics::unreachable}, val: Scalar(<ZST>) }
    }

    bb2: {
        StorageDead(_1);                 // bb2[0]: scope 0 at src/main.rs:15:1: 15:2
        return;                          // bb2[1]: scope 0 at src/main.rs:15:2: 15:2
    }
}
