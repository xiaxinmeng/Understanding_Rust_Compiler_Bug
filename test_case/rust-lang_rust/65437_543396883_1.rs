
fn  foo() -> bool {
    let mut _0: bool;                    // return place in scope 0 at src/lib.rs:1:13: 1:17

    bb0: {
        _0 = const false;                // bb0[0]: scope 0 at src/lib.rs:2:5: 2:10
                                         // ty::Const
                                         // + ty: bool
                                         // + val: Scalar(0x00)
                                         // mir::Constant
                                         // + span: src/lib.rs:2:5: 2:10
                                         // + ty: bool
                                         // + literal: Const { ty: bool, val: Scalar(0x00) }
        return;                          // bb0[1]: scope 0 at src/lib.rs:3:2: 3:2
    }
}
