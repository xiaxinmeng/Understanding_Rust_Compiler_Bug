
// WARNING: This output format is intended for human consumers only
// and is subject to change without notice. Knock yourself out.
fn f() -> f64 {
    let mut _0: f64;                     // return place in scope 0 at repro.rs:1:15: 1:18
    let mut _1: f64;                     // in scope 0 at repro.rs:2:5: 2:31
    let mut _2: f64;                     // in scope 0 at repro.rs:2:34: 2:60

    bb0: {
        _1 = std::hint::black_box::<f64>(const -1f64) -> bb1; // scope 0 at repro.rs:2:5: 2:31
                                         // mir::Constant
                                         // + span: repro.rs:2:5: 2:25
                                         // + literal: Const { ty: fn(f64) -> f64 {std::hint::black_box::<f64>}, val: Value(<ZST>) }
    }

    bb1: {
        _2 = std::hint::black_box::<f64>(const -1f64) -> bb2; // scope 0 at repro.rs:2:34: 2:60
                                         // mir::Constant
                                         // + span: repro.rs:2:34: 2:54
                                         // + literal: Const { ty: fn(f64) -> f64 {std::hint::black_box::<f64>}, val: Value(<ZST>) }
    }

    bb2: {
        _0 = Rem(move _1, move _2);      // scope 0 at repro.rs:2:5: 2:60
        return;                          // scope 0 at repro.rs:3:2: 3:2
    }
}

fn g() -> f64 {
    let mut _0: f64;                     // return place in scope 0 at repro.rs:5:15: 5:18

    bb0: {
        _0 = const 0f64;                 // scope 0 at repro.rs:6:5: 6:16
        return;                          // scope 0 at repro.rs:7:2: 7:2
    }
}
