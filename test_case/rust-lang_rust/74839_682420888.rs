
// MIR for `check_foo2` after PreCodegen

fn check_foo2() -> u64 {
    let mut _0: u64;                     // return place in scope 0 at /home/alarsyo/work/rust/src/test/mir-opt/my_test.rs:14:20: 14:23

    bb0: {
        _0 = foo2(const 100000_u64) -> bb1; // scope 0 at /home/alarsyo/work/rust/src/test/mir-opt/my_test.rs:15:5: 15:17
                                         // mir::Constant
                                         // + span: /home/alarsyo/work/rust/src/test/mir-opt/my_test.rs:15:5: 15:9
                                         // + literal: Const { ty: fn(u64) -> u64 {foo2}, val: Value(Scalar(<ZST>)) }
    }

    bb1: {
        return;                          // scope 0 at /home/alarsyo/work/rust/src/test/mir-opt/my_test.rs:16:2: 16:2
    }
}
