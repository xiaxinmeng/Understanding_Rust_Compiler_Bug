plain
Some tests failed in compiletest suite=mir-opt mode=mir-opt host=x86_64-unknown-linux-gnu target=i686-unknown-linux-gnu
.........
failures:

---- [mir-opt] src/test/mir-opt/simplify-locals-removes-unused-discriminant-reads.rs stdout ----
29           Deinit(_0);                      // scope 1 at $DIR/simplify-locals-removes-unused-discriminant-reads.rs:+3:20: +3:27
30           ((_0 as Some).0: std::boxed::Box<()>) = move _4; // scope 1 at $DIR/simplify-locals-removes-unused-discriminant-reads.rs:+3:20: +3:27
31           discriminant(_0) = 1;            // scope 1 at $DIR/simplify-locals-removes-unused-discriminant-reads.rs:+3:20: +3:27
- <<<<<<< HEAD
- =======
34           StorageDead(_4);                 // scope 1 at $DIR/simplify-locals-removes-unused-discriminant-reads.rs:+3:26: +3:27
35           StorageDead(_3);                 // scope 0 at $DIR/simplify-locals-removes-unused-discriminant-reads.rs:+3:26: +3:27
- >>>>>>> bda64a11052 (Simplify MIR opt tests)
37           goto -> bb4;                     // scope 0 at $DIR/simplify-locals-removes-unused-discriminant-reads.rs:+3:26: +3:27
39   

48       }
49   
49   
50       bb4: {
- <<<<<<< HEAD
- =======
53 -         _6 = discriminant(_1);           // scope 0 at $DIR/simplify-locals-removes-unused-discriminant-reads.rs:+5:1: +5:2
- >>>>>>> bda64a11052 (Simplify MIR opt tests)
55           return;                          // scope 0 at $DIR/simplify-locals-removes-unused-discriminant-reads.rs:+5:2: +5:2
57   }


thread '[mir-opt] src/test/mir-opt/simplify-locals-removes-unused-discriminant-reads.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/src/test/mir-opt/simplify_locals_removes_unused_discriminant_reads.map.SimplifyLocals.32bit.diff', src/tools/compiletest/src/runtest.rs:3516:25


failures:
    [mir-opt] src/test/mir-opt/simplify-locals-removes-unused-discriminant-reads.rs
