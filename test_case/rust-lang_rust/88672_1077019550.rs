plain
............................................................i.i..................................... 8300/12764
...............................................iiii................................................. 8400/12764
....................................................................i............................... 8500/12764
........i...................................................................i....................... 8600/12764
................................................................F.F................................. 8700/12764
.................................................................................................... 8900/12764
............i....................................................................................... 9000/12764
.................................................................................................... 9100/12764
.................................................................................................... 9200/12764
---
...........................................................iii...................................... 12700/12764
................................................................
failures:

---- [ui] ui/parser/increment-autofix.rs stdout ----

14   --> $DIR/increment-autofix.rs:11:11
15    |
15    |
16 LL |     while ++i < 5 {
-    |           ^^ not a valid prefix operator
+    |     ----- ^^ not a valid prefix operator
+    |     |
+    |     while parsing the condition of this `while` expression
18    |
19 help: use `+= 1` instead

37   --> $DIR/increment-autofix.rs:25:11
38    |
38    |
39 LL |     while ++tmp < 5 {
-    |           ^^ not a valid prefix operator
+    |     ----- ^^ not a valid prefix operator
+    |     |
+    |     while parsing the condition of this `while` expression
41    |
42 help: use `+= 1` instead


The actual stderr differed from the expected stderr.
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/increment-autofix/increment-autofix.stderr
To only update this specific test, also pass `--test-args parser/increment-autofix.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/parser/increment-autofix.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/increment-autofix" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/increment-autofix/auxiliary"
stdout: none
--- stderr -------------------------------
error: Rust has no prefix increment operator
  --> /checkout/src/test/ui/parser/increment-autofix.rs:5:5
   |
LL |     ++i; //~ ERROR Rust has no prefix increment operator
   |     ^^ not a valid prefix operator
   |
help: use `+= 1` instead
   |
LL -     ++i; //~ ERROR Rust has no prefix increment operator
LL +     i += 1; //~ ERROR Rust has no prefix increment operator

error: Rust has no prefix increment operator
  --> /checkout/src/test/ui/parser/increment-autofix.rs:11:11
   |
   |
LL |     while ++i < 5 {
   |     ----- ^^ not a valid prefix operator
   |     |
   |     while parsing the condition of this `while` expression
   |
help: use `+= 1` instead
   |
LL |     while { i += 1; i } < 5 {
   |           ~   +++++++++
error: Rust has no prefix increment operator
  --> /checkout/src/test/ui/parser/increment-autofix.rs:19:5
   |
   |
LL |     ++tmp; //~ ERROR Rust has no prefix increment operator
   |     ^^ not a valid prefix operator
   |
help: use `+= 1` instead
   |
LL -     ++tmp; //~ ERROR Rust has no prefix increment operator
LL +     tmp += 1; //~ ERROR Rust has no prefix increment operator

error: Rust has no prefix increment operator
  --> /checkout/src/test/ui/parser/increment-autofix.rs:25:11
   |
   |
LL |     while ++tmp < 5 {
   |     ----- ^^ not a valid prefix operator
   |     |
   |     while parsing the condition of this `while` expression
   |
help: use `+= 1` instead
   |
LL |     while { tmp += 1; tmp } < 5 {

error: aborting due to 4 previous errors
------------------------------------------



---- [ui] ui/parser/increment-notfixed.rs stdout ----

16   --> $DIR/increment-notfixed.rs:17:12
17    |
17    |
18 LL |     while i++ < 5 {
-    |            ^^ not a valid postfix operator
+    |     -----  ^^ not a valid postfix operator
+    |     |
+    |     while parsing the condition of this `while` expression
20    |
21 help: use `+= 1` instead

44   --> $DIR/increment-notfixed.rs:31:14
45    |
45    |
46 LL |     while tmp++ < 5 {
-    |              ^^ not a valid postfix operator
+    |     -----    ^^ not a valid postfix operator
+    |     |
+    |     while parsing the condition of this `while` expression
48    |
49 help: use `+= 1` instead


The actual stderr differed from the expected stderr.
The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/increment-notfixed/increment-notfixed.stderr
To only update this specific test, also pass `--test-args parser/increment-notfixed.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/parser/increment-notfixed.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/increment-notfixed" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/increment-notfixed/auxiliary"
stdout: none
--- stderr -------------------------------
error: Rust has no postfix increment operator
  --> /checkout/src/test/ui/parser/increment-notfixed.rs:11:6
   |
LL |     i++; //~ ERROR Rust has no postfix increment operator
   |      ^^ not a valid postfix operator
   |
help: use `+= 1` instead
   |
LL |     { let tmp = i; i += 1; tmp }; //~ ERROR Rust has no postfix increment operator
   |     +++++++++++  ~~~~~~~~~~~~~~~
LL -     i++; //~ ERROR Rust has no postfix increment operator
LL +     i += 1; //~ ERROR Rust has no postfix increment operator

error: Rust has no postfix increment operator
  --> /checkout/src/test/ui/parser/increment-notfixed.rs:17:12
   |
   |
LL |     while i++ < 5 {
   |     -----  ^^ not a valid postfix operator
   |     |
   |     while parsing the condition of this `while` expression
   |
help: use `+= 1` instead
   |
LL |     while { let tmp = i; i += 1; tmp } < 5 {
   |           +++++++++++  ~~~~~~~~~~~~~~~
LL -     while i++ < 5 {
LL +     while i += 1 < 5 {

error: Rust has no postfix increment operator
  --> /checkout/src/test/ui/parser/increment-notfixed.rs:25:8
   |
   |
LL |     tmp++; //~ ERROR Rust has no postfix increment operator
   |        ^^ not a valid postfix operator
   |
help: use `+= 1` instead
   |
LL |     { let tmp_ = tmp; tmp += 1; tmp_ }; //~ ERROR Rust has no postfix increment operator
   |     ++++++++++++    ~~~~~~~~~~~~~~~~~~
LL -     tmp++; //~ ERROR Rust has no postfix increment operator
LL +     tmp += 1; //~ ERROR Rust has no postfix increment operator

error: Rust has no postfix increment operator
  --> /checkout/src/test/ui/parser/increment-notfixed.rs:31:14
   |
   |
LL |     while tmp++ < 5 {
   |     -----    ^^ not a valid postfix operator
   |     |
   |     while parsing the condition of this `while` expression
   |
help: use `+= 1` instead
   |
LL |     while { let tmp_ = tmp; tmp += 1; tmp_ } < 5 {
   |           ++++++++++++    ~~~~~~~~~~~~~~~~~~
LL -     while tmp++ < 5 {
LL +     while tmp += 1 < 5 {

error: Rust has no postfix increment operator
  --> /checkout/src/test/ui/parser/increment-notfixed.rs:39:16
   |
   |
LL |     foo.bar.qux++;
   |                ^^ not a valid postfix operator
   |
help: use `+= 1` instead
   |
LL |     { let tmp = foo.bar.qux; foo.bar.qux += 1; tmp };
   |     +++++++++++            ~~~~~~~~~~~~~~~~~~~~~~~~~
LL -     foo.bar.qux++;
LL +     foo.bar.qux += 1;

error: Rust has no postfix increment operator
  --> /checkout/src/test/ui/parser/increment-notfixed.rs:49:10
   |
   |
LL |     s.tmp++;
   |          ^^ not a valid postfix operator
   |
help: use `+= 1` instead
   |
LL |     { let tmp = s.tmp; s.tmp += 1; tmp };
   |     +++++++++++      ~~~~~~~~~~~~~~~~~~~
LL -     s.tmp++;
LL +     s.tmp += 1;

error: Rust has no prefix increment operator
  --> /checkout/src/test/ui/parser/increment-notfixed.rs:56:5
   |
   |
LL |     ++foo.bar.qux;
   |     ^^ not a valid prefix operator
   |
help: use `+= 1` instead
   |
LL -     ++foo.bar.qux;
LL +     foo.bar.qux += 1;

error: aborting due to 7 previous errors
------------------------------------------

