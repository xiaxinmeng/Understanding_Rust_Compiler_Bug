
fn f(_1: BigStruct) -> () {
    debug x => _1;                       // in scope 0 at src/lib.rs:1:6: 1:7
    let mut _0: ();                      // return place in scope 0 at src/lib.rs:1:20: 1:20
    let mut _2: BigStruct;               // in scope 0 at src/lib.rs:1:24: 1:25

    bb0: {
        _2 = move _1;                    // scope 0 at src/lib.rs:1:24: 1:25
        _0 = g(move _2) -> bb1;          // scope 0 at src/lib.rs:1:22: 1:26
                                         // mir::Constant
                                         // + span: src/lib.rs:1:22: 1:23
                                         // + literal: Const { ty: fn(BigStruct) {g}, val: Value(<ZST>) }
    }

    bb1: {
        return;                          // scope 0 at src/lib.rs:1:28: 1:28
    }
}

fn g(_1: BigStruct) -> () {
    debug x => _1;                       // in scope 0 at src/lib.rs:2:6: 2:11
    let mut _0: ();                      // return place in scope 0 at src/lib.rs:2:24: 2:24
    let _2: usize;                       // in scope 0 at src/lib.rs:2:30: 2:32

    bb0: {
        _2 = const 42_usize;             // scope 0 at src/lib.rs:2:30: 2:32
        (_1.0: [u8; 999])[_2] = const 1_u8; // scope 0 at src/lib.rs:2:26: 2:37
        return;                          // scope 0 at src/lib.rs:2:40: 2:40
    }
}
