
// MIR for `func` after Inline

fn func(_1: Foo) -> () {
    debug arg => _1;                     // in scope 0 at inline-no-generics-no-vec.rs:17:9: 17:12
    let mut _0: ();                      // return place in scope 0 at inline-no-generics-no-vec.rs:17:19: 17:19
    let mut _2: std::mem::ManuallyDrop<Foo>; // in scope 0 at inline-no-generics-no-vec.rs:18:9: 18:15
    let mut _3: Foo;                     // in scope 0 at inline-no-generics-no-vec.rs:18:45: 18:48
    let _4: ();                          // in scope 0 at inline-no-generics-no-vec.rs:19:14: 19:41
    let mut _5: &mut std::mem::ManuallyDrop<Foo>; // in scope 0 at inline-no-generics-no-vec.rs:19:33: 19:40
    let mut _6: &mut std::mem::ManuallyDrop<Foo>; // in scope 0 at inline-no-generics-no-vec.rs:19:33: 19:40
    scope 1 {
        debug md => _2;                  // in scope 1 at inline-no-generics-no-vec.rs:18:9: 18:15
        scope 3 {
            scope 5 (inlined ManuallyDrop::<Foo>::drop) { // at inline-no-generics-no-vec.rs:19:14: 19:41
                debug slot => _5;        // in scope 5 at /home/gh-b-naber/rust/library/core/src/mem/manually_drop.rs:140:24: 140:28
                let mut _7: *mut Foo;    // in scope 5 at /home/gh-b-naber/rust/library/core/src/mem/manually_drop.rs:144:37: 144:52
                let mut _8: &mut Foo;    // in scope 5 at /home/gh-b-naber/rust/library/core/src/mem/manually_drop.rs:144:37: 144:52
                scope 6 {
                    scope 7 (inlined std::ptr::drop_in_place::<Foo> - shim(Some(Foo))) { // at /home/gh-b-naber/rust/library/core/src/mem/manually_drop.rs:144:18: 144:53
                        let mut _9: &mut Foo; // in scope 7 at /home/gh-b-naber/rust/library/core/src/ptr/mod.rs:490:1: 490:56
                        let mut _10: (); // in scope 7 at /home/gh-b-naber/rust/library/core/src/ptr/mod.rs:490:1: 490:56
                    }
                }
            }
        }
    }
    scope 2 {
        scope 4 (inlined ManuallyDrop::<Foo>::new) { // at inline-no-generics-no-vec.rs:18:27: 18:49
            debug value => _3;           // in scope 4 at /home/gh-b-naber/rust/library/core/src/mem/manually_drop.rs:70:22: 70:27
        }
    }

    bb0: {
        StorageLive(_2);                 // scope 0 at inline-no-generics-no-vec.rs:18:9: 18:15
        StorageLive(_3);                 // scope 2 at inline-no-generics-no-vec.rs:18:45: 18:48
        _3 = move _1;                    // scope 2 at inline-no-generics-no-vec.rs:18:45: 18:48
        _2 = ManuallyDrop::<Foo> { value: move _3 }; // scope 4 at /home/gh-b-naber/rust/library/core/src/mem/manually_drop.rs:71:9: 71:31
        StorageDead(_3);                 // scope 2 at inline-no-generics-no-vec.rs:18:48: 18:49
        StorageLive(_4);                 // scope 1 at inline-no-generics-no-vec.rs:19:5: 19:43
        StorageLive(_5);                 // scope 3 at inline-no-generics-no-vec.rs:19:33: 19:40
        StorageLive(_6);                 // scope 3 at inline-no-generics-no-vec.rs:19:33: 19:40
        _6 = &mut _2;                    // scope 3 at inline-no-generics-no-vec.rs:19:33: 19:40
        _5 = &mut (*_6);                 // scope 3 at inline-no-generics-no-vec.rs:19:33: 19:40
        StorageLive(_7);                 // scope 6 at /home/gh-b-naber/rust/library/core/src/mem/manually_drop.rs:144:37: 144:52
        StorageLive(_8);                 // scope 6 at /home/gh-b-naber/rust/library/core/src/mem/manually_drop.rs:144:37: 144:52
        _8 = &mut ((*_5).0: Foo);        // scope 6 at /home/gh-b-naber/rust/library/core/src/mem/manually_drop.rs:144:37: 144:52
        _7 = &raw mut (*_8);             // scope 6 at /home/gh-b-naber/rust/library/core/src/mem/manually_drop.rs:144:37: 144:52
        StorageLive(_9);                 // scope 6 at /home/gh-b-naber/rust/library/core/src/mem/manually_drop.rs:144:18: 144:53
        StorageLive(_10);                // scope 6 at /home/gh-b-naber/rust/library/core/src/mem/manually_drop.rs:144:18: 144:53
        _9 = &mut (*_7);                 // scope 7 at /home/gh-b-naber/rust/library/core/src/ptr/mod.rs:490:1: 490:56
        _10 = <Foo as Drop>::drop(move _9) -> [return: bb2, unwind: bb1]; // scope 7 at /home/gh-b-naber/rust/library/core/src/ptr/mod.rs:490:1: 490:56
                                         // mir::Constant
                                         // + span: /home/gh-b-naber/rust/library/core/src/ptr/mod.rs:490:1: 490:56
                                         // + literal: Const { ty: for<'a> fn(&'a mut Foo) {<Foo as Drop>::drop}, val: Value(<ZST>) }
    }

    bb1 (cleanup): {
        resume;                          // scope 0 at inline-no-generics-no-vec.rs:17:1: 20:2
    }

    bb2: {
        StorageDead(_10);                // scope 6 at /home/gh-b-naber/rust/library/core/src/mem/manually_drop.rs:144:18: 144:53
        StorageDead(_9);                 // scope 6 at /home/gh-b-naber/rust/library/core/src/mem/manually_drop.rs:144:18: 144:53
        StorageDead(_7);                 // scope 6 at /home/gh-b-naber/rust/library/core/src/mem/manually_drop.rs:144:52: 144:53
        StorageDead(_8);                 // scope 5 at /home/gh-b-naber/rust/library/core/src/mem/manually_drop.rs:145:5: 145:6
        StorageDead(_5);                 // scope 3 at inline-no-generics-no-vec.rs:19:40: 19:41
        StorageDead(_6);                 // scope 1 at inline-no-generics-no-vec.rs:19:43: 19:44
        StorageDead(_4);                 // scope 1 at inline-no-generics-no-vec.rs:19:43: 19:44
        _0 = const ();                   // scope 0 at inline-no-generics-no-vec.rs:17:19: 20:2
        StorageDead(_2);                 // scope 0 at inline-no-generics-no-vec.rs:20:1: 20:2
        return;                          // scope 0 at inline-no-generics-no-vec.rs:20:2: 20:2
    }
}
