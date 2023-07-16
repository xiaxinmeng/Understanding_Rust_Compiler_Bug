plain
test [mir-opt] src/test/mir-opt/inline/polymorphic_recursion.rs ... ok

failures:

---- [mir-opt] src/test/mir-opt/sroa.rs stdout ----
29           StorageDead(_4);                 // scope 0 at $DIR/sroa.rs:+1:29: +1:30
30           StorageDead(_3);                 // scope 0 at $DIR/sroa.rs:+1:29: +1:30
31           _1 = move (_2.1: Tag);           // scope 0 at $DIR/sroa.rs:+1:5: +1:32
-           drop(_1) -> [return: bb1, unwind: bb2]; // scope 0 at $DIR/sroa.rs:+1:32: +1:33
+           drop(_1) -> bb1;                 // scope 0 at $DIR/sroa.rs:+1:32: +1:33
34   
35       bb1: {


-           drop((_2.0: Tag)) -> [return: bb6, unwind: bb5]; // scope 0 at $DIR/sroa.rs:+1:32: +1:33
+           drop((_2.0: Tag)) -> bb3;        // scope 0 at $DIR/sroa.rs:+1:32: +1:33
38   
38   
-       bb2 (cleanup): {
-           drop((_2.0: Tag)) -> bb7;        // scope 0 at $DIR/sroa.rs:+1:32: +1:33
-   
-   
-       bb3 (cleanup): {
-           resume;                          // scope 0 at $DIR/sroa.rs:+0:1: +2:2
-   
-       bb4: {
+       bb2: {
+       bb2: {
48           StorageDead(_2);                 // scope 0 at $DIR/sroa.rs:+1:32: +1:33
49           StorageDead(_1);                 // scope 0 at $DIR/sroa.rs:+1:32: +1:33
50           _0 = const ();                   // scope 0 at $DIR/sroa.rs:+0:19: +2:2

51           return;                          // scope 0 at $DIR/sroa.rs:+2:2: +2:2
53   
53   
-       bb5 (cleanup): {
-           drop((_2.2: Tag)) -> bb3;        // scope 0 at $DIR/sroa.rs:+1:32: +1:33
-   
-       bb6: {
-       bb6: {
-           drop((_2.2: Tag)) -> [return: bb4, unwind: bb3]; // scope 0 at $DIR/sroa.rs:+1:32: +1:33
-   
-   
-       bb7 (cleanup): {
-           drop((_2.2: Tag)) -> bb3;        // scope 0 at $DIR/sroa.rs:+1:32: +1:33
+       bb3: {
+           drop((_2.2: Tag)) -> bb2;        // scope 0 at $DIR/sroa.rs:+1:32: +1:33
65   }
66   


thread '[mir-opt] src/test/mir-opt/sroa.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/src/test/mir-opt/sroa.dropping.ScalarReplacementOfAggregates.diff', src/tools/compiletest/src/runtest.rs:3447:21


failures:
    [mir-opt] src/test/mir-opt/sroa.rs
