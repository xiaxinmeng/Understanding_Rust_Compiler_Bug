
    bb9: {
        StorageDead(_46);                // bb9[0]: scope 2 at /home/nmatsakis/versioned/rust-6/src/test/run-pass/intrinsic-move-val.rs:35:37: 35:38
        StorageLive(_48);                // bb9[1]: scope 2 at /home/nmatsakis/versioned/rust-6/src/test/run-pass/intrinsic-move-val.rs:37:29: 37:42
        _48 = const rusti::::init() -> [return: bb10, unwind: bb3]; // bb9[2]: scope 2 at /home/nmatsakis/versioned/rust-6/src/test/run-pass/intrinsic-move-val.rs:37:29: 37:42
                                         // ty::Const
                                         // + ty: unsafe extern "rust-intrinsic" fn() -> std::boxed::Box<D> {rusti::::init::<std::boxed::Box<D>>}
                                         // + val: Scalar(Bits { size: 0, bits: 0 })
                                         // mir::Constant
                                         // + span: /home/nmatsakis/versioned/rust-6/src/test/run-pass/intrinsic-move-val.rs:37:29: 37:40
                                         // + ty: unsafe extern "rust-intrinsic" fn() -> std::boxed::Box<D> {rusti::::init::<std::boxed::Box<D>>}
                                         // + literal: Const { ty: unsafe extern "rust-intrinsic" fn() -> std::boxed::Box<D> {rusti::::init::<std::boxed::Box<D>>}, val: Scalar(Bits { size: 0, bits: 0 }) \
}
    }

    bb10: {
        StorageLive(_47);                // bb10[0]: scope 2 at /home/nmatsakis/versioned/rust-6/src/test/run-pass/intrinsic-move-val.rs:37:13: 37:18
        _47 = move _48;                  // bb10[1]: scope 2 at /home/nmatsakis/versioned/rust-6/src/test/run-pass/intrinsic-move-val.rs:37:13: 37:18
        StorageDead(_48);                // bb10[2]: scope 2 at /home/nmatsakis/versioned/rust-6/src/test/run-pass/intrinsic-move-val.rs:37:42: 37:43
