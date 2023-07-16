
// MIR for `foo`
// source = Fn(NodeId(4))
// pass_name = NLL
// disambiguator = after

fn foo(_1: std::boxed::Box<isize>) -> isize {
    let mut _0: isize;                   // return pointer
    scope 1 {
        let _2: std::boxed::Box<isize>;  // "x" in scope 1 at ../src/test/compile-fail/borrowck/borrowck-unary-move.rs:12:8: 12:9
        scope 2 {
            let _3: &'13_0rs isize;      // "y" in scope 2 at ../src/test/compile-fail/borrowck/borrowck-unary-move.rs:13:9: 13:10
        }
        scope 3 {
        }
    }
    let mut _4: ();
    let mut _5: std::boxed::Box<isize>;
    let mut _6: isize;

    bb0: {
        StorageLive(_2);                 // scope 1 at ../src/test/compile-fail/borrowck/borrowck-unary-move.rs:12:8: 12:9
        _2 = _1;                         // scope 1 at ../src/test/compile-fail/borrowck/borrowck-unary-move.rs:12:8: 12:9
        StorageLive(_3);                 // scope 2 at ../src/test/compile-fail/borrowck/borrowck-unary-move.rs:13:9: 13:10
        _3 = &'13_0rs (*_2);             // scope 2 at ../src/test/compile-fail/borrowck/borrowck-unary-move.rs:13:13: 13:16
        StorageLive(_5);                 // scope 2 at ../src/test/compile-fail/borrowck/borrowck-unary-move.rs:14:10: 14:11
        _5 = _2;                         // scope 2 at ../src/test/compile-fail/borrowck/borrowck-unary-move.rs:14:10: 14:11
        _4 = const free(_5) -> [return: bb1, unwind: bb6]; // scope 2 at ../src/test/compile-fail/borrowck/borrowck-unary-move.rs:14:5: 14:12
    }

    bb1: {
        drop(_5) -> [return: bb7, unwind: bb4]; // scope 2 at ../src/test/compile-fail/borrowck/borrowck-unary-move.rs:14:12: 14:12
    }

    bb2: {                               // cleanup
        resume;                          // scope 0 at ../src/test/compile-fail/borrowck/borrowck-unary-move.rs:12:1: 16:2
    }

    bb3: {                               // cleanup
        drop(_2) -> bb2;                 // scope 0 at ../src/test/compile-fail/borrowck/borrowck-unary-move.rs:16:2: 16:2
    }

    bb4: {                               // cleanup
        drop(_1) -> bb3;                 // scope 0 at ../src/test/compile-fail/borrowck/borrowck-unary-move.rs:16:2: 16:2
    }

    bb5: {                               // cleanup
        EndRegion('13_0rs);              // scope 1 at ../src/test/compile-fail/borrowck/borrowck-unary-move.rs:12:32: 16:2
        goto -> bb4;                     // scope 1 at ../src/test/compile-fail/borrowck/borrowck-unary-move.rs:12:32: 16:2
    }

    bb6: {                               // cleanup
        drop(_5) -> bb5;                 // scope 2 at ../src/test/compile-fail/borrowck/borrowck-unary-move.rs:14:12: 14:12
    }

    bb7: {
        StorageDead(_5);                 // scope 2 at ../src/test/compile-fail/borrowck/borrowck-unary-move.rs:14:12: 14:12
        StorageLive(_6);                 // scope 2 at ../src/test/compile-fail/borrowck/borrowck-unary-move.rs:15:5: 15:7
        _6 = (*_3);                      // scope 2 at ../src/test/compile-fail/borrowck/borrowck-unary-move.rs:15:5: 15:7
        _0 = _6;                         // scope 2 at ../src/test/compile-fail/borrowck/borrowck-unary-move.rs:15:5: 15:7
        StorageDead(_6);                 // scope 2 at ../src/test/compile-fail/borrowck/borrowck-unary-move.rs:15:7: 15:7
        EndRegion('13_0rs);              // scope 1 at ../src/test/compile-fail/borrowck/borrowck-unary-move.rs:12:32: 16:2
        StorageDead(_3);                 // scope 1 at ../src/test/compile-fail/borrowck/borrowck-unary-move.rs:16:2: 16:2
        drop(_1) -> [return: bb8, unwind: bb3]; // scope 0 at ../src/test/compile-fail/borrowck/borrowck-unary-move.rs:16:2: 16:2
    }

    bb8: {
        drop(_2) -> bb9;                 // scope 0 at ../src/test/compile-fail/borrowck/borrowck-unary-move.rs:16:2: 16:2
    }

    bb9: {
        StorageDead(_2);                 // scope 0 at ../src/test/compile-fail/borrowck/borrowck-unary-move.rs:16:2: 16:2
        return;                          // scope 0 at ../src/test/compile-fail/borrowck/borrowck-unary-move.rs:16:2: 16:2
    }
}
