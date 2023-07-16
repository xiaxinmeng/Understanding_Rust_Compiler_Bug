rust
// MIR for `std::ops::FnOnce::call_once` before AddMovesForPackedDrops

fn std::ops::FnOnce::call_once(_1: Self, _2: Args) -> <Self as std::ops::FnOnce<Args>>::Output {
    let mut _0: <Self as std::ops::FnOnce<Args>>::Output;
    let _3: &mut Self;

    bb0: {
        _3 = &mut _1;
        _0 = const <Self as std::ops::FnMut<Args>>::call_mut(move _3, move _2) -> [return: bb1, unwind: bb3];
                                         // ty::Const
                                         // + ty: for<'r> extern "rust-call" fn(&'r mut Self, Args) -> <Self as std::ops::FnOnce<Args>>::Output {<Self as std::ops::FnMut<Args>>::call_mut}
                                         // + val: Value(Scalar(<ZST>))
                                         // mir::Constant
                                         // + span: ~/.rustup/toolchains/nightly-2020-06-12-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/src/libcore/ops/function.rs:232:5: 232:71
                                         // + literal: Const { ty: for<'r> extern "rust-call" fn(&'r mut Self, Args) -> <Self as std::ops::FnOnce<Args>>::Output {<Self as std::ops::FnMut<Args>>::call_mut}, val: Value(Scalar(<ZST>)) }
    }

    bb1: {
        drop(_1) -> bb2;
    }

    bb2: {
        return;
    }

    bb3 (cleanup): {
        drop(_1) -> bb4;
    }

    bb4 (cleanup): {
        resume;
    }
}
