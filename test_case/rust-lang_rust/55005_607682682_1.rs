rust
fn <impl at src/lib.rs:58:1: 71:2>::drop(_1: &mut Arc<T>) -> () {
    debug self => _1;                    // in scope 0 at src/lib.rs:60:13: 60:22
    let mut _0: ();                      // return place in scope 0 at src/lib.rs:60:24: 60:24
    let mut _2: bool;                    // in scope 0 at src/lib.rs:61:12: 61:58
    let mut _3: usize;                   // in scope 0 at src/lib.rs:61:12: 61:53
    let mut _4: &std::sync::atomic::AtomicUsize; // in scope 0 at src/lib.rs:61:12: 61:31
    // XXXXXX We care about _5, the reference to ArcInner
    let _5: &ArcInner<T>;                // in scope 0 at src/lib.rs:61:12: 61:24
    let mut _6: &Arc<T>;                 // in scope 0 at src/lib.rs:61:12: 61:16
    let mut _7: std::sync::atomic::Ordering; // in scope 0 at src/lib.rs:61:45: 61:52
    let _8: ();                          // in scope 0 at src/lib.rs:13:9: 13:31
    let mut _9: std::sync::atomic::Ordering; // in scope 0 at src/lib.rs:13:23: 13:30
    let _10: ();                         // in scope 0 at src/lib.rs:68:13: 68:29
    let mut _11: &mut Arc<T>;            // in scope 0 at src/lib.rs:68:13: 68:17
    scope 1 {
    }

    bb0: {
        StorageLive(_2);                 // bb0[0]: scope 0 at src/lib.rs:61:12: 61:58
        StorageLive(_3);                 // bb0[1]: scope 0 at src/lib.rs:61:12: 61:53
        StorageLive(_4);                 // bb0[2]: scope 0 at src/lib.rs:61:12: 61:31
        // XXXXXX _5 is marked live here
        StorageLive(_5);                 // bb0[3]: scope 0 at src/lib.rs:61:12: 61:24
        StorageLive(_6);                 // bb0[4]: scope 0 at src/lib.rs:61:12: 61:16
        _6 = _1;                         // bb0[5]: scope 0 at src/lib.rs:61:12: 61:16
        // XXXXXX _5 is created here
        _5 = const Arc::<T>::inner(move _6) -> bb1; // bb0[6]: scope 0 at src/lib.rs:61:12: 61:24
                                         // ty::Const
                                         // + ty: for<'r> fn(&'r Arc<T>) -> &'r ArcInner<T> {Arc::<T>::inner}
                                         // + val: Value(Scalar(<ZST>))
                                         // mir::Constant
                                         // + span: src/lib.rs:61:17: 61:22
                                         // + literal: Const { ty: for<'r> fn(&'r Arc<T>) -> &'r ArcInner<T> {Arc::<T>::inner}, val: Value(Scalar(<ZST>)) }
    }

    bb1: {
        StorageDead(_6);                 // bb1[0]: scope 0 at src/lib.rs:61:23: 61:24
        _4 = &((*_5).0: std::sync::atomic::AtomicUsize); // bb1[1]: scope 0 at src/lib.rs:61:12: 61:31
        StorageLive(_7);                 // bb1[2]: scope 0 at src/lib.rs:61:45: 61:52
        discriminant(_7) = 1;            // bb1[3]: scope 0 at src/lib.rs:61:45: 61:52
        // XXXXXX fetch_sub is exectued here
        _3 = const std::sync::atomic::AtomicUsize::fetch_sub(move _4, const 1usize, move _7) -> bb2; // bb1[4]: scope 0 at src/lib.rs:61:12: 61:53
                                         // ty::Const
                                         // + ty: for<'r> fn(&'r std::sync::atomic::AtomicUsize, usize, std::sync::atomic::Ordering) -> usize {std::sync::atomic::AtomicUsize::fetch_sub}
                                         // + val: Value(Scalar(<ZST>))
                                         // mir::Constant
                                         // + span: src/lib.rs:61:32: 61:41
                                         // + literal: Const { ty: for<'r> fn(&'r std::sync::atomic::AtomicUsize, usize, std::sync::atomic::Ordering) -> usize {std::sync::atomic::AtomicUsize::fetch_sub}, val: Value(Scalar(<ZST>)) }
                                         // ty::Const
                                         // + ty: usize
                                         // + val: Value(Scalar(0x0000000000000001))
                                         // mir::Constant
                                         // + span: src/lib.rs:61:42: 61:43
                                         // + literal: Const { ty: usize, val: Value(Scalar(0x0000000000000001)) }
    }

    bb2: {
        // XXXXXX _5 may be dangling here
        StorageDead(_7);                 // bb2[0]: scope 0 at src/lib.rs:61:52: 61:53
        StorageDead(_4);                 // bb2[1]: scope 0 at src/lib.rs:61:52: 61:53
        _2 = Ne(move _3, const 1usize);  // bb2[2]: scope 0 at src/lib.rs:61:12: 61:58
                                         // ty::Const
                                         // + ty: usize
                                         // + val: Value(Scalar(0x0000000000000001))
                                         // mir::Constant
                                         // + span: src/lib.rs:61:57: 61:58
                                         // + literal: Const { ty: usize, val: Value(Scalar(0x0000000000000001)) }
        // XXXXX _5 is marked dead here
        StorageDead(_5);                 // bb2[3]: scope 0 at src/lib.rs:61:57: 61:58
        StorageDead(_3);                 // bb2[4]: scope 0 at src/lib.rs:61:57: 61:58
        switchInt(_2) -> [false: bb3, otherwise: bb4]; // bb2[5]: scope 0 at src/lib.rs:61:9: 63:10
    }

    bb3: {
        StorageDead(_2);                 // bb3[0]: scope 0 at src/lib.rs:63:9: 63:10
        StorageLive(_8);                 // bb3[1]: scope 0 at src/lib.rs:13:9: 13:31
        StorageLive(_9);                 // bb3[2]: scope 0 at src/lib.rs:13:23: 13:30
        discriminant(_9) = 2;            // bb3[3]: scope 0 at src/lib.rs:13:23: 13:30
        _8 = const std::sync::atomic::fence(move _9) -> bb6; // bb3[4]: scope 0 at src/lib.rs:13:9: 13:31
                                         // ty::Const
                                         // + ty: fn(std::sync::atomic::Ordering) {std::sync::atomic::fence}
                                         // + val: Value(Scalar(<ZST>))
                                         // mir::Constant
                                         // + span: src/lib.rs:13:9: 13:22
                                         // + literal: Const { ty: fn(std::sync::atomic::Ordering) {std::sync::atomic::fence}, val: Value(Scalar(<ZST>)) }
    }

    bb4: {
        StorageDead(_2);                 // bb4[0]: scope 0 at src/lib.rs:63:9: 63:10
        goto -> bb5;                     // bb4[1]: scope 0 at src/lib.rs:62:13: 62:19
    }

    bb5: {
        return;                          // bb5[0]: scope 0 at src/lib.rs:70:6: 70:6
    }

    bb6: {
        StorageDead(_9);                 // bb6[0]: scope 0 at src/lib.rs:13:30: 13:31
        StorageDead(_8);                 // bb6[1]: scope 0 at src/lib.rs:13:30: 13:31
        StorageLive(_10);                // bb6[2]: scope 1 at src/lib.rs:68:13: 68:29
        StorageLive(_11);                // bb6[3]: scope 1 at src/lib.rs:68:13: 68:17
        _11 = _1;                        // bb6[4]: scope 1 at src/lib.rs:68:13: 68:17
        _10 = const Arc::<T>::drop_slow(move _11) -> bb7; // bb6[5]: scope 1 at src/lib.rs:68:13: 68:29
                                         // ty::Const
                                         // + ty: for<'r> unsafe fn(&'r mut Arc<T>) {Arc::<T>::drop_slow}
                                         // + val: Value(Scalar(<ZST>))
                                         // mir::Constant
                                         // + span: src/lib.rs:68:18: 68:27
                                         // + literal: Const { ty: for<'r> unsafe fn(&'r mut Arc<T>) {Arc::<T>::drop_slow}, val: Value(Scalar(<ZST>)) }
    }

    bb7: {
        StorageDead(_11);                // bb7[0]: scope 1 at src/lib.rs:68:28: 68:29
        StorageDead(_10);                // bb7[1]: scope 1 at src/lib.rs:68:29: 68:30
        goto -> bb5;                     // bb7[2]: scope 0 at src/lib.rs:70:6: 70:6
    }
}
