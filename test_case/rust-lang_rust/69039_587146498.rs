rust
fn  my_scenario::{{closure}}#0(_1: std::pin::Pin<&mut [generator@gen.rs:13:5: 17:6 {std::string::String, ()}]>, _2: std::string::String) -> std::ops::GeneratorState<(), ()> {
    debug _arg => (((*(_1.0: &mut [generator@gen.rs:13:5: 17:6 {std::string::String, ()}])) as variant#3).1: std::string::String); // in scope 0 at gen.rs:13:6: 13:10
    let mut _0: std::ops::GeneratorState<(), ()>; // return place in scope 0 at gen.rs:13:5: 17:6
    let mut _3: ();                      // in scope 0 at gen.rs:14:21: 14:26
    let mut _4: ();                      // in scope 0 at gen.rs:15:21: 15:26
    let _5: ();                          // in scope 0 at gen.rs:16:9: 16:26
    let mut _6: &std::string::String;    // in scope 0 at gen.rs:16:11: 16:17
    let _7: &std::string::String;        // in scope 0 at gen.rs:16:11: 16:17
    let mut _8: &std::string::String;    // in scope 0 at gen.rs:16:19: 16:25
    let _9: &std::string::String;        // in scope 0 at gen.rs:16:19: 16:25
    let mut _10: ();                     // in scope 0 at gen.rs:13:20: 13:20
    let mut _11: isize;                  // in scope 0 at gen.rs:13:5: 17:6
    scope 1 {
        debug name1 => (((*(_1.0: &mut [generator@gen.rs:13:5: 17:6 {std::string::String, ()}])) as variant#3).0: std::string::String); // in scope 1 at gen.rs:14:13: 14:18
        scope 2 {
            debug name2 => (((*(_1.0: &mut [generator@gen.rs:13:5: 17:6 {std::string::String, ()}])) as variant#4).1: std::string::String); // in scope 2 at gen.rs:15:13: 15:18
        }
    }

    bb0: {
        _11 = discriminant((*(_1.0: &mut [generator@gen.rs:13:5: 17:6 {std::string::String, ()}]))); // bb0[0]: scope 0 at gen.rs:13:5: 17:6
        switchInt(move _11) -> [0u32: bb1, 1u32: bb12, 2u32: bb13, 3u32: bb10, 4u32: bb11, otherwise: bb14]; // bb0[1]: scope 0 at gen.rs:13:5: 17:6
    }

    bb1: {
        (((*(_1.0: &mut [generator@gen.rs:13:5: 17:6 {std::string::String, ()}])) as variant#3).1: std::string::String) = move _2; // bb1[0]: scope 0 at gen.rs:13:5: 17:6
        StorageLive(_3);                 // bb1[1]: scope 0 at gen.rs:14:21: 14:26
        ((_0 as Yielded).0: ()) = move _3; // bb1[2]: scope 0 at gen.rs:14:21: 14:26
        discriminant(_0) = 0;            // bb1[3]: scope 0 at gen.rs:14:21: 14:26
        discriminant((*(_1.0: &mut [generator@gen.rs:13:5: 17:6 {std::string::String, ()}]))) = 3; // bb1[4]: scope 0 at gen.rs:14:21: 14:26
        return;                          // bb1[5]: scope 0 at gen.rs:14:21: 14:26
    }
...
    bb10: {
        StorageLive(_3);                 // bb10[0]: scope 0 at gen.rs:13:5: 17:6
        (((*(_1.0: &mut [generator@gen.rs:13:5: 17:6 {std::string::String, ()}])) as variant#3).0: std::string::String) = move _2; // bb10[1]: scope 0 at gen.rs:13:5: 17:6
        StorageDead(_3);                 // bb10[2]: scope 0 at gen.rs:14:25: 14:26
        StorageLive(_4);                 // bb10[3]: scope 1 at gen.rs:15:21: 15:26
        ((_0 as Yielded).0: ()) = move _4; // bb10[4]: scope 1 at gen.rs:15:21: 15:26
        discriminant(_0) = 0;            // bb10[5]: scope 1 at gen.rs:15:21: 15:26
        discriminant((*(_1.0: &mut [generator@gen.rs:13:5: 17:6 {std::string::String, ()}]))) = 4; // bb10[6]: scope 1 at gen.rs:15:21: 15:26
        return;                          // bb10[7]: scope 1 at gen.rs:15:21: 15:26
    }

    bb11: {
        StorageLive(_4);                 // bb11[0]: scope 0 at gen.rs:13:5: 17:6
        (((*(_1.0: &mut [generator@gen.rs:13:5: 17:6 {std::string::String, ()}])) as variant#4).1: std::string::String) = move _2; // bb11[1]: scope 0 at gen.rs:13:5: 17:6
        StorageDead(_4);                 // bb11[2]: scope 1 at gen.rs:15:25: 15:26
        StorageLive(_5);                 // bb11[3]: scope 2 at gen.rs:16:9: 16:26
        StorageLive(_6);                 // bb11[4]: scope 2 at gen.rs:16:11: 16:17
        StorageLive(_7);                 // bb11[5]: scope 2 at gen.rs:16:11: 16:17
        _7 = &(((*(_1.0: &mut [generator@gen.rs:13:5: 17:6 {std::string::String, ()}])) as variant#3).0: std::string::String); // bb11[6]: scope 2 at gen.rs:16:11: 16:17
        _6 = _7;                         // bb11[7]: scope 2 at gen.rs:16:11: 16:17
        StorageLive(_8);                 // bb11[8]: scope 2 at gen.rs:16:19: 16:25
        StorageLive(_9);                 // bb11[9]: scope 2 at gen.rs:16:19: 16:25
        _9 = &(((*(_1.0: &mut [generator@gen.rs:13:5: 17:6 {std::string::String, ()}])) as variant#4).1: std::string::String); // bb11[10]: scope 2 at gen.rs:16:19: 16:25
        _8 = _9;                         // bb11[11]: scope 2 at gen.rs:16:19: 16:25
        _5 = const p(move _6, move _8) -> [return: bb3, unwind: bb6]; // bb11[12]: scope 2 at gen.rs:16:9: 16:26
                                         // ty::Const
                                         // + ty: for<'r, 's> fn(&'r std::string::String, &'s std::string::String) {p}
                                         // + val: Value(Scalar(<ZST>))
                                         // mir::Constant
                                         // + span: gen.rs:16:9: 16:10
                                         // + literal: Const { ty: for<'r, 's> fn(&'r std::string::String, &'s std::string::String) {p}, val: Value(Scalar(<ZST>)) }
    }
...
