rust
fn <impl at src/lib.rs:73:1: 82:2>::drop(_1: &mut Weak<T>) -> () {
    debug self => _1;                    // in scope 0 at src/lib.rs:74:13: 74:22
    let mut _0: ();                      // return place in scope 0 at src/lib.rs:74:24: 74:24
    // XXXXXX We care about _2, the reference to ArcInner
    let _2: &ArcInner<T>;                // in scope 0 at src/lib.rs:75:13: 75:18
    let mut _3: std::option::Option<&ArcInner<T>>; // in scope 0 at src/lib.rs:75:42: 75:54
    let mut _4: &Weak<T>;                // in scope 0 at src/lib.rs:75:42: 75:46
    let mut _5: isize;                   // in scope 0 at src/lib.rs:75:28: 75:39
    // XXXXXX This ArcInner is never dangling
    let _6: &ArcInner<T>;                // in scope 0 at src/lib.rs:75:33: 75:38
    let mut _7: bool;                    // in scope 0 at src/lib.rs:77:12: 77:49
    let mut _8: usize;                   // in scope 0 at src/lib.rs:77:12: 77:44
    let mut _9: &std::sync::atomic::AtomicUsize; // in scope 0 at src/lib.rs:77:12: 77:22
    let mut _10: std::sync::atomic::Ordering; // in scope 0 at src/lib.rs:77:36: 77:43
    let _11: ();                         // in scope 0 at src/lib.rs:78:13: 78:35
    let mut _12: std::sync::atomic::Ordering; // in scope 0 at src/lib.rs:78:27: 78:34
    let mut _13: &mut std::alloc::Global; // in scope 0 at src/lib.rs:79:22: 79:28
    let mut _14: std::alloc::Global;     // in scope 0 at src/lib.rs:79:22: 79:28
    let mut _15: std::ptr::NonNull<u8>;  // in scope 0 at src/lib.rs:79:37: 79:52
    let mut _16: std::ptr::NonNull<ArcInner<T>>; // in scope 0 at src/lib.rs:79:37: 79:45
    let mut _17: std::alloc::Layout;     // in scope 0 at src/lib.rs:79:54: 79:90
    // XXXXXX This ArcInner is never dangling
    let mut _18: &ArcInner<T>;           // in scope 0 at src/lib.rs:79:72: 79:89
    // XXXXXX This ArcInner is never dangling
    let _19: &ArcInner<T>;               // in scope 0 at src/lib.rs:79:72: 79:89
    let mut _20: &std::ptr::NonNull<ArcInner<T>>; // in scope 0 at src/lib.rs:79:72: 79:80
    scope 1 {
        debug inner => _2;               // in scope 1 at src/lib.rs:75:13: 75:18
        scope 3 {
        }
    }
    scope 2 {
        debug inner => _6;               // in scope 2 at src/lib.rs:75:33: 75:38
    }

    bb0: {
        // XXXXXX _2 is marked live here
        StorageLive(_2);                 // bb0[0]: scope 0 at src/lib.rs:75:13: 75:18
        StorageLive(_3);                 // bb0[1]: scope 0 at src/lib.rs:75:42: 75:54
        StorageLive(_4);                 // bb0[2]: scope 0 at src/lib.rs:75:42: 75:46
        _4 = _1;                         // bb0[3]: scope 0 at src/lib.rs:75:42: 75:46
        _3 = const Weak::<T>::inner(move _4) -> bb1; // bb0[4]: scope 0 at src/lib.rs:75:42: 75:54
                                         // ty::Const
                                         // + ty: for<'r> fn(&'r Weak<T>) -> std::option::Option<&'r ArcInner<T>> {Weak::<T>::inner}
                                         // + val: Value(Scalar(<ZST>))
                                         // mir::Constant
                                         // + span: src/lib.rs:75:47: 75:52
                                         // + literal: Const { ty: for<'r> fn(&'r Weak<T>) -> std::option::Option<&'r ArcInner<T>> {Weak::<T>::inner}, val: Value(Scalar(<ZST>)) }
    }

    bb1: {
        StorageDead(_4);                 // bb1[0]: scope 0 at src/lib.rs:75:53: 75:54
        _5 = discriminant(_3);           // bb1[1]: scope 0 at src/lib.rs:75:28: 75:39
        switchInt(move _5) -> [1isize: bb3, otherwise: bb2]; // bb1[2]: scope 0 at src/lib.rs:75:28: 75:39
    }

    bb2: {
        StorageDead(_3);                 // bb2[0]: scope 0 at src/lib.rs:75:80: 75:81
        // XXXXXX _2 is marked dead here
        StorageDead(_2);                 // bb2[1]: scope 0 at src/lib.rs:81:5: 81:6
        goto -> bb4;                     // bb2[2]: scope 0 at src/lib.rs:75:72: 75:78
    }

    bb3: {
        // XXXXXX _6 is marked live here
        StorageLive(_6);                 // bb3[0]: scope 0 at src/lib.rs:75:33: 75:38
        // XXXXXX _6 is created here
        _6 = ((_3 as Some).0: &ArcInner<T>); // bb3[1]: scope 0 at src/lib.rs:75:33: 75:38
        // XXXXXX _2 is created here
        _2 = _6;                         // bb3[2]: scope 2 at src/lib.rs:75:57: 75:62
        // XXXXXX _6 is marked dead here
        StorageDead(_6);                 // bb3[3]: scope 0 at src/lib.rs:75:63: 75:64
        StorageDead(_3);                 // bb3[4]: scope 0 at src/lib.rs:75:80: 75:81
        StorageLive(_7);                 // bb3[5]: scope 1 at src/lib.rs:77:12: 77:49
        StorageLive(_8);                 // bb3[6]: scope 1 at src/lib.rs:77:12: 77:44
        StorageLive(_9);                 // bb3[7]: scope 1 at src/lib.rs:77:12: 77:22
        _9 = &((*_2).1: std::sync::atomic::AtomicUsize); // bb3[8]: scope 1 at src/lib.rs:77:12: 77:22
        StorageLive(_10);                // bb3[9]: scope 1 at src/lib.rs:77:36: 77:43
        discriminant(_10) = 1;           // bb3[10]: scope 1 at src/lib.rs:77:36: 77:43
        // XXXXXX fetch_sub is executed here
        _8 = const std::sync::atomic::AtomicUsize::fetch_sub(move _9, const 1usize, move _10) -> bb5; // bb3[11]: scope 1 at src/lib.rs:77:12: 77:44
                                         // ty::Const
                                         // + ty: for<'r> fn(&'r std::sync::atomic::AtomicUsize, usize, std::sync::atomic::Ordering) -> usize {std::sync::atomic::AtomicUsize::fetch_sub}
                                         // + val: Value(Scalar(<ZST>))
                                         // mir::Constant
                                         // + span: src/lib.rs:77:23: 77:32
                                         // + literal: Const { ty: for<'r> fn(&'r std::sync::atomic::AtomicUsize, usize, std::sync::atomic::Ordering) -> usize {std::sync::atomic::AtomicUsize::fetch_sub}, val: Value(Scalar(<ZST>)) }
                                         // ty::Const
                                         // + ty: usize
                                         // + val: Value(Scalar(0x0000000000000001))
                                         // mir::Constant
                                         // + span: src/lib.rs:77:33: 77:34
                                         // + literal: Const { ty: usize, val: Value(Scalar(0x0000000000000001)) }
    }

    bb4: {
        return;                          // bb4[0]: scope 0 at src/lib.rs:81:6: 81:6
    }

    bb5: {
        // XXXXXX _2 may be dangling here 
        StorageDead(_10);                // bb5[0]: scope 1 at src/lib.rs:77:43: 77:44
        StorageDead(_9);                 // bb5[1]: scope 1 at src/lib.rs:77:43: 77:44
        _7 = Eq(move _8, const 1usize);  // bb5[2]: scope 1 at src/lib.rs:77:12: 77:49
                                         // ty::Const
                                         // + ty: usize
                                         // + val: Value(Scalar(0x0000000000000001))
                                         // mir::Constant
                                         // + span: src/lib.rs:77:48: 77:49
                                         // + literal: Const { ty: usize, val: Value(Scalar(0x0000000000000001)) }
        StorageDead(_8);                 // bb5[3]: scope 1 at src/lib.rs:77:48: 77:49
        switchInt(_7) -> [false: bb12, otherwise: bb6]; // bb5[4]: scope 1 at src/lib.rs:77:9: 80:10
    }

    bb6: {
        // XXXXXX ArcInner is still live, drop it
        StorageLive(_11);                // bb6[0]: scope 1 at src/lib.rs:78:13: 78:35
        StorageLive(_12);                // bb6[1]: scope 1 at src/lib.rs:78:27: 78:34
        discriminant(_12) = 2;           // bb6[2]: scope 1 at src/lib.rs:78:27: 78:34
        _11 = const std::sync::atomic::fence(move _12) -> bb7; // bb6[3]: scope 1 at src/lib.rs:78:13: 78:35
                                         // ty::Const
                                         // + ty: fn(std::sync::atomic::Ordering) {std::sync::atomic::fence}
                                         // + val: Value(Scalar(<ZST>))
                                         // mir::Constant
                                         // + span: src/lib.rs:78:13: 78:26
                                         // + literal: Const { ty: fn(std::sync::atomic::Ordering) {std::sync::atomic::fence}, val: Value(Scalar(<ZST>)) }
    }

    bb7: {
        StorageDead(_12);                // bb7[0]: scope 1 at src/lib.rs:78:34: 78:35
        StorageDead(_11);                // bb7[1]: scope 1 at src/lib.rs:78:35: 78:36
        StorageLive(_13);                // bb7[2]: scope 3 at src/lib.rs:79:22: 79:28
        StorageLive(_14);                // bb7[3]: scope 3 at src/lib.rs:79:22: 79:28
        _13 = &mut _14;                  // bb7[4]: scope 3 at src/lib.rs:79:22: 79:28
        StorageLive(_15);                // bb7[5]: scope 3 at src/lib.rs:79:37: 79:52
        StorageLive(_16);                // bb7[6]: scope 3 at src/lib.rs:79:37: 79:45
        _16 = ((*_1).0: std::ptr::NonNull<ArcInner<T>>); // bb7[7]: scope 3 at src/lib.rs:79:37: 79:45
        _15 = const std::ptr::NonNull::<ArcInner<T>>::cast::<u8>(move _16) -> bb8; // bb7[8]: scope 3 at src/lib.rs:79:37: 79:52
                                         // ty::Const
                                         // + ty: fn(std::ptr::NonNull<ArcInner<T>>) -> std::ptr::NonNull<u8> {std::ptr::NonNull::<ArcInner<T>>::cast::<u8>}
                                         // + val: Value(Scalar(<ZST>))
                                         // mir::Constant
                                         // + span: src/lib.rs:79:46: 79:50
                                         // + literal: Const { ty: fn(std::ptr::NonNull<ArcInner<T>>) -> std::ptr::NonNull<u8> {std::ptr::NonNull::<ArcInner<T>>::cast::<u8>}, val: Value(Scalar(<ZST>)) }
    }

    bb8: {
        StorageDead(_16);                // bb8[0]: scope 3 at src/lib.rs:79:51: 79:52
        StorageLive(_17);                // bb8[1]: scope 3 at src/lib.rs:79:54: 79:90
        // XXXXXX _18 is marked live here
        StorageLive(_18);                // bb8[2]: scope 3 at src/lib.rs:79:72: 79:89
        // XXXXXX _19 is marked live here
        StorageLive(_19);                // bb8[3]: scope 3 at src/lib.rs:79:72: 79:89
        StorageLive(_20);                // bb8[4]: scope 3 at src/lib.rs:79:72: 79:80
        _20 = &((*_1).0: std::ptr::NonNull<ArcInner<T>>); // bb8[5]: scope 3 at src/lib.rs:79:72: 79:80
        // XXXXXX _19 is created here
        _19 = const std::ptr::NonNull::<ArcInner<T>>::as_ref(move _20) -> bb9; // bb8[6]: scope 3 at src/lib.rs:79:72: 79:89
                                         // ty::Const
                                         // + ty: for<'r> unsafe fn(&'r std::ptr::NonNull<ArcInner<T>>) -> &'r ArcInner<T> {std::ptr::NonNull::<ArcInner<T>>::as_ref}
                                         // + val: Value(Scalar(<ZST>))
                                         // mir::Constant
                                         // + span: src/lib.rs:79:81: 79:87
                                         // + literal: Const { ty: for<'r> unsafe fn(&'r std::ptr::NonNull<ArcInner<T>>) -> &'r ArcInner<T> {std::ptr::NonNull::<ArcInner<T>>::as_ref}, val: Value(Scalar(<ZST>)) }
    }

    bb9: {
        // XXXXXX _18 is created here
        _18 = _19;                       // bb9[0]: scope 3 at src/lib.rs:79:72: 79:89
        StorageDead(_20);                // bb9[1]: scope 3 at src/lib.rs:79:88: 79:89
        _17 = const std::alloc::Layout::for_value::<ArcInner<T>>(move _18) -> bb10; // bb9[2]: scope 3 at src/lib.rs:79:54: 79:90
                                         // ty::Const
                                         // + ty: for<'r> fn(&'r ArcInner<T>) -> std::alloc::Layout {std::alloc::Layout::for_value::<ArcInner<T>>}
                                         // + val: Value(Scalar(<ZST>))
                                         // mir::Constant
                                         // + span: src/lib.rs:79:54: 79:71
                                         // + user_ty: UserType(0)
                                         // + literal: Const { ty: for<'r> fn(&'r ArcInner<T>) -> std::alloc::Layout {std::alloc::Layout::for_value::<ArcInner<T>>}, val: Value(Scalar(<ZST>)) }
    }

    bb10: {
        // XXXXXX _18 is marked dead here
        StorageDead(_18);                // bb10[0]: scope 3 at src/lib.rs:79:89: 79:90
        // XXXXXX ArcInner is dealloc'd
        _0 = const <std::alloc::Global as std::alloc::AllocRef>::dealloc(move _13, move _15, move _17) -> bb11; // bb10[1]: scope 3 at src/lib.rs:79:22: 79:91
                                         // ty::Const
                                         // + ty: for<'r> unsafe fn(&'r mut std::alloc::Global, std::ptr::NonNull<u8>, std::alloc::Layout) {<std::alloc::Global as std::alloc::AllocRef>::dealloc}
                                         // + val: Value(Scalar(<ZST>))
                                         // mir::Constant
                                         // + span: src/lib.rs:79:29: 79:36
                                         // + literal: Const { ty: for<'r> unsafe fn(&'r mut std::alloc::Global, std::ptr::NonNull<u8>, std::alloc::Layout) {<std::alloc::Global as std::alloc::AllocRef>::dealloc}, val: Value(Scalar(<ZST>)) }
    }

    bb11: {
        // XXXXXX _19 IS DEFINITELY DANGLING XXXXXX
        // XXXXXX _2 IS DEFINITELY DANGLING XXXXXX
        StorageDead(_17);                // bb11[0]: scope 3 at src/lib.rs:79:90: 79:91
        StorageDead(_15);                // bb11[1]: scope 3 at src/lib.rs:79:90: 79:91
        StorageDead(_13);                // bb11[2]: scope 3 at src/lib.rs:79:90: 79:91
        // XXXXXX _19 is marked dead here
        StorageDead(_19);                // bb11[3]: scope 1 at src/lib.rs:80:9: 80:10
        StorageDead(_14);                // bb11[4]: scope 1 at src/lib.rs:80:9: 80:10
        goto -> bb12;                    // bb11[5]: scope 1 at src/lib.rs:77:9: 80:10
    }

    bb12: {
        // XXXXXX _2 is marked dead here
        StorageDead(_2);                 // bb12[0]: scope 0 at src/lib.rs:81:5: 81:6
        StorageDead(_7);                 // bb12[1]: scope 0 at src/lib.rs:81:5: 81:6
        goto -> bb4;                     // bb12[2]: scope 0 at src/lib.rs:81:6: 81:6
    }
}
