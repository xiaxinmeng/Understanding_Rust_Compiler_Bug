rust
        _20 = &_0;                       // scope 1 at mis.rs:26:39: 26:43
        nop;                             // scope 1 at mis.rs:26:39: 26:43
        StorageLive(_22);                // scope 1 at mis.rs:26:45: 26:46
        _22 = _2;                        // scope 1 at mis.rs:26:45: 26:46
        StorageLive(_23);                // scope 1 at mis.rs:26:48: 26:49
        _23 = _2;                        // scope 1 at mis.rs:26:48: 26:49
        StorageLive(_24);                // scope 1 at mis.rs:26:51: 26:52
        _24 = _2;                        // scope 1 at mis.rs:26:51: 26:52
        _0 = matrix_prod(move _18, move _20, move _22, move _23, move _24) -> bb6; // scope 1 at mis.rs:26:23: 26:53
                                         // mir::Constant
                                         // + span: mis.rs:26:23: 26:34
                                         // + literal: Const { ty: for<'r, 's> fn(&'r [[i64; _]; _], &'s [[i64; _]; _], usize, usize, usize) -> [[i64; _]; _] {main::matrix_prod}, val: Value(Scalar(<ZST>)) }
