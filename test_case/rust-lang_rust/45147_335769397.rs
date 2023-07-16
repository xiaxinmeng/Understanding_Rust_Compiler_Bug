
// WARNING: This output format is intended for human consumers only
// and is subject to change without notice. Knock yourself out.
fn vert() -> () {
    let mut _0: ();                      // return pointer
    scope 1 {
        let _1: Test;                    // "t" in scope 1 at rlsl-example/src/main.rs:78:9: 78:10
        scope 3 {
            let _2: f32;                 // "f" in scope 3 at rlsl-example/src/main.rs:82:9: 82:10
            scope 5 {
                let _3: f32;             // "x" in scope 5 at rlsl-example/src/main.rs:83:9: 83:10
            }
            scope 6 {
            }
            scope 7 {
            }
        }
        scope 4 {
        }
        scope 9 {
        }
    }
    scope 2 {
    }
    scope 8 {
    }
    let mut _4: Test;

    bb0: {                              
        StorageLive(_1);                 // scope 1 at rlsl-example/src/main.rs:78:9: 78:10
        (_1.0: f32) = const 1f32;        // scope 1 at rlsl-example/src/main.rs:78:13: 81:6
        (_1.1: f32) = const 2f32;        // scope 1 at rlsl-example/src/main.rs:78:13: 81:6
        StorageLive(_2);                 // scope 3 at rlsl-example/src/main.rs:82:9: 82:10
        nop;                             // scope 3 at rlsl-example/src/main.rs:82:13: 82:26
        nop;                             // scope 8 at /home/maik/projects/rlsl/libstd/src/lib.rs:51:21: 51:25
        nop;                             // scope 8 at /home/maik/projects/rlsl/libstd/src/lib.rs:51:21: 51:25
        nop;                             // scope 9 at /home/maik/projects/rlsl/libstd/src/lib.rs:52:42: 52:46
        nop;                             // scope 9 at /home/maik/projects/rlsl/libstd/src/lib.rs:52:42: 52:46
        _2 = const std::intrinsics::sqrtf32(const 1f32) -> bb2; // scope 9 at /home/maik/projects/rlsl/libstd/src/lib.rs:52:22: 52:47
    }

    bb1: {                              
        StorageDead(_4);                 // scope 5 at rlsl-example/src/main.rs:83:20: 83:20
        _0 = ();                         // scope 0 at rlsl-example/src/main.rs:77:11: 118:2
        StorageDead(_3);                 // scope 3 at rlsl-example/src/main.rs:118:2: 118:2
        StorageDead(_2);                 // scope 1 at rlsl-example/src/main.rs:118:2: 118:2
        StorageDead(_1);                 // scope 0 at rlsl-example/src/main.rs:118:2: 118:2
        return;                          // scope 0 at rlsl-example/src/main.rs:118:2: 118:2
    }

    bb2: {                              
        nop;                             // scope 9 at /home/maik/projects/rlsl/libstd/src/lib.rs:52:47: 52:47
        nop;                             // scope 7 at /home/maik/projects/rlsl/libstd/src/lib.rs:53:10: 53:10
        StorageLive(_3);                 // scope 5 at rlsl-example/src/main.rs:83:9: 83:10
        StorageLive(_4);                 // scope 5 at rlsl-example/src/main.rs:83:13: 83:14
        _4 = _1;                         // scope 5 at rlsl-example/src/main.rs:83:13: 83:14
        _3 = const Foo::get(_4) -> bb1;  // scope 5 at rlsl-example/src/main.rs:83:13: 83:20
    }
}
...
