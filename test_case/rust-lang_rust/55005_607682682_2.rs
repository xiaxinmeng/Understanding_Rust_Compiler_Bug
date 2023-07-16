rust
// WARNING: This output format is intended for human consumers only
// and is subject to change without notice. Knock yourself out.
fn <impl at src/lib.rs:37:1: 50:2>::drop_slow(_1: &mut Arc<T>) -> () {
    debug self => _1;                    // in scope 0 at src/lib.rs:42:25: 42:34
    let mut _0: ();                      // return place in scope 0 at src/lib.rs:42:36: 42:36
    let _2: ();                          // in scope 0 at src/lib.rs:43:9: 43:56
    let mut _3: *mut T;                  // in scope 0 at src/lib.rs:43:28: 43:55
    let mut _4: &mut T;                  // in scope 0 at src/lib.rs:43:28: 43:55
    // XXXXXX This ArcInner is never dangling
    let mut _5: &mut ArcInner<T>;        // in scope 0 at src/lib.rs:43:33: 43:50
    let mut _6: &mut std::ptr::NonNull<ArcInner<T>>; // in scope 0 at src/lib.rs:43:33: 43:41
    let mut _7: bool;                    // in scope 0 at src/lib.rs:45:12: 45:56
    let mut _8: usize;                   // in scope 0 at src/lib.rs:45:12: 45:51
    let mut _9: &std::sync::atomic::AtomicUsize; // in scope 0 at src/lib.rs:45:12: 45:29
    // XXXXXX We care about _10, the reference to ArcInner
    let _10: &ArcInner<T>;               // in scope 0 at src/lib.rs:45:12: 45:24
    let mut _11: &Arc<T>;                // in scope 0 at src/lib.rs:45:12: 45:16
    let mut _12: std::sync::atomic::Ordering; // in scope 0 at src/lib.rs:45:43: 45:50
    let _13: ();                         // in scope 0 at src/lib.rs:13:9: 13:31
    let mut _14: std::sync::atomic::Ordering; // in scope 0 at src/lib.rs:13:23: 13:30
    let mut _15: &mut std::alloc::Global; // in scope 0 at src/lib.rs:47:13: 47:19
    let mut _16: std::alloc::Global;     // in scope 0 at src/lib.rs:47:13: 47:19
    let mut _17: std::ptr::NonNull<u8>;  // in scope 0 at src/lib.rs:47:28: 47:43
    let mut _18: std::ptr::NonNull<ArcInner<T>>; // in scope 0 at src/lib.rs:47:28: 47:36
    let mut _19: std::alloc::Layout;     // in scope 0 at src/lib.rs:47:45: 47:81
    // XXXXXX This ArcInner is never dangling
    let mut _20: &ArcInner<T>;           // in scope 0 at src/lib.rs:47:63: 47:80
    // XXXXXX We care about _21, the reference to ArcInner
    let _21: &ArcInner<T>;               // in scope 0 at src/lib.rs:47:63: 47:80
    let mut _22: &std::ptr::NonNull<ArcInner<T>>; // in scope 0 at src/lib.rs:47:63: 47:71

    bb0: {
        StorageLive(_2);                 // bb0[0]: scope 0 at src/lib.rs:43:9: 43:56
        StorageLive(_3);                 // bb0[1]: scope 0 at src/lib.rs:43:28: 43:55
        StorageLive(_4);                 // bb0[2]: scope 0 at src/lib.rs:43:28: 43:55
        // XXXXXX _5 is marked live here
        StorageLive(_5);                 // bb0[3]: scope 0 at src/lib.rs:43:33: 43:50
        StorageLive(_6);                 // bb0[4]: scope 0 at src/lib.rs:43:33: 43:41
        _6 = &mut ((*_1).0: std::ptr::NonNull<ArcInner<T>>); // bb0[5]: scope 0 at src/lib.rs:43:33: 43:41
        _5 = const std::ptr::NonNull::<ArcInner<T>>::as_mut(move _6) -> bb1; // bb0[6]: scope 0 at src/lib.rs:43:33: 43:50
                                         // ty::Const
                                         // + ty: for<'r> unsafe fn(&'r mut std::ptr::NonNull<ArcInner<T>>) -> &'r mut ArcInner<T> {std::ptr::NonNull::<ArcInner<T>>::as_mut}
                                         // + val: Value(Scalar(<ZST>))
                                         // mir::Constant
                                         // + span: src/lib.rs:43:42: 43:48
                                         // + literal: Const { ty: for<'r> unsafe fn(&'r mut std::ptr::NonNull<ArcInner<T>>) -> &'r mut ArcInner<T> {std::ptr::NonNull::<ArcInner<T>>::as_mut}, val: Value(Scalar(<ZST>)) }
    }

    bb1: {
        StorageDead(_6);                 // bb1[0]: scope 0 at src/lib.rs:43:49: 43:50
        _4 = &mut ((*_5).2: T);          // bb1[1]: scope 0 at src/lib.rs:43:28: 43:55
        _3 = &raw mut (*_4);             // bb1[2]: scope 0 at src/lib.rs:43:28: 43:55
        _2 = const std::intrinsics::drop_in_place::<T>(move _3) -> bb2; // bb1[3]: scope 0 at src/lib.rs:43:9: 43:56
                                         // ty::Const
                                         // + ty: unsafe fn(*mut T) {std::intrinsics::drop_in_place::<T>}
                                         // + val: Value(Scalar(<ZST>))
                                         // mir::Constant
                                         // + span: src/lib.rs:43:9: 43:27
                                         // + literal: Const { ty: unsafe fn(*mut T) {std::intrinsics::drop_in_place::<T>}, val: Value(Scalar(<ZST>)) }
    }

    bb2: {
        StorageDead(_3);                 // bb2[0]: scope 0 at src/lib.rs:43:55: 43:56
        // XXXXXX _5 is marked dead here
        StorageDead(_5);                 // bb2[1]: scope 0 at src/lib.rs:43:56: 43:57
        StorageDead(_4);                 // bb2[2]: scope 0 at src/lib.rs:43:56: 43:57
        StorageDead(_2);                 // bb2[3]: scope 0 at src/lib.rs:43:56: 43:57
        StorageLive(_7);                 // bb2[4]: scope 0 at src/lib.rs:45:12: 45:56
        StorageLive(_8);                 // bb2[5]: scope 0 at src/lib.rs:45:12: 45:51
        StorageLive(_9);                 // bb2[6]: scope 0 at src/lib.rs:45:12: 45:29
        // XXXXXX _10 is marked live here
        StorageLive(_10);                // bb2[7]: scope 0 at src/lib.rs:45:12: 45:24
        StorageLive(_11);                // bb2[8]: scope 0 at src/lib.rs:45:12: 45:16
        _11 = _1;                        // bb2[9]: scope 0 at src/lib.rs:45:12: 45:16
        // XXXXXX _10 is created here
        _10 = const Arc::<T>::inner(move _11) -> bb3; // bb2[10]: scope 0 at src/lib.rs:45:12: 45:24
                                         // ty::Const
                                         // + ty: for<'r> fn(&'r Arc<T>) -> &'r ArcInner<T> {Arc::<T>::inner}
                                         // + val: Value(Scalar(<ZST>))
                                         // mir::Constant
                                         // + span: src/lib.rs:45:17: 45:22
                                         // + literal: Const { ty: for<'r> fn(&'r Arc<T>) -> &'r ArcInner<T> {Arc::<T>::inner}, val: Value(Scalar(<ZST>)) }
    }

    bb3: {
        StorageDead(_11);                // bb3[0]: scope 0 at src/lib.rs:45:23: 45:24
        _9 = &((*_10).1: std::sync::atomic::AtomicUsize); // bb3[1]: scope 0 at src/lib.rs:45:12: 45:29
        StorageLive(_12);                // bb3[2]: scope 0 at src/lib.rs:45:43: 45:50
        discriminant(_12) = 1;           // bb3[3]: scope 0 at src/lib.rs:45:43: 45:50
        // XXXXXX fetch_sub is executed here
        _8 = const std::sync::atomic::AtomicUsize::fetch_sub(move _9, const 1usize, move _12) -> bb4; // bb3[4]: scope 0 at src/lib.rs:45:12: 45:51
                                         // ty::Const
                                         // + ty: for<'r> fn(&'r std::sync::atomic::AtomicUsize, usize, std::sync::atomic::Ordering) -> usize {std::sync::atomic::AtomicUsize::fetch_sub}
                                         // + val: Value(Scalar(<ZST>))
                                         // mir::Constant
                                         // + span: src/lib.rs:45:30: 45:39
                                         // + literal: Const { ty: for<'r> fn(&'r std::sync::atomic::AtomicUsize, usize, std::sync::atomic::Ordering) -> usize {std::sync::atomic::AtomicUsize::fetch_sub}, val: Value(Scalar(<ZST>)) }
                                         // ty::Const
                                         // + ty: usize
                                         // + val: Value(Scalar(0x0000000000000001))
                                         // mir::Constant
                                         // + span: src/lib.rs:45:40: 45:41
                                         // + literal: Const { ty: usize, val: Value(Scalar(0x0000000000000001)) }
    }

    bb4: {
        // XXXXXX _10 may be dangling here
        StorageDead(_12);                // bb4[0]: scope 0 at src/lib.rs:45:50: 45:51
        StorageDead(_9);                 // bb4[1]: scope 0 at src/lib.rs:45:50: 45:51
        _7 = Eq(move _8, const 1usize);  // bb4[2]: scope 0 at src/lib.rs:45:12: 45:56
                                         // ty::Const
                                         // + ty: usize
                                         // + val: Value(Scalar(0x0000000000000001))
                                         // mir::Constant
                                         // + span: src/lib.rs:45:55: 45:56
                                         // + literal: Const { ty: usize, val: Value(Scalar(0x0000000000000001)) }
        // XXXXXX _10 is marked dead here
        StorageDead(_10);                // bb4[3]: scope 0 at src/lib.rs:45:55: 45:56
        StorageDead(_8);                 // bb4[4]: scope 0 at src/lib.rs:45:55: 45:56
        switchInt(_7) -> [false: bb11, otherwise: bb5]; // bb4[5]: scope 0 at src/lib.rs:45:9: 48:10
    }

    bb5: {
        StorageLive(_13);                // bb5[0]: scope 0 at src/lib.rs:13:9: 13:31
        StorageLive(_14);                // bb5[1]: scope 0 at src/lib.rs:13:23: 13:30
        discriminant(_14) = 2;           // bb5[2]: scope 0 at src/lib.rs:13:23: 13:30
        _13 = const std::sync::atomic::fence(move _14) -> bb6; // bb5[3]: scope 0 at src/lib.rs:13:9: 13:31
                                         // ty::Const
                                         // + ty: fn(std::sync::atomic::Ordering) {std::sync::atomic::fence}
                                         // + val: Value(Scalar(<ZST>))
                                         // mir::Constant
                                         // + span: src/lib.rs:13:9: 13:22
                                         // + literal: Const { ty: fn(std::sync::atomic::Ordering) {std::sync::atomic::fence}, val: Value(Scalar(<ZST>)) }
    }

    bb6: {
        StorageDead(_14);                // bb6[0]: scope 0 at src/lib.rs:13:30: 13:31
        StorageDead(_13);                // bb6[1]: scope 0 at src/lib.rs:13:30: 13:31
        StorageLive(_15);                // bb6[2]: scope 0 at src/lib.rs:47:13: 47:19
        StorageLive(_16);                // bb6[3]: scope 0 at src/lib.rs:47:13: 47:19
        _15 = &mut _16;                  // bb6[4]: scope 0 at src/lib.rs:47:13: 47:19
        StorageLive(_17);                // bb6[5]: scope 0 at src/lib.rs:47:28: 47:43
        StorageLive(_18);                // bb6[6]: scope 0 at src/lib.rs:47:28: 47:36
        _18 = ((*_1).0: std::ptr::NonNull<ArcInner<T>>); // bb6[7]: scope 0 at src/lib.rs:47:28: 47:36
        _17 = const std::ptr::NonNull::<ArcInner<T>>::cast::<u8>(move _18) -> bb7; // bb6[8]: scope 0 at src/lib.rs:47:28: 47:43
                                         // ty::Const
                                         // + ty: fn(std::ptr::NonNull<ArcInner<T>>) -> std::ptr::NonNull<u8> {std::ptr::NonNull::<ArcInner<T>>::cast::<u8>}
                                         // + val: Value(Scalar(<ZST>))
                                         // mir::Constant
                                         // + span: src/lib.rs:47:37: 47:41
                                         // + literal: Const { ty: fn(std::ptr::NonNull<ArcInner<T>>) -> std::ptr::NonNull<u8> {std::ptr::NonNull::<ArcInner<T>>::cast::<u8>}, val: Value(Scalar(<ZST>)) }
    }

    bb7: {
        StorageDead(_18);                // bb7[0]: scope 0 at src/lib.rs:47:42: 47:43
        StorageLive(_19);                // bb7[1]: scope 0 at src/lib.rs:47:45: 47:81
        // XXXXXX _20 is marked live here
        StorageLive(_20);                // bb7[2]: scope 0 at src/lib.rs:47:63: 47:80
        // XXXXXX _21 is marked live here
        StorageLive(_21);                // bb7[3]: scope 0 at src/lib.rs:47:63: 47:80
        StorageLive(_22);                // bb7[4]: scope 0 at src/lib.rs:47:63: 47:71
        _22 = &((*_1).0: std::ptr::NonNull<ArcInner<T>>); // bb7[5]: scope 0 at src/lib.rs:47:63: 47:71
        // XXXXXX _21 is created here
        _21 = const std::ptr::NonNull::<ArcInner<T>>::as_ref(move _22) -> bb8; // bb7[6]: scope 0 at src/lib.rs:47:63: 47:80
                                         // ty::Const
                                         // + ty: for<'r> unsafe fn(&'r std::ptr::NonNull<ArcInner<T>>) -> &'r ArcInner<T> {std::ptr::NonNull::<ArcInner<T>>::as_ref}
                                         // + val: Value(Scalar(<ZST>))
                                         // mir::Constant
                                         // + span: src/lib.rs:47:72: 47:78
                                         // + literal: Const { ty: for<'r> unsafe fn(&'r std::ptr::NonNull<ArcInner<T>>) -> &'r ArcInner<T> {std::ptr::NonNull::<ArcInner<T>>::as_ref}, val: Value(Scalar(<ZST>)) }
    }

    bb8: {
        // XXXXXX _20 is created here
        _20 = _21;                       // bb8[0]: scope 0 at src/lib.rs:47:63: 47:80
        StorageDead(_22);                // bb8[1]: scope 0 at src/lib.rs:47:79: 47:80
        _19 = const std::alloc::Layout::for_value::<ArcInner<T>>(move _20) -> bb9; // bb8[2]: scope 0 at src/lib.rs:47:45: 47:81
                                         // ty::Const
                                         // + ty: for<'r> fn(&'r ArcInner<T>) -> std::alloc::Layout {std::alloc::Layout::for_value::<ArcInner<T>>}
                                         // + val: Value(Scalar(<ZST>))
                                         // mir::Constant
                                         // + span: src/lib.rs:47:45: 47:62
                                         // + user_ty: UserType(0)
                                         // + literal: Const { ty: for<'r> fn(&'r ArcInner<T>) -> std::alloc::Layout {std::alloc::Layout::for_value::<ArcInner<T>>}, val: Value(Scalar(<ZST>)) }
    }

    bb9: {
        // XXXXXX _20 is marked dead here
        StorageDead(_20);                // bb9[0]: scope 0 at src/lib.rs:47:80: 47:81
        // XXXXXX ArcInner is dealloc'd here
        _0 = const <std::alloc::Global as std::alloc::AllocRef>::dealloc(move _15, move _17, move _19) -> bb10; // bb9[1]: scope 0 at src/lib.rs:47:13: 47:82
                                         // ty::Const
                                         // + ty: for<'r> unsafe fn(&'r mut std::alloc::Global, std::ptr::NonNull<u8>, std::alloc::Layout) {<std::alloc::Global as std::alloc::AllocRef>::dealloc}
                                         // + val: Value(Scalar(<ZST>))
                                         // mir::Constant
                                         // + span: src/lib.rs:47:20: 47:27
                                         // + literal: Const { ty: for<'r> unsafe fn(&'r mut std::alloc::Global, std::ptr::NonNull<u8>, std::alloc::Layout) {<std::alloc::Global as std::alloc::AllocRef>::dealloc}, val: Value(Scalar(<ZST>)) }
    }

    bb10: {
        // XXXXXX _21 IS DEFINITELY DANGLING XXXXXX
        StorageDead(_19);                // bb10[0]: scope 0 at src/lib.rs:47:81: 47:82
        StorageDead(_17);                // bb10[1]: scope 0 at src/lib.rs:47:81: 47:82
        StorageDead(_15);                // bb10[2]: scope 0 at src/lib.rs:47:81: 47:82
        // XXXXXX _21 is marked dead here
        StorageDead(_21);                // bb10[3]: scope 0 at src/lib.rs:48:9: 48:10
        StorageDead(_16);                // bb10[4]: scope 0 at src/lib.rs:48:9: 48:10
        goto -> bb11;                    // bb10[5]: scope 0 at src/lib.rs:45:9: 48:10
    }

    bb11: {
        StorageDead(_7);                 // bb11[0]: scope 0 at src/lib.rs:49:5: 49:6
        return;                          // bb11[1]: scope 0 at src/lib.rs:49:6: 49:6
    }
}
