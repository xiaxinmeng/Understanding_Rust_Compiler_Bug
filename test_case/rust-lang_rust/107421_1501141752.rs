plain
 finished in 0.488 seconds
Check compiletest suite=mir-opt mode=mir-opt (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 244 tests
...........F.................................................................i..........  88/244
......................................................i...........ii.iF................. 176/244

failures:

---- [mir-opt] tests/mir-opt/building/async_await.rs stdout ----
---- [mir-opt] tests/mir-opt/building/async_await.rs stdout ----
185         StorageDead(_12);                // scope 1 at $DIR/async_await.rs:+1:13: +1:14
186         StorageDead(_9);                 // scope 1 at $DIR/async_await.rs:+1:13: +1:14
187         StorageDead(_8);                 // scope 1 at $DIR/async_await.rs:+1:13: +1:14
-         drop((((*(_1.0: &mut [async fn body@$DIR/async_await.rs:14:18: 17:2])) as variant#3).0: impl std::future::Future<Output = ()>)) -> bb12; // scope 0 at $DIR/async_await.rs:+1:13: +1:14
+         drop((((*(_1.0: &mut [async fn body@$DIR/async_await.rs:14:18: 17:2])) as variant#3).0: impl std::future::Future<Output = ()>)) -> [return: bb12, unwind unreachable]; // scope 0 at $DIR/async_await.rs:+1:13: +1:14
190 
191     bb11: {


289         StorageDead(_28);                // scope 4 at $DIR/async_await.rs:+2:13: +2:14
290         StorageDead(_25);                // scope 4 at $DIR/async_await.rs:+2:13: +2:14
291         StorageDead(_24);                // scope 4 at $DIR/async_await.rs:+2:13: +2:14
-         drop((((*(_1.0: &mut [async fn body@$DIR/async_await.rs:14:18: 17:2])) as variant#4).0: impl std::future::Future<Output = ()>)) -> bb23; // scope 0 at $DIR/async_await.rs:+2:13: +2:14
+         drop((((*(_1.0: &mut [async fn body@$DIR/async_await.rs:14:18: 17:2])) as variant#4).0: impl std::future::Future<Output = ()>)) -> [return: bb23, unwind unreachable]; // scope 0 at $DIR/async_await.rs:+2:13: +2:14
294 
295     bb22: {


thread '[mir-opt] tests/mir-opt/building/async_await.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/tests/mir-opt/building/async_await.b-{closure#0}.generator_resume.0.mir', src/tools/compiletest/src/runtest.rs:3518:21

---- [mir-opt] tests/mir-opt/inline/inline_generator.rs stdout ----
92       }
93   
93   
94 -     bb5 (cleanup): {
- -         drop(_4) -> bb6;                 // scope 0 at $DIR/inline_generator.rs:+1:46: +1:47
+ -         drop(_4) -> [return: bb6, unwind terminate]; // scope 0 at $DIR/inline_generator.rs:+1:46: +1:47
96 +     bb3 (cleanup): {
- +         drop(_4) -> bb4;                 // scope 0 at $DIR/inline_generator.rs:+1:46: +1:47
+ +         drop(_4) -> [return: bb4, unwind terminate]; // scope 0 at $DIR/inline_generator.rs:+1:46: +1:47
99   
99   
100 -     bb6 (cleanup): {

thread '[mir-opt] tests/mir-opt/inline/inline_generator.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/tests/mir-opt/inline/inline_generator.main.Inline.diff', src/tools/compiletest/src/runtest.rs:3518:21

failures:
    [mir-opt] tests/mir-opt/building/async_await.rs
    [mir-opt] tests/mir-opt/inline/inline_generator.rs
