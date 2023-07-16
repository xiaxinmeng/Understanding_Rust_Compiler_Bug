rust
    bb0: {
        StorageLive(_1);                 // bb0[0]: scope 0 at src/main.rs:3:36: 3:59
        StorageLive(_2);                 // bb0[1]: scope 0 at src/main.rs:3:36: 3:47
        StorageLive(_3);                 // bb0[2]: scope 0 at src/main.rs:3:37: 3:47
        _3 = const std::vec::Vec::<i32>::new() -> bb1; // bb0[3]: scope 0 at src/main.rs:3:37: 3:47
                                         // ty::Const
                                         // + ty: fn() -> std::vec::Vec<i32> {std::vec::Vec::<i32>::new}
                                         // + val: Value(Scalar(<ZST>))
                                         // mir::Constant
                                         // + span: src/main.rs:3:37: 3:45
                                         // + user_ty: UserType(0)
                                         // + literal: Const { ty: fn() -> std::vec::Vec<i32> {std::vec::Vec::<i32>::new}, val: Value(Scalar(<ZST>)) }
    }

    bb1: {
        _2 = &_3;                        // bb1[0]: scope 0 at src/main.rs:3:36: 3:47
        _1 = &raw const (*_2);           // bb1[1]: scope 0 at src/main.rs:3:36: 3:47
        _0 = _1;                         // bb1[2]: scope 0 at src/main.rs:3:36: 3:59
        StorageDead(_2);                 // bb1[3]: scope 0 at src/main.rs:3:58: 3:59
        StorageDead(_1);                 // bb1[4]: scope 0 at src/main.rs:3:58: 3:59
        return;                          // bb1[5]: scope 0 at src/main.rs:3:1: 3:60
    }
