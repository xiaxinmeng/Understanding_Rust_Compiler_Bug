mir
fn fails::{closure#0}(_1: [closure@issues/issue_105699.rs:2:15: 2:17]) -> &mut dyn Drop {
    debug input => (*(_1.0: &mut std::boxed::Box<dyn std::ops::Drop>)); // in scope 0 at issues/issue_105699.rs:1:10: 1:15
    let mut _0: &mut dyn std::ops::Drop; // return place in scope 0 at issues/issue_105699.rs:2:18: 2:18
    let mut _2: &mut std::boxed::Box<dyn std::ops::Drop>; // in scope 0 at issues/issue_105699.rs:2:18: 2:32

    bb0: {
        StorageLive(_2);                 // scope 0 at issues/issue_105699.rs:2:18: 2:32
        _2 = &mut (*(_1.0: &mut std::boxed::Box<dyn std::ops::Drop>)); // scope 0 at issues/issue_105699.rs:2:18: 2:32
        _0 = <Box<dyn Drop> as AsMut<dyn Drop>>::as_mut(move _2) -> [return: bb1, unwind: bb2]; // scope 0 at issues/issue_105699.rs:2:18: 2:32
                                         // mir::Constant
                                         // + span: issues/issue_105699.rs:2:24: 2:30
                                         // + literal: Const { ty: for<'a> fn(&'a mut Box<dyn Drop>) -> &'a mut dyn Drop {<Box<dyn Drop> as AsMut<dyn Drop>>::as_mut}, val: Value(<ZST>) }
    }

    bb1: {
        StorageDead(_2);                 // scope 0 at issues/issue_105699.rs:2:31: 2:32
        return;                          // scope 0 at issues/issue_105699.rs:2:32: 2:32
    }

    bb2 (cleanup): {
        resume;                          // scope 0 at issues/issue_105699.rs:2:15: 2:32
    }
}
