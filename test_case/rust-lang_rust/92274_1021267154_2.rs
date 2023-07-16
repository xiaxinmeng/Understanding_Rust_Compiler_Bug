
// MIR for `is_const_eval` after ReplaceConstEvalSelect

fn is_const_eval() -> bool {
    let mut _0: bool;                    // return place in scope 0 at src/test/ui/intrinsics/const-eval-select.rs:16:29: 16:33
    let mut _1: ();                      // in scope 0 at src/test/ui/intrinsics/const-eval-select.rs:17:32: 17:34
    scope 1 {
    }

    bb0: {
        StorageLive(_1);                 // scope 1 at src/test/ui/intrinsics/const-eval-select.rs:17:32: 17:34
        _0 = no() -> bb1;                // scope 1 at src/test/ui/intrinsics/const-eval-select.rs:17:14: 17:44
                                         // mir::Constant
                                         // + span: src/test/ui/intrinsics/const-eval-select.rs:17:41: 17:43
                                         // + literal: Const { ty: fn() -> bool {no}, val: Value(Scalar(<ZST>)) }
    }

    bb1: {
        StorageDead(_1);                 // scope 1 at src/test/ui/intrinsics/const-eval-select.rs:17:43: 17:44
        return;                          // scope 0 at src/test/ui/intrinsics/const-eval-select.rs:18:2: 18:2
    }
}
