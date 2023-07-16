
// MIR for `func` after Inline

fn func(_1: T, _2: <T as A>::MyType) -> () {
    debug foo => _1;                     // in scope 0 at manual-drop-associated-type.rs:29:15: 29:18
    debug arg => _2;                     // in scope 0 at manual-drop-associated-type.rs:29:23: 29:26
    let mut _0: ();                      // return place in scope 0 at manual-drop-associated-type.rs:29:46: 29:46
    let mut _3: std::mem::ManuallyDrop<<T as A>::MyType>; // in scope 0 at manual-drop-associated-type.rs:30:9: 30:20
    let mut _4: <T as A>::MyType;        // in scope 0 at manual-drop-associated-type.rs:30:50: 30:53
    let mut _5: &mut std::mem::ManuallyDrop<<T as A>::MyType>; // in scope 0 at manual-drop-associated-type.rs:31:33: 31:45
    let mut _6: &mut std::mem::ManuallyDrop<<T as A>::MyType>; // in scope 0 at manual-drop-associated-type.rs:31:33: 31:45
    scope 1 {
        debug to_drop => _3;             // in scope 1 at manual-drop-associated-type.rs:30:9: 30:20
        scope 3 {
            scope 5 (inlined ManuallyDrop::<<T as A>::MyType>::drop) { // at manual-drop-associated-type.rs:31:14: 31:46
                debug slot => _5;        // in scope 5 at /home/gh-b-naber/rust/library/core/src/mem/manually_drop.rs:140:24: 140:28
                let mut _7: *mut <T as A>::MyType; // in scope 5 at /home/gh-b-naber/rust/library/core/src/mem/manually_drop.rs:144:37: 144:52
                let mut _8: &mut <T as A>::MyType; // in scope 5 at /home/gh-b-naber/rust/library/core/src/mem/manually_drop.rs:144:37: 144:52
                scope 6 {
                }
            }
        }
    }
    scope 2 {
        scope 4 (inlined ManuallyDrop::<<T as A>::MyType>::new) { // at manual-drop-associated-type.rs:30:32: 30:54
            debug value => _4;           // in scope 4 at /home/gh-b-naber/rust/library/core/src/mem/manually_drop.rs:70:22: 70:27
        }
    }

    bb0: {
        StorageLive(_3);                 // scope 0 at manual-drop-associated-type.rs:30:9: 30:20
        StorageLive(_4);                 // scope 2 at manual-drop-associated-type.rs:30:50: 30:53
        _4 = move _2;                    // scope 2 at manual-drop-associated-type.rs:30:50: 30:53
        _3 = ManuallyDrop::<<T as A>::MyType> { value: move _4 }; // scope 4 at /home/gh-b-naber/rust/library/core/src/mem/manually_drop.rs:71:9: 71:31
        StorageDead(_4);                 // scope 2 at manual-drop-associated-type.rs:30:53: 30:54
        StorageLive(_5);                 // scope 3 at manual-drop-associated-type.rs:31:33: 31:45
        StorageLive(_6);                 // scope 3 at manual-drop-associated-type.rs:31:33: 31:45
        _6 = &mut _3;                    // scope 3 at manual-drop-associated-type.rs:31:33: 31:45
        _5 = &mut (*_6);                 // scope 3 at manual-drop-associated-type.rs:31:33: 31:45
        StorageLive(_7);                 // scope 6 at /home/gh-b-naber/rust/library/core/src/mem/manually_drop.rs:144:37: 144:52
        StorageLive(_8);                 // scope 6 at /home/gh-b-naber/rust/library/core/src/mem/manually_drop.rs:144:37: 144:52
        _8 = &mut ((*_5).0: <T as A>::MyType); // scope 6 at /home/gh-b-naber/rust/library/core/src/mem/manually_drop.rs:144:37: 144:52
        _7 = &raw mut (*_8);             // scope 6 at /home/gh-b-naber/rust/library/core/src/mem/manually_drop.rs:144:37: 144:52
        _0 = std::ptr::drop_in_place::<<T as A>::MyType>(move _7) -> [return: bb4, unwind: bb2]; // scope 6 at /home/gh-b-naber/rust/library/core/src/mem/manually_drop.rs:144:18: 144:53
                                         // mir::Constant
                                         // + span: /home/gh-b-naber/rust/library/core/src/mem/manually_drop.rs:144:18: 144:36
                                         // + literal: Const { ty: unsafe fn(*mut <T as A>::MyType) {std::ptr::drop_in_place::<<T as A>::MyType>}, val: Value(<ZST>) }
    }

    bb1: {
        return;                          // scope 0 at manual-drop-associated-type.rs:32:2: 32:2
    }

    bb2 (cleanup): {
        drop(_1) -> [return: bb3, unwind terminate]; // scope 0 at manual-drop-associated-type.rs:32:1: 32:2
    }

    bb3 (cleanup): {
        resume;                          // scope 0 at manual-drop-associated-type.rs:29:1: 32:2
    }

    bb4: {
        StorageDead(_7);                 // scope 6 at /home/gh-b-naber/rust/library/core/src/mem/manually_drop.rs:144:52: 144:53
        StorageDead(_8);                 // scope 5 at /home/gh-b-naber/rust/library/core/src/mem/manually_drop.rs:145:5: 145:6
        StorageDead(_5);                 // scope 3 at manual-drop-associated-type.rs:31:45: 31:46
        StorageDead(_3);                 // scope 0 at manual-drop-associated-type.rs:32:1: 32:2
        StorageDead(_6);                 // scope 0 at manual-drop-associated-type.rs:32:1: 32:2
        drop(_1) -> [return: bb1, unwind: bb3]; // scope 0 at manual-drop-associated-type.rs:32:1: 32:2
    }
}
