rust
fn unreachable() -> ! {
    let mut _0: !;                       // return place in scope 0 at test2.rs:1:34: 1:35
    let mut _1: !;                       // in scope 0 at test2.rs:1:36: 6:2
    let mut _2: !;                       // in scope 0 at test2.rs:5:5: 5:25
    let mut _3: *const std::convert::Infallible; // in scope 0 at test2.rs:5:12: 5:22

    bb0: {
        StorageLive(_1);                 // scope 0 at test2.rs:1:36: 6:2
        StorageLive(_2);                 // scope 0 at test2.rs:5:5: 5:25
        StorageLive(_3);                 // scope 0 at test2.rs:5:12: 5:22
        _3 = const INFALLIBLE;           // scope 0 at test2.rs:5:12: 5:22
                                         // ty::Const
                                         // + ty: *const std::convert::Infallible
                                         // + val: Unevaluated(WithOptConstParam { did: DefId(0:5 ~ test2[317d]::unreachable::INFALLIBLE), const_param_did: None }, [], None)
                                         // mir::Constant
                                         // + span: test2.rs:5:12: 5:22
                                         // + literal: Const { ty: *const std::convert::Infallible, val: Unevaluated(WithOptConstParam { did: DefId(0:5 ~ test2[317d]::unreachable::INFALLIBLE), const_param_did: None }, [], None) }
        FakeRead(ForMatchedPlace, (*_3)); // scope 0 at test2.rs:5:11: 5:22
        unreachable;                     // scope 0 at test2.rs:5:11: 5:22
    }

    bb1: {
        unreachable;                     // scope 0 at test2.rs:5:5: 5:25
    }

    bb2: {
        StorageDead(_2);                 // scope 0 at test2.rs:5:24: 5:25
        unreachable;                     // scope 0 at test2.rs:1:36: 6:2
    }

    bb3: {
        StorageDead(_3);                 // scope 0 at test2.rs:6:1: 6:2
        StorageDead(_1);                 // scope 0 at test2.rs:6:1: 6:2
        return;                          // scope 0 at test2.rs:6:2: 6:2
    }
}
