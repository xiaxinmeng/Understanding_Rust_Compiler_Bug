
static mut  FOO: *const &'static i32 = {
    let mut _0: *const &'static i32;     // return place in scope 0 at /home/oli/rustc/rust6/src/test/mir-opt/const-promotion-extern-static.rs:9:17: 9:36
    let mut _1: &[&'static i32];         // in scope 0 at /home/oli/rustc/rust6/src/test/mir-opt/const-promotion-extern-static.rs:9:39: 9:54
    let mut _2: &[&'static i32; 1];      // in scope 0 at /home/oli/rustc/rust6/src/test/mir-opt/const-promotion-extern-static.rs:9:39: 9:54
    let _3: [&'static i32; 1];           // in scope 0 at /home/oli/rustc/rust6/src/test/mir-opt/const-promotion-extern-static.rs:9:39: 9:54
    let mut _4: &'static i32;            // in scope 0 at /home/oli/rustc/rust6/src/test/mir-opt/const-promotion-extern-static.rs:9:40: 9:53
    let _5: *const i32;                  // in scope 0 at /home/oli/rustc/rust6/src/test/mir-opt/const-promotion-extern-static.rs:9:50: 9:51
    scope 1 {
    }

    bb0: {
        StorageLive(_1);                 // bb0[0]: scope 0 at /home/oli/rustc/rust6/src/test/mir-opt/const-promotion-extern-static.rs:9:39: 9:54
        StorageLive(_2);                 // bb0[1]: scope 0 at /home/oli/rustc/rust6/src/test/mir-opt/const-promotion-extern-static.rs:9:39: 9:54
        StorageLive(_3);                 // bb0[2]: scope 0 at /home/oli/rustc/rust6/src/test/mir-opt/const-promotion-extern-static.rs:9:39: 9:54
        StorageLive(_4);                 // bb0[3]: scope 0 at /home/oli/rustc/rust6/src/test/mir-opt/const-promotion-extern-static.rs:9:40: 9:53
        StorageLive(_5);                 // bb0[4]: scope 1 at /home/oli/rustc/rust6/src/test/mir-opt/const-promotion-extern-static.rs:9:50: 9:51
        _5 = const Value(Scalar(AllocId(1).0x0)) : *const i32; // bb0[5]: scope 1 at /home/oli/rustc/rust6/src/test/mir-opt/const-promotion-extern-static.rs:9:50: 9:51
                                         // ty::Const
                                         // + ty: *const i32
                                         // + val: Value(Scalar(AllocId(1).0x0))
                                         // mir::Constant
                                         // + span: /home/oli/rustc/rust6/src/test/mir-opt/const-promotion-extern-static.rs:9:50: 9:51
                                         // + literal: Const { ty: *const i32, val: Value(Scalar(AllocId(1).0x0)) }
        _4 = &(*_5);                     // bb0[6]: scope 1 at /home/oli/rustc/rust6/src/test/mir-opt/const-promotion-extern-static.rs:9:49: 9:51
        _3 = [move _4];                  // bb0[7]: scope 0 at /home/oli/rustc/rust6/src/test/mir-opt/const-promotion-extern-static.rs:9:39: 9:54
        _2 = &_3;                        // bb0[8]: scope 0 at /home/oli/rustc/rust6/src/test/mir-opt/const-promotion-extern-static.rs:9:39: 9:54
        _1 = move _2 as &[&'static i32] (Pointer(Unsize)); // bb0[9]: scope 0 at /home/oli/rustc/rust6/src/test/mir-opt/const-promotion-extern-static.rs:9:39: 9:54
        _0 = const core::slice::<impl [&'static i32]>::as_ptr(move _1) -> [return: bb2, unwind: bb1]; // bb0[10]: scope 0 at /home/oli/rustc/rust6/src/test/mir-opt/const-promotion-extern-static.rs:9:39: 9:63
                                         // ty::Const
                                         // + ty: for<'r> fn(&'r [&'static i32]) -> *const &'static i32 {core::slice::<impl [&'static i32]>::as_ptr}
                                         // + val: Value(Scalar(<ZST>))
                                         // mir::Constant
                                         // + span: /home/oli/rustc/rust6/src/test/mir-opt/const-promotion-extern-static.rs:9:55: 9:61
                                         // + literal: Const { ty: for<'r> fn(&'r [&'static i32]) -> *const &'static i32 {core::slice::<impl [&'static i32]>::as_ptr}, val: Value(Scalar(<ZST>)) }
    }

    bb1 (cleanup): {
        resume;                          // bb1[0]: scope 0 at /home/oli/rustc/rust6/src/test/mir-opt/const-promotion-extern-static.rs:9:1: 9:64
    }

    bb2: {
        StorageDead(_5);                 // bb2[0]: scope 0 at /home/oli/rustc/rust6/src/test/mir-opt/const-promotion-extern-static.rs:9:62: 9:63
        StorageDead(_3);                 // bb2[1]: scope 0 at /home/oli/rustc/rust6/src/test/mir-opt/const-promotion-extern-static.rs:9:62: 9:63
        return;                          // bb2[2]: scope 0 at /home/oli/rustc/rust6/src/test/mir-opt/const-promotion-extern-static.rs:9:1: 9:64
    }
}
