mir
fn works::{closure#0}(_1: [closure@issues/issue_105699.rs:2:15: 2:17]) -> &mut dyn Drop {
    debug input => (*(_1.0: &mut std::boxed::Box<dyn std::ops::Drop>)); // in scope 0 at issues/issue_105699.rs:1:10: 1:15
    let mut _0: &mut dyn std::ops::Drop; // return place in scope 0 at issues/issue_105699.rs:2:18: 2:18
    let mut _2: &mut dyn std::ops::Drop; // in scope 0 at issues/issue_105699.rs:2:20: 2:34
    let mut _3: &mut dyn std::ops::Drop; // in scope 0 at issues/issue_105699.rs:2:20: 2:34
    let mut _4: &mut std::boxed::Box<dyn std::ops::Drop>; // in scope 0 at issues/issue_105699.rs:2:20: 2:34

    bb0: {
        StorageLive(_2);                 // scope 0 at issues/issue_105699.rs:2:20: 2:34
        StorageLive(_3);                 // scope 0 at issues/issue_105699.rs:2:20: 2:34
        StorageLive(_4);                 // scope 0 at issues/issue_105699.rs:2:20: 2:34
        _4 = &mut (*(_1.0: &mut std::boxed::Box<dyn std::ops::Drop>)); // scope 0 at issues/issue_105699.rs:2:20: 2:34
        _3 = <Box<dyn Drop> as AsMut<dyn Drop>>::as_mut(move _4) -> [return: bb1, unwind: bb2]; // scope 0 at issues/issue_105699.rs:2:20: 2:34
                                         // mir::Constant
                                         // + span: issues/issue_105699.rs:2:26: 2:32
                                         // + literal: Const { ty: for<'a> fn(&'a mut Box<dyn Drop>) -> &'a mut dyn Drop {<Box<dyn Drop> as AsMut<dyn Drop>>::as_mut}, val: Value(<ZST>) }
    }

    bb1: {
        StorageDead(_4);                 // scope 0 at issues/issue_105699.rs:2:33: 2:34
        _2 = &mut (*_3);                 // scope 0 at issues/issue_105699.rs:2:20: 2:34
        _0 = move _2 as &mut dyn std::ops::Drop (Pointer(Unsize)); // scope 0 at issues/issue_105699.rs:2:20: 2:34
        StorageDead(_3);                 // scope 0 at issues/issue_105699.rs:2:35: 2:36
        StorageDead(_2);                 // scope 0 at issues/issue_105699.rs:2:35: 2:36
        return;                          // scope 0 at issues/issue_105699.rs:2:36: 2:36
    }

    bb2 (cleanup): {
        resume;                          // scope 0 at issues/issue_105699.rs:2:15: 2:36
    }
}
