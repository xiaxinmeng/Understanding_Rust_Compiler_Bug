rust

        _1 = BTreeSet::<()>::new() -> bb1; // scope 0 at src/main.rs:2:5: 2:44
                                         // mir::Constant
                                         // + span: src/main.rs:2:5: 2:42
                                         // + user_ty: UserType(0)
                                         // + literal: Const { ty: fn() -> std::collections::BTreeSet<()> {std::collections::BTreeSet::<()>::new}, val: Value(Scalar(<ZST>)) }
