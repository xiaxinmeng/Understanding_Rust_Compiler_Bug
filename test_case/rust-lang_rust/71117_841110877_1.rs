rust
// WARNING: This output format is intended for human consumers only
// and is subject to change without notice. Knock yourself out.
fn bar(_1: Vec<u8>) -> () {
    debug a => _1;                       // in scope 0 at <anon>:1:30: 1:31
    let mut _0: ();                      // return place in scope 0 at <anon>:1:42: 1:42
    let _2: ();                          // in scope 0 at <anon>:1:44: 1:50
    let mut _3: std::vec::Vec<u8>;       // in scope 0 at <anon>:1:48: 1:49

    bb0: {
        _3 = move _1;                    // scope 0 at <anon>:1:48: 1:49
        _2 = foo(move _3) -> bb1;        // scope 0 at <anon>:1:44: 1:50
                                         // mir::Constant
                                         // + span: <anon>:1:44: 1:47
                                         // + literal: Const { ty: fn(std::vec::Vec<u8>) {foo}, val: Value(Scalar(<ZST>)) }
    }

    bb1: {
        _0 = const ();                   // scope 0 at <anon>:1:42: 1:53
        return;                          // scope 0 at <anon>:1:53: 1:53
    }
}

fn foo(_1: Vec<u8>) -> () {
    debug a => _1;                       // in scope 0 at <anon>:1:8: 1:9
    let mut _0: ();                      // return place in scope 0 at <anon>:1:20: 1:20

    bb0: {
        _0 = const ();                   // scope 0 at <anon>:1:20: 1:22
        drop(_1) -> bb1;                 // scope 0 at <anon>:1:21: 1:22
    }

    bb1: {
        return;                          // scope 0 at <anon>:1:22: 1:22
    }
}
