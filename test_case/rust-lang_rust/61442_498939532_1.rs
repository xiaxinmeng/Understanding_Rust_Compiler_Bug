
   bb2: {
        StorageLive(_3);                 // bb2[0]: scope 1 at src/main.rs:16:5: 16:6
        _3 = &mut _1;                    // bb2[1]: scope 1 at src/main.rs:16:5: 16:6
        StorageLive(_4);                 // bb2[2]: scope 1 at src/main.rs:16:10: 16:32
        StorageLive(_5);                 // bb2[3]: scope 1 at src/main.rs:16:16: 16:17
        _5 = const "Hello";              // bb2[4]: scope 1 at src/main.rs:16:20: 16:27
                                         // ty::Const
                                         // + ty: &str
                                         // + val: Slice(Ptr(Pointer { alloc_id: AllocId(0), offset: Size { raw: 0 }, tag: () }), 5)
                                         // mir::Constant
                                         // + span: src/main.rs:16:20: 16:27
                                         // + ty: &str
                                         // + literal: Const { ty: &str, val: Slice(Ptr(Pointer { alloc_id: AllocId(0), offset: Size { raw: 0 }, tag: () }), 5) }
        _4 = _5;                         // bb2[5]: scope 3 at src/main.rs:16:29: 16:30
        StorageDead(_5);                 // bb2[6]: scope 1 at src/main.rs:16:31: 16:32
        _2 = const std::ops::AddAssign::add_assign(move _3, move _4) -> [return: bb3, unwind: bb4]; // bb2[7]: scope 1 at src/main.rs:16:5: 16:32
                                         // ty::Const
                                         // + ty: for<'r> fn(&'r mut std::string::String, &str) {<std::string::String as std::ops::AddAssign<&str>>::add_assign}
                                         // + val: Scalar(Bits { size: 0, bits: 0 })
                                         // mir::Constant
                                         // + span: src/main.rs:16:5: 16:32
                                         // + ty: for<'r> fn(&'r mut std::string::String, &str) {<std::string::String as std::ops::AddAssign<&str>>::add_assign}
                                         // + literal: Const { ty: for<'r> fn(&'r mut std::string::String, &str) {<std::string::String as std::ops::AddAssign<&str>>::add_assign}, val: Scalar(Bits { size: 0, bits: 0 }) }
    }
