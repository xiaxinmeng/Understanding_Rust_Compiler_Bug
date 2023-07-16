plain
failures:

---- [mir-opt] mir-opt/simplify-locals.rs stdout ----
15   
16       bb0: {
17           StorageLive(_1);                 // scope 0 at $DIR/simplify-locals.rs:14:9: 14:14
-           _1 = [const 0_u8; 10];           // scope 0 at $DIR/simplify-locals.rs:14:17: 14:26
19 -         StorageLive(_2);                 // scope 1 at $DIR/simplify-locals.rs:16:20: 16:26
20 -         StorageLive(_3);                 // scope 1 at $DIR/simplify-locals.rs:16:20: 16:26
21 -         StorageLive(_4);                 // scope 1 at $DIR/simplify-locals.rs:16:20: 16:26

- -         _4 = &_1;                        // scope 1 at $DIR/simplify-locals.rs:16:20: 16:26
- -         _3 = _4;                         // scope 1 at $DIR/simplify-locals.rs:16:20: 16:26
- -         _2 = move _3 as &[u8] (Pointer(Unsize)); // scope 1 at $DIR/simplify-locals.rs:16:20: 16:26
25 -         StorageDead(_3);                 // scope 1 at $DIR/simplify-locals.rs:16:25: 16:26
26 -         StorageDead(_4);                 // scope 1 at $DIR/simplify-locals.rs:16:26: 16:27
27 -         StorageDead(_2);                 // scope 1 at $DIR/simplify-locals.rs:16:26: 16:27

thread '[mir-opt] mir-opt/simplify-locals.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/src/test/mir-opt/simplify_locals.c.SimplifyLocals.diff', src/tools/compiletest/src/runtest.rs:3375:25


failures:
    [mir-opt] mir-opt/simplify-locals.rs
