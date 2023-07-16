
    bb14: {
        _5 = move _2;                    // bb14[3]: scope 0 at src/main.rs:13:5: 17:6
        _9 = &_3;                        // bb14[8]: scope 2 at src/main.rs:16:11: 16:17
        _8 = _9;                         // bb14[9]: scope 2 at src/main.rs:16:11: 16:17
        _11 = &_5;                       // bb14[12]: scope 2 at src/main.rs:16:19: 16:25
        _10 = _11;                       // bb14[13]: scope 2 at src/main.rs:16:19: 16:25
        _7 = const p(move _8, move _10) -> [return: bb3, unwind: bb7]; // bb14[14]: scope 2 at src/main.rs:16:9: 16:26
                                         // ty::Const
                                         // + ty: for<'r, 's> fn(&'r std::string::String, &'s std::string::String) -> std::string::String {p}
                                         // + val: Value(Scalar(<ZST>))
                                         // mir::Constant
                                         // + span: src/main.rs:16:9: 16:10
                                         // + literal: Const { ty: for<'r, 's> fn(&'r std::string::String, &'s std::string::String) -> std::string::String {p}, val: Value(Scalar(<ZST>)) }
    }
