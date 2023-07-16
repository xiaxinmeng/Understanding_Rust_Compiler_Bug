rust
fn f(_1: !) -> ! {
    debug x => _1;                       // in scope 0 at src/main.rs:13:6: 13:7
    let mut _0: !;                       // return place in scope 0 at src/main.rs:13:15: 13:16

    bb0: {
        unreachable;                     // scope 0 at src/main.rs:13:19: 13:20
    }
}

fn main() -> () {
    let mut _0: ();                      // return place in scope 0 at src/main.rs:4:11: 4:11
    let _1: &i32;                        // in scope 0 at src/main.rs:5:9: 5:10
    let mut _3: *const !;                // in scope 0 at src/main.rs:7:10: 7:37
    let mut _4: *const i32;              // in scope 0 at src/main.rs:7:11: 7:24
    let mut _5: *const i32;              // in scope 0 at src/main.rs:7:11: 7:24
    let mut _6: &i32;                    // in scope 0 at src/main.rs:5:13: 5:15
    scope 1 {
        debug y => _1;                   // in scope 1 at src/main.rs:5:9: 5:10
        let _2: ! as UserTypeProjection { base: UserType(0), projs: [] }; // in scope 1 at src/main.rs:6:9: 6:10
        scope 2 {
            debug x => _2;               // in scope 2 at src/main.rs:6:9: 6:10
        }
        scope 3 {
        }
    }

    bb0: {
        StorageLive(_1);                 // scope 0 at src/main.rs:5:9: 5:10
        _6 = const main::promoted[0];    // scope 0 at src/main.rs:5:13: 5:15
                                         // ty::Const
                                         // + ty: &i32
                                         // + val: Unevaluated(DefId(0:3 ~ playground[c63f]::main[0]), [], Some(promoted[0]))
                                         // mir::Constant
                                         // + span: src/main.rs:5:13: 5:15
                                         // + literal: Const { ty: &i32, val: Unevaluated(DefId(0:3 ~ playground[c63f]::main[0]), [], Some(promoted[0])) }
        _1 = _6;                         // scope 0 at src/main.rs:5:13: 5:15
        StorageLive(_2);                 // scope 1 at src/main.rs:6:9: 6:10
        StorageLive(_3);                 // scope 3 at src/main.rs:7:10: 7:37
        StorageLive(_4);                 // scope 3 at src/main.rs:7:11: 7:24
        StorageLive(_5);                 // scope 3 at src/main.rs:7:11: 7:24
        _5 = &raw const (*_1);           // scope 3 at src/main.rs:7:11: 7:12
        _4 = _5;                         // scope 3 at src/main.rs:7:11: 7:24
        _3 = move _4 as *const ! (Misc); // scope 3 at src/main.rs:7:10: 7:37
        StorageDead(_4);                 // scope 3 at src/main.rs:7:36: 7:37
        unreachable;                     // scope 3 at src/main.rs:7:9: 7:37
    }
}

promoted[0] in main: &i32 = {
    let mut _0: &i32;                    // return place in scope 0 at src/main.rs:5:13: 5:15
    let mut _1: i32;                     // in scope 0 at src/main.rs:5:14: 5:15
    scope 1 {
        scope 2 {
        }
        scope 3 {
        }
    }

    bb0: {
        _1 = const 5i32;                 // scope 0 at src/main.rs:5:14: 5:15
                                         // ty::Const
                                         // + ty: i32
                                         // + val: Value(Scalar(0x00000005))
                                         // mir::Constant
                                         // + span: src/main.rs:5:14: 5:15
                                         // + literal: Const { ty: i32, val: Value(Scalar(0x00000005)) }
        _0 = &_1;                        // scope 0 at src/main.rs:5:13: 5:15
        return;                          // scope 0 at src/main.rs:5:13: 5:15
    }
}
