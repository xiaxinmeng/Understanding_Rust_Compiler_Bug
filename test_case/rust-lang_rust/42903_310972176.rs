
// MIR for `<impl at src/main.rs:10:1: 15:2>::somemethod`
// source = Fn(NodeId(19))
// pass_name = PreTrans
// disambiguator = after

fn <impl at src/main.rs:10:1: 15:2>::somemethod(_1: A) -> A {
    let mut _0: A;                       // return pointer
    scope 1 {
        let mut _2: A;                   // "self" in scope 1 at src/main.rs:11:19: 11:27
    }
    let mut _3: fst::set::StreamBuilder;
    let mut _4: fst::set::StreamBuilder;
    let mut _5: A;

    bb0: {
        StorageLive(_2);                 // scope 0 at src/main.rs:11:19: 11:27
        _2 = _1;                         // scope 0 at src/main.rs:11:19: 11:27
        StorageLive(_3);                 // scope 1 at src/main.rs:12:23: 12:45
        StorageLive(_4);                 // scope 1 at src/main.rs:12:23: 12:34
        _4 = (_2.0: fst::set::StreamBuilder<'a>); // scope 1 at src/main.rs:12:23: 12:34
        _3 = const <fst::set::StreamBuilder<'s, A>>::ge(_4, const "doc0") -> [return: bb1, unwind: bb3]; // scope 1 at src/main.rs:12:23: 12:45
    }

    bb1: {
        StorageDead(_4);                 // scope 1 at src/main.rs:12:45: 12:45
        drop((_2.0: fst::set::StreamBuilder)) -> [return: bb5, unwind: bb4]; // scope 1 at src/main.rs:12:9: 12:20
    }

    bb2: {
        resume;                          // scope 0 at src/main.rs:11:5: 14:6
    }

    bb3: {
        drop((_2.0: fst::set::StreamBuilder)) -> bb2; // scope 0 at src/main.rs:14:6: 14:6
    }

    bb4: {
        (_2.0: fst::set::StreamBuilder) = _3; // scope 1 at src/main.rs:12:9: 12:20
        goto -> bb3;                     // scope 1 at src/main.rs:12:9: 12:20
    }

    bb5: {
        (_2.0: fst::set::StreamBuilder) = _3; // scope 1 at src/main.rs:12:9: 12:20
        StorageDead(_3);                 // scope 1 at src/main.rs:12:45: 12:45
        StorageLive(_5);                 // scope 1 at src/main.rs:13:9: 13:13
        _5 = _2;                         // scope 1 at src/main.rs:13:9: 13:13
        _0 = _5;                         // scope 1 at src/main.rs:13:9: 13:13
        StorageDead(_5);                 // scope 1 at src/main.rs:13:13: 13:13
        StorageDead(_2);                 // scope 0 at src/main.rs:14:6: 14:6
        return;                          // scope 1 at src/main.rs:14:6: 14:6
    }
}
