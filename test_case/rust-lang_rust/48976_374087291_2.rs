
    | Live variables on entry to bb17: [_1 (drop), _2, _8, _12]
    bb17: {                             
                                         | Live variables on entry to bb17[0]: [_1 (drop), _2, _8, _12]
        StorageLive(_16);                // bb17[0]: scope 6 at bug.rs:15:9: 15:10
                                         | Live variables on entry to bb17[1]: [_1 (drop), _2, _8, _12]
        _16 = ((_12 as Some).0: usize);  // bb17[1]: scope 6 at bug.rs:15:9: 15:10
                                         | Live variables on entry to bb17[2]: [_1 (drop), _2, _8, _16]
        StorageLive(_18);                // bb17[2]: scope 8 at bug.rs:15:9: 15:10
                                         | Live variables on entry to bb17[3]: [_1 (drop), _2, _8, _16]
        _18 = _16;                       // bb17[3]: scope 8 at bug.rs:15:9: 15:10
                                         | Live variables on entry to bb17[4]: [_1 (drop), _2, _8, _18]
        _10 = move _18;                  // bb17[4]: scope 8 at bug.rs:15:9: 15:10
                                         | Live variables on entry to bb17[5]: [_1 (drop), _2, _8, _10]
        _11 = ();                        // bb17[5]: scope 8 at bug.rs:15:9: 15:10
                                         | Live variables on entry to bb17[6]: [_1 (drop), _2, _8, _10]
        StorageDead(_18);                // bb17[6]: scope 8 at bug.rs:15:9: 15:10
                                         | Live variables on entry to bb17[7]: [_1 (drop), _2, _8, _10]
        StorageDead(_16);                // bb17[7]: scope 6 at bug.rs:15:18: 15:19
                                         | Live variables on entry to bb17[8]: [_1 (drop), _2, _8, _10]
        StorageDead(_12);                // bb17[8]: scope 6 at bug.rs:15:18: 15:19
                                         | Live variables on entry to bb17[9]: [_1 (drop), _2, _8, _10]
        StorageDead(_14);                // bb17[9]: scope 6 at bug.rs:15:18: 15:19
                                         | Live variables on entry to bb17[10]: [_1 (drop), _2, _8, _10]
        StorageLive(_20);                // bb17[10]: scope 6 at bug.rs:15:9: 15:10
                                         | Live variables on entry to bb17[11]: [_1 (drop), _2, _8, _10]
        StorageLive(_21);                // bb17[11]: scope 6 at bug.rs:15:14: 15:19
                                         | Live variables on entry to bb17[12]: [_1 (drop), _2, _8, _10]
        _21 = _10;                       // bb17[12]: scope 6 at bug.rs:15:14: 15:19
                                         | Live variables on entry to bb17[13]: [_1 (drop), _2, _8, _21]
        _20 = move _21;                  // bb17[13]: scope 6 at bug.rs:15:14: 15:19
                                         | Live variables on entry to bb17[14]: [_1 (drop), _2, _8, _20]
        StorageDead(_21);                // bb17[14]: scope 6 at bug.rs:15:18: 15:19
                                         | Live variables on entry to bb17[15]: [_1 (drop), _2, _8, _20]
        StorageLive(_23);                // bb17[15]: scope 9 at bug.rs:23:13: 23:14
                                         | Live variables on entry to bb17[16]: [_1 (drop), _2, _8, _20]
        StorageLive(_24);                // bb17[16]: scope 9 at bug.rs:23:22: 23:24
                                         | Live variables on entry to bb17[17]: [_1 (drop), _2, _8, _20]
        StorageLive(_25);                // bb17[17]: scope 9 at bug.rs:23:22: 23:24
                                         | Live variables on entry to bb17[18]: [_1 (drop), _2, _8, _20]
        _25 = &mut _2;                   // bb17[18]: scope 9 at bug.rs:23:22: 23:24
                                         | Live variables on entry to bb17[19]: [_1 (drop), _2, _8, _20, _25]
        _24 = const std::ops::DerefMut::deref_mut(move _25) -> [return: bb18, unwind: bb19]; // bb17[19]: scope 9 at bug.rs:23:22: 23:24
                                         // ty::Const
                                         // + ty: for<'r> fn(&'r mut std::cell::RefMut<'_, SharedState>) -> &'r mut <std::cell::RefMut<'_, SharedState> as std::ops::Deref>::Target {<std::cell::RefMut<'_, SharedState> as std::ops::DerefMut>::deref_mut}
                                         // + val: Value(ByVal(Undef))
                                         // mir::Constant
                                         // + span: bug.rs:23:22: 23:24
                                         // + ty: for<'r> fn(&'r mut std::cell::RefMut<'_, SharedState>) -> &'r mut <std::cell::RefMut<'_, SharedState> as std::ops::Deref>::Target {<std::cell::RefMut<'_, SharedState> as std::ops::DerefMut>::deref_mut}
                                         // + literal: const std::ops::DerefMut::deref_mut
    }

    | Live variables on entry to bb18: [_1 (drop), _2, _8, _20, _24]
    bb18: {                             
                                         | Live variables on entry to bb18[0]: [_1 (drop), _2, _8, _20, _24]
        StorageDead(_25);                // bb18[0]: scope 9 at bug.rs:23:23: 23:24
                                         | Live variables on entry to bb18[1]: [_1 (drop), _2, _8, _20, _24]
        _23 = &mut ((*_24).0: std::vec::Vec<usize>); // bb18[1]: scope 9 at bug.rs:23:17: 23:31
                                         | Live variables on entry to bb18[2]: [_1 (drop), _2, _8, _20, _23]
        StorageLive(_26);                // bb18[2]: scope 11 at bug.rs:24:13: 24:16
                                         | Live variables on entry to bb18[3]: [_1 (drop), _2, _8, _20, _23]
        StorageLive(_27);                // bb18[3]: scope 11 at bug.rs:24:19: 24:20
                                         | Live variables on entry to bb18[4]: [_1 (drop), _2, _8, _20, _23]
        _27 = &mut (*_23);               // bb18[4]: scope 11 at bug.rs:24:19: 24:20
                                         | Live variables on entry to bb18[5]: [_1 (drop), _2, _8, _20, _27]
        StorageLive(_28);                // bb18[5]: scope 11 at bug.rs:24:31: 24:41
                                         | Live variables on entry to bb18[6]: [_1 (drop), _2, _8, _20, _27]
        StorageLive(_29);                // bb18[6]: scope 11 at bug.rs:24:31: 24:33
                                         | Live variables on entry to bb18[7]: [_1 (drop), _2, _8, _20, _27]
        StorageLive(_30);                // bb18[7]: scope 11 at bug.rs:24:31: 24:33
                                         | Live variables on entry to bb18[8]: [_1 (drop), _2, _8, _20, _27]
        _30 = &_2;                       // bb18[8]: scope 11 at bug.rs:24:31: 24:33
                                         | Live variables on entry to bb18[9]: [_1 (drop), _2, _8, _20, _27, _30]
        _29 = const std::ops::Deref::deref(move _30) -> [return: bb20, unwind: bb22]; // bb18[9]: scope 11 at bug.rs:24:31: 24:33
                                         // ty::Const
                                         // + ty: for<'r> fn(&'r std::cell::RefMut<'_, SharedState>) -> &'r <std::cell::RefMut<'_, SharedState> as std::ops::Deref>::Target {<std::cell::RefMut<'_, SharedState> as std::ops::Deref>::deref}
                                         // + val: Value(ByVal(Undef))
                                         // mir::Constant
                                         // + span: bug.rs:24:31: 24:33
                                         // + ty: for<'r> fn(&'r std::cell::RefMut<'_, SharedState>) -> &'r <std::cell::RefMut<'_, SharedState> as std::ops::Deref>::Target {<std::cell::RefMut<'_, SharedState> as std::ops::Deref>::deref}
                                         // + literal: const std::ops::Deref::deref
    }

    | Live variables on entry to bb20: [_1 (drop), _2, _8, _20, _27, _29]
    bb20: {                             
                                         | Live variables on entry to bb20[0]: [_1 (drop), _2, _8, _20, _27, _29]
        nop;                             // bb20[0]: scope 11 at bug.rs:24:31: 24:33
                                         | Live variables on entry to bb20[1]: [_1 (drop), _2, _8, _20, _27, _29]
        StorageDead(_30);                // bb20[1]: scope 11 at bug.rs:24:32: 24:33
                                         | Live variables on entry to bb20[2]: [_1 (drop), _2, _8, _20, _27, _29]
        _28 = ((*_29).1: usize);         // bb20[2]: scope 11 at bug.rs:24:31: 24:41
                                         | Live variables on entry to bb20[3]: [_1 (drop), _2, _8, _20, _27, _28]
        _26 = const std::ops::IndexMut::index_mut(move _27, move _28) -> [return: bb23, unwind: bb21]; // bb20[3]: scope 11 at bug.rs:24:19: 24:42
                                         // ty::Const
                                         // + ty: for<'r> fn(&'r mut std::vec::Vec<usize>, usize) -> &'r mut <std::vec::Vec<usize> as std::ops::Index<usize>>::Output {<std::vec::Vec<usize> as std::ops::IndexMut<usize>>::index_mut}
                                         // + val: Value(ByVal(Undef))
                                         // mir::Constant
                                         // + span: bug.rs:24:19: 24:42
                                         // + ty: for<'r> fn(&'r mut std::vec::Vec<usize>, usize) -> &'r mut <std::vec::Vec<usize> as std::ops::Index<usize>>::Output {<std::vec::Vec<usize> as std::ops::IndexMut<usize>>::index_mut}
                                         // + literal: const std::ops::IndexMut::index_mut
    }
