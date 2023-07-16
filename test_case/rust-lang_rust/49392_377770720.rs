
        _3 = Len((*_1));                 // bb0[3]: scope 1 at borrowck-use-in-index-lvalue.rs:16:5: 16:9
                                         | Live variables on entry to bb0[4]: [_1, _2, _3]
        _4 = Lt(_2, _3);                 // bb0[4]: scope 1 at borrowck-use-in-index-lvalue.rs:16:5: 16:9
                                         | Live variables on entry to bb0[5]: [_1, _2, _3, _4]
        assert(move _4, "index out of bounds: the len is {} but the index is {}", move _3, _2) -> [success: bb2, unwind: bb1]; // bb0[5]: scope 1 at borrowck-use-in-index-lvalue.rs:16:5: 16:9
    }

    | Live variables on entry to bb1: []
    bb1: {                               // cleanup
                                         | Live variables on entry to bb1[0]: []
        resume;                          // bb1[0]: scope 0 at borrowck-use-in-index-lvalue.rs:14:1: 22:2
    }

    | Live variables on entry to bb2: [_1, _2]
    bb2: {                              
                                         | Live variables on entry to bb2[0]: [_1, _2]
        (*_1)[_2] = const 0isize;  
