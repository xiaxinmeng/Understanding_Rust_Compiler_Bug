
    bb0: {
        StorageLive(_1);                 // scope 0 at main.rs:2:10: 2:11
        _0 = const ();                   // scope 0 at main.rs:1:11: 3:2
                                         // ty::Const
                                         // + ty: ()
                                         // + val: Value(Scalar(<ZST>))
                                         // mir::Constant
                                         // + span: main.rs:1:11: 3:2
                                         // + literal: Const { ty: (), val: Value(Scalar(<ZST>)) }
        drop(_1) -> bb1;                 // scope 0 at main.rs:3:1: 3:2
    }
