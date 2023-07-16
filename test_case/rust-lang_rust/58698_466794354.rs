
failures:

---- [ui] ui\issues\issue-45107-unnecessary-unsafe-in-closure.rs stdout ----
diff of stderr:

1       error: unnecessary `unsafe` block
-         --> $DIR/issue-45107-unnecessary-unsafe-in-closure.rs:7:13
+         --> $DIR/issue-45107-unnecessary-unsafe-in-closure.rs:9:38
3          |
4       LL |     unsafe {
5          |     ------ because it's nested under this `unsafe` block

-       LL |         let f = |v: &mut Vec<_>| {
-       LL |             unsafe { //~ ERROR unnecessary `unsafe`
-          |             ^^^^^^ unnecessary `unsafe` block
+       ...
+       LL |                 |w: &mut Vec<u32>| { unsafe { //~ ERROR unnecessary `unsafe`
+          |                                      ^^^^^^ unnecessary `unsafe` block
9          |
10      note: lint level defined here
11        --> $DIR/issue-45107-unnecessary-unsafe-in-closure.rs:1:8

14         |        ^^^^^^^^^^^^^
15
16      error: unnecessary `unsafe` block
-         --> $DIR/issue-45107-unnecessary-unsafe-in-closure.rs:9:38
+         --> $DIR/issue-45107-unnecessary-unsafe-in-closure.rs:7:13
18         |
19      LL |     unsafe {
20         |     ------ because it's nested under this `unsafe` block

-       ...
-       LL |                 |w: &mut Vec<u32>| { unsafe { //~ ERROR unnecessary `unsafe`
-          |                                      ^^^^^^ unnecessary `unsafe` block
+       LL |         let f = |v: &mut Vec<_>| {
+       LL |             unsafe { //~ ERROR unnecessary `unsafe`
+          |             ^^^^^^ unnecessary `unsafe` block
24
25      error: unnecessary `unsafe` block
26        --> $DIR/issue-45107-unnecessary-unsafe-in-closure.rs:13:34

---- [ui] ui\span\lint-unused-unsafe.rs stdout ----
diff of stderr:

47         |         ^^^^^^ unnecessary `unsafe` block
48
49      error: unnecessary `unsafe` block
-         --> $DIR/lint-unused-unsafe.rs:29:5
+         --> $DIR/lint-unused-unsafe.rs:30:9
51         |
52      LL | unsafe fn bad7() {
53         | ---------------- because it's nested under this `unsafe` fn

54      LL |     unsafe {                             //~ ERROR: unnecessary `unsafe` block
-          |     ^^^^^^ unnecessary `unsafe` block
+       LL |         unsafe {                         //~ ERROR: unnecessary `unsafe` block
+          |         ^^^^^^ unnecessary `unsafe` block
56
57      error: unnecessary `unsafe` block
-         --> $DIR/lint-unused-unsafe.rs:30:9
+         --> $DIR/lint-unused-unsafe.rs:29:5
59         |
60      LL | unsafe fn bad7() {
61         | ---------------- because it's nested under this `unsafe` fn

62      LL |     unsafe {                             //~ ERROR: unnecessary `unsafe` block
-       LL |         unsafe {                         //~ ERROR: unnecessary `unsafe` block
-          |         ^^^^^^ unnecessary `unsafe` block
+          |     ^^^^^^ unnecessary `unsafe` block
65
66      error: aborting due to 8 previous errors
67

failures:
    [ui] ui\issues\issue-45107-unnecessary-unsafe-in-closure.rs
    [ui] ui\span\lint-unused-unsafe.rs
