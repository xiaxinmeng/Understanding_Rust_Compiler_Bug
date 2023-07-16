plain
.................................................................................................... 4300/11472
.......................................................ii........................................... 4400/11472
........................................i........................................................... 4500/11472
.................................................................................................... 4600/11472
.......................................................................................F............ 4700/11472
.............................................FF.........F........................................... 4800/11472
......F............................................................................................. 5000/11472
.................................................................................................... 5100/11472
...........................................................i..i..................................... 5200/11472
........F........................................................................................... 5300/11472
---
.................................................i.................................................. 7100/11472
................................................ii................i.i..ii........................... 7200/11472
.................................................................................................... 7300/11472
.................................................................................................... 7400/11472
...F.................F.............................................................................. 7500/11472
...........................F......................................................................i. 7600/11472
............................................................................i........i.............. 7800/11472
.....................................................................i.............................. 7900/11472
..............................................................i..................................... 8000/11472
.................................................................................................... 8100/11472
---
.................................................................................................... 9200/11472
.................................................................................................... 9300/11472
.................................................................................................... 9400/11472
.............................i......i............................................................... 9500/11472
....................................................................iiiiiii..iiiiii.i............... 9600/11472
.................................................................................................... 9800/11472
.................................................................................................... 9900/11472
.................................................................................................... 10000/11472
.................................................................................................... 10100/11472
---

4 LL |         self.a();
5    |              ^
6    |
- note: first, the lifetime cannot outlive the anonymous lifetime #1 defined on the method body at 3:5...
-   --> $DIR/issue-16683.rs:3:5
+ note: first, the lifetime cannot outlive the anonymous lifetime defined on the method body at 3:10...
9    |
10 LL |     fn b(&self) {
-    |     ^^^^^^^^^^^
+    |          ^^^^^
+    |          ^^^^^
12 note: ...so that reference does not outlive borrowed content
14    |


The actual stderr differed from the expected stderr.
The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-16683/issue-16683.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args issues/issue-16683.rs`
error: 1 errors occurred comparing output.
status: exit code: 1
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues/issue-16683.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-16683" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-16683/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0495]: cannot infer an appropriate lifetime for autoref due to conflicting requirements
   |
   |
LL |         self.a(); //~ ERROR cannot infer
   |
   |
note: first, the lifetime cannot outlive the anonymous lifetime defined on the method body at 3:10...
   |
LL |     fn b(&self) {
   |          ^^^^^
   |          ^^^^^
note: ...so that reference does not outlive borrowed content
   |
   |
LL |         self.a(); //~ ERROR cannot infer
   |         ^^^^
note: but, the lifetime must be valid for the lifetime `'a` as defined on the trait at 1:9...
   |
   |
LL | trait T<'a> {
   |         ^^
note: ...so that the types are compatible
   |
   |
LL |         self.a(); //~ ERROR cannot infer
   |              ^
   = note: expected `&'a Self`
              found `&Self`
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
error: aborting due to previous error

For more information about this error, try `rustc --explain E0495`.
---
---- [ui] ui/issues/issue-17740.rs stdout ----
diff of stderr:

6    |
7    = note: expected struct `Foo<'a>`
8               found struct `Foo<'_>`
- note: the anonymous lifetime #2 defined on the method body at 6:5...
-   --> $DIR/issue-17740.rs:6:5
+ note: the anonymous lifetime defined on the method body at 6:23...
11    |
11    |
12 LL |     fn bar(self: &mut Foo) {
+    |                       ^^^
+    |                       ^^^
14 note: ...does not necessarily outlive the lifetime `'a` as defined on the impl at 5:7
16    |

30    |
30    |
31 LL | impl <'a> Foo<'a>{
32    |       ^^
- note: ...does not necessarily outlive the anonymous lifetime #2 defined on the method body at 6:5
-   --> $DIR/issue-17740.rs:6:5
+ note: ...does not necessarily outlive the anonymous lifetime defined on the method body at 6:23
35    |
35    |
36 LL |     fn bar(self: &mut Foo) {
+    |                       ^^^
38 
39 error: aborting due to 2 previous errors
40 
40 


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-17740/issue-17740.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args issues/issue-17740.rs`
error: 1 errors occurred comparing output.
status: exit code: 1
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues/issue-17740.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-17740" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-17740/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0308]: mismatched `self` parameter type
   |
   |
LL |     fn bar(self: &mut Foo) {
   |                  ^^^^^^^^ lifetime mismatch
   = note: expected struct `Foo<'a>`
              found struct `Foo<'_>`
              found struct `Foo<'_>`
note: the anonymous lifetime defined on the method body at 6:23...
   |
   |
LL |     fn bar(self: &mut Foo) {
   |                       ^^^
note: ...does not necessarily outlive the lifetime `'a` as defined on the impl at 5:7
   |
   |
LL | impl <'a> Foo<'a>{


error[E0308]: mismatched `self` parameter type
   |
   |
LL |     fn bar(self: &mut Foo) {
   |                  ^^^^^^^^ lifetime mismatch
   = note: expected struct `Foo<'a>`
              found struct `Foo<'_>`
              found struct `Foo<'_>`
note: the lifetime `'a` as defined on the impl at 5:7...
   |
   |
LL | impl <'a> Foo<'a>{
   |       ^^
note: ...does not necessarily outlive the anonymous lifetime defined on the method body at 6:23
   |
   |
LL |     fn bar(self: &mut Foo) {

error: aborting due to 2 previous errors

For more information about this error, try `rustc --explain E0308`.
For more information about this error, try `rustc --explain E0308`.

------------------------------------------


---- [ui] ui/issues/issue-17758.rs stdout ----
diff of stderr:

4 LL |         self.foo();
6    |
6    |
- note: first, the lifetime cannot outlive the anonymous lifetime #1 defined on the method body at 6:5...
-   --> $DIR/issue-17758.rs:6:5
+ note: first, the lifetime cannot outlive the anonymous lifetime defined on the method body at 6:12...
9    |
10 LL |     fn bar(&self) {
-    |     ^^^^^^^^^^^^^
+    |            ^^^^^
+    |            ^^^^^
12 note: ...so that reference does not outlive borrowed content
14    |


The actual stderr differed from the expected stderr.
The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-17758/issue-17758.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args issues/issue-17758.rs`
error: 1 errors occurred comparing output.
status: exit code: 1
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues/issue-17758.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-17758" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-17758/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0495]: cannot infer an appropriate lifetime for autoref due to conflicting requirements
   |
   |
LL |         self.foo();
   |
   |
note: first, the lifetime cannot outlive the anonymous lifetime defined on the method body at 6:12...
   |
LL |     fn bar(&self) {
   |            ^^^^^
   |            ^^^^^
note: ...so that reference does not outlive borrowed content
   |
   |
LL |         self.foo();
   |         ^^^^
note: but, the lifetime must be valid for the lifetime `'a` as defined on the trait at 4:11...
   |
   |
LL | trait Foo<'a> {
   |           ^^
note: ...so that the types are compatible
   |
   |
LL |         self.foo();
   |              ^^^
   = note: expected `&'a Self`
              found `&Self`
error: aborting due to previous error

For more information about this error, try `rustc --explain E0495`.


------------------------------------------


---- [ui] ui/issues/issue-17905-2.rs stdout ----
diff of stderr:

6    |
7    = note: expected struct `Pair<&str, _>`
8               found struct `Pair<&str, _>`
- note: the anonymous lifetime #2 defined on the method body at 8:5...
-   --> $DIR/issue-17905-2.rs:8:5
+ note: the anonymous lifetime defined on the method body at 8:24...
11    |
11    |
12 LL |     fn say(self: &Pair<&str, isize>) {
+    |                        ^^^^
+    |                        ^^^^
14 note: ...does not necessarily outlive the lifetime `'_` as defined on the impl at 5:5
16    |

30    |
31 LL |     &str,
31 LL |     &str,
32    |     ^
- note: ...does not necessarily outlive the anonymous lifetime #2 defined on the method body at 8:5
-   --> $DIR/issue-17905-2.rs:8:5
+ note: ...does not necessarily outlive the anonymous lifetime defined on the method body at 8:24
35    |
35    |
36 LL |     fn say(self: &Pair<&str, isize>) {
+    |                        ^^^^
38 
39 error: aborting due to 2 previous errors
40 
40 


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-17905-2/issue-17905-2.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args issues/issue-17905-2.rs`
error: 1 errors occurred comparing output.
status: exit code: 1
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues/issue-17905-2.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-17905-2" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-17905-2/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0308]: mismatched `self` parameter type
   |
   |
LL |     fn say(self: &Pair<&str, isize>) {
   |                  ^^^^^^^^^^^^^^^^^^ lifetime mismatch
   |
   = note: expected struct `Pair<&str, _>`
              found struct `Pair<&str, _>`
note: the anonymous lifetime defined on the method body at 8:24...
   |
   |
LL |     fn say(self: &Pair<&str, isize>) {
   |                        ^^^^
note: ...does not necessarily outlive the lifetime `'_` as defined on the impl at 5:5
   |
LL |     &str,
   |     ^


error[E0308]: mismatched `self` parameter type
   |
   |
LL |     fn say(self: &Pair<&str, isize>) {
   |                  ^^^^^^^^^^^^^^^^^^ lifetime mismatch
   |
   = note: expected struct `Pair<&str, _>`
              found struct `Pair<&str, _>`
note: the lifetime `'_` as defined on the impl at 5:5...
   |
LL |     &str,
   |     ^
   |     ^
note: ...does not necessarily outlive the anonymous lifetime defined on the method body at 8:24
   |
   |
LL |     fn say(self: &Pair<&str, isize>) {

error: aborting due to 2 previous errors

For more information about this error, try `rustc --explain E0308`.
For more information about this error, try `rustc --explain E0308`.

------------------------------------------


---- [ui] ui/issues/issue-20831-debruijn.rs stdout ----
diff of stderr:

4 LL |     fn subscribe(&mut self, t : Box<dyn Subscriber<Input=<Self as Publisher>::Output> + 'a>) {
6    |
6    |
- note: first, the lifetime cannot outlive the anonymous lifetime #2 defined on the method body at 28:5...
-   --> $DIR/issue-20831-debruijn.rs:28:5
+ note: first, the lifetime cannot outlive the anonymous lifetime defined on the method body at 28:58...
9    |
9    |
10 LL |     fn subscribe(&mut self, t : Box<dyn Subscriber<Input=<Self as Publisher>::Output> + 'a>) {
+    |                                                          ^^^^^^^^^^^^^^^^^^^^^^^^^^^
+    |                                                          ^^^^^^^^^^^^^^^^^^^^^^^^^^^
12 note: ...but the lifetime must also be valid for the lifetime `'a` as defined on the impl at 26:6...
14    |


The actual stderr differed from the expected stderr.
The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-20831-debruijn/issue-20831-debruijn.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args issues/issue-20831-debruijn.rs`
error: 1 errors occurred comparing output.
status: exit code: 1
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues/issue-20831-debruijn.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-20831-debruijn" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-20831-debruijn/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0495]: cannot infer an appropriate lifetime for lifetime parameter `'a` due to conflicting requirements
   |
   |
LL |     fn subscribe(&mut self, t : Box<dyn Subscriber<Input=<Self as Publisher>::Output> + 'a>) {
   |
   |
note: first, the lifetime cannot outlive the anonymous lifetime defined on the method body at 28:58...
   |
   |
LL |     fn subscribe(&mut self, t : Box<dyn Subscriber<Input=<Self as Publisher>::Output> + 'a>) {
   |                                                          ^^^^^^^^^^^^^^^^^^^^^^^^^^^
note: ...but the lifetime must also be valid for the lifetime `'a` as defined on the impl at 26:6...
   |
   |
LL | impl<'a> Publisher<'a> for MyStruct<'a> {
   |      ^^
note: ...so that the types are compatible
   |
   |
LL |     fn subscribe(&mut self, t : Box<dyn Subscriber<Input=<Self as Publisher>::Output> + 'a>) {
   |                                 ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
   = note: expected `Publisher<'_>`
              found `Publisher<'_>`
error: aborting due to previous error

For more information about this error, try `rustc --explain E0495`.


------------------------------------------


---- [ui] ui/issues/issue-27942.rs stdout ----
diff of stderr:

6    |
7    = note: expected type `Resources<'_>`
8               found type `Resources<'a>`
- note: the anonymous lifetime #1 defined on the method body at 5:5...
-   --> $DIR/issue-27942.rs:5:5
+ note: the anonymous lifetime defined on the method body at 5:15...
11    |
11    |
12 LL |     fn select(&self) -> BufferViewHandle<R>;
+    |               ^^^^^
+    |               ^^^^^
14 note: ...does not necessarily outlive the lifetime `'a` as defined on the trait at 3:18
16    |

30    |
30    |
31 LL | pub trait Buffer<'a, R: Resources<'a>> {
32    |                  ^^
- note: ...does not necessarily outlive the anonymous lifetime #1 defined on the method body at 5:5
-   --> $DIR/issue-27942.rs:5:5
+ note: ...does not necessarily outlive the anonymous lifetime defined on the method body at 5:15
35    |
35    |
36 LL |     fn select(&self) -> BufferViewHandle<R>;
+    |               ^^^^^
38 
39 error: aborting due to 2 previous errors
40 
40 


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-27942/issue-27942.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args issues/issue-27942.rs`
error: 1 errors occurred comparing output.
status: exit code: 1
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues/issue-27942.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-27942" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-27942/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0308]: mismatched types
  --> /checkout/src/test/ui/issues/issue-27942.rs:5:25
   |
LL |     fn select(&self) -> BufferViewHandle<R>;
   |                         ^^^^^^^^^^^^^^^^^^^ lifetime mismatch
   |
   = note: expected type `Resources<'_>`
              found type `Resources<'a>`
note: the anonymous lifetime defined on the method body at 5:15...
   |
   |
LL |     fn select(&self) -> BufferViewHandle<R>;
   |               ^^^^^
note: ...does not necessarily outlive the lifetime `'a` as defined on the trait at 3:18
   |
   |
LL | pub trait Buffer<'a, R: Resources<'a>> {

error[E0308]: mismatched types
  --> /checkout/src/test/ui/issues/issue-27942.rs:5:25
   |
   |
LL |     fn select(&self) -> BufferViewHandle<R>;
   |                         ^^^^^^^^^^^^^^^^^^^ lifetime mismatch
   |
   = note: expected type `Resources<'_>`
              found type `Resources<'a>`
note: the lifetime `'a` as defined on the trait at 3:18...
   |
   |
LL | pub trait Buffer<'a, R: Resources<'a>> {
   |                  ^^
note: ...does not necessarily outlive the anonymous lifetime defined on the method body at 5:15
   |
   |
LL |     fn select(&self) -> BufferViewHandle<R>;

error: aborting due to 2 previous errors

For more information about this error, try `rustc --explain E0308`.
---
---- [ui] ui/nll/issue-52742.rs stdout ----
diff of stderr:

9    |
10 LL | impl Foo<'_, '_> {
11    |          ^^
- note: ...but the borrowed content is only valid for the anonymous lifetime #2 defined on the method body at 13:5
-   --> $DIR/issue-52742.rs:13:5
+ note: ...but the borrowed content is only valid for the anonymous lifetime defined on the method body at 13:31
14    |
14    |
15 LL |     fn take_bar(&mut self, b: Bar<'_>) {
+    |                               ^^^^^^^
17 
18 error: aborting due to previous error
19 
19 


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/issue-52742/issue-52742.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args nll/issue-52742.rs`
error: 1 errors occurred comparing output.
status: exit code: 1
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/nll/issue-52742.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/issue-52742" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/issue-52742/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0312]: lifetime of reference outlives lifetime of borrowed content...
   |
   |
LL |         self.y = b.z
   |
   |
note: ...the reference is valid for the lifetime `'_` as defined on the impl at 12:10...
   |
   |
LL | impl Foo<'_, '_> {
   |          ^^
note: ...but the borrowed content is only valid for the anonymous lifetime defined on the method body at 13:31
   |
   |
LL |     fn take_bar(&mut self, b: Bar<'_>) {

error: aborting due to previous error

For more information about this error, try `rustc --explain E0312`.
---

4 LL |         Foo { bar }
5    |         ^^^
6    |
- note: first, the lifetime cannot outlive the anonymous lifetime #1 defined on the method body at 8:5...
-   --> $DIR/issue-55394.rs:8:5
+ note: first, the lifetime cannot outlive the anonymous lifetime defined on the method body at 8:17...
9    |
9    |
10 LL |     fn new(bar: &mut Bar) -> Self {
+    |                 ^^^^^^^^
+    |                 ^^^^^^^^
12 note: ...so that reference does not outlive borrowed content
14    |


The actual stderr differed from the expected stderr.
The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/issue-55394/issue-55394.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args nll/issue-55394.rs`
error: 1 errors occurred comparing output.
status: exit code: 1
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/nll/issue-55394.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/issue-55394" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/issue-55394/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0495]: cannot infer an appropriate lifetime for lifetime parameter `'s` due to conflicting requirements
   |
   |
LL |         Foo { bar } //~ERROR
   |
   |
note: first, the lifetime cannot outlive the anonymous lifetime defined on the method body at 8:17...
   |
   |
LL |     fn new(bar: &mut Bar) -> Self {
   |                 ^^^^^^^^
note: ...so that reference does not outlive borrowed content
   |
   |
LL |         Foo { bar } //~ERROR
   |               ^^^
note: but, the lifetime must be valid for the lifetime `'_` as defined on the impl at 7:10...
   |
   |
LL | impl Foo<'_> {
   |          ^^
note: ...so that the expression is assignable
   |
   |
LL |         Foo { bar } //~ERROR
   = note: expected `Foo<'_>`
              found `Foo<'_>`

error: aborting due to previous error
---

---- [ui] ui/nll/type-alias-free-regions.rs stdout ----
diff of stderr:

4 LL |         C { f: b }
6    |
6    |
- note: first, the lifetime cannot outlive the anonymous lifetime #1 defined on the method body at 16:5...
-   --> $DIR/type-alias-free-regions.rs:16:5
+ note: first, the lifetime cannot outlive the anonymous lifetime defined on the method body at 16:24...
9    |
9    |
10 LL |     fn from_box(b: Box<B>) -> Self {
+    |                        ^
+    |                        ^
12 note: ...so that the expression is assignable
14    |


35 LL |         C { f: Box::new(b.0) }
37    |
37    |
- note: first, the lifetime cannot outlive the anonymous lifetime #1 defined on the method body at 26:5...
-   --> $DIR/type-alias-free-regions.rs:26:5
+ note: first, the lifetime cannot outlive the anonymous lifetime defined on the method body at 26:23...
40    |
40    |
41 LL |     fn from_tuple(b: (B,)) -> Self {
+    |                       ^
+    |                       ^
43 note: ...so that the expression is assignable
45    |


The actual stderr differed from the expected stderr.
The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/type-alias-free-regions/type-alias-free-regions.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args nll/type-alias-free-regions.rs`
error: 1 errors occurred comparing output.
status: exit code: 1
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/nll/type-alias-free-regions.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/type-alias-free-regions" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/type-alias-free-regions/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0495]: cannot infer an appropriate lifetime for lifetime parameter `'a` due to conflicting requirements
   |
   |
LL |         C { f: b } //~ ERROR
   |
   |
note: first, the lifetime cannot outlive the anonymous lifetime defined on the method body at 16:24...
   |
   |
LL |     fn from_box(b: Box<B>) -> Self {
   |                        ^
note: ...so that the expression is assignable
   |
   |
LL |         C { f: b } //~ ERROR
   |                ^
   = note: expected `Box<Box<&isize>>`
              found `Box<Box<&isize>>`
note: but, the lifetime must be valid for the lifetime `'a` as defined on the impl at 15:6...
   |
   |
LL | impl<'a> FromBox<'a> for C<'a> {
   |      ^^
note: ...so that the expression is assignable
   |
   |
LL |         C { f: b } //~ ERROR
   |         ^^^^^^^^^^
   = note: expected `C<'a>`
              found `C<'_>`

error[E0495]: cannot infer an appropriate lifetime due to conflicting requirements
   |
   |
LL |         C { f: Box::new(b.0) } //~ ERROR
   |
   |
note: first, the lifetime cannot outlive the anonymous lifetime defined on the method body at 26:23...
   |
   |
LL |     fn from_tuple(b: (B,)) -> Self {
   |                       ^
note: ...so that the expression is assignable
   |
   |
LL |         C { f: Box::new(b.0) } //~ ERROR
   |                         ^^^
   = note: expected `Box<&isize>`
              found `Box<&isize>`
note: but, the lifetime must be valid for the lifetime `'a` as defined on the impl at 25:6...
   |
   |
LL | impl<'a> FromTuple<'a> for C<'a> {
   |      ^^
note: ...so that the expression is assignable
   |
   |
LL |         C { f: Box::new(b.0) } //~ ERROR
   |         ^^^^^^^^^^^^^^^^^^^^^^
   = note: expected `C<'a>`
              found `C<'_>`
error: aborting due to 2 previous errors

For more information about this error, try `rustc --explain E0495`.


------------------------------------------


---- [ui] ui/regions/regions-infer-paramd-indirect.rs stdout ----
diff of stderr:

6    |
7    = note: expected struct `Box<Box<&'a isize>>`
8               found struct `Box<Box<&isize>>`
- note: the anonymous lifetime #2 defined on the method body at 21:5...
-   --> $DIR/regions-infer-paramd-indirect.rs:21:5
+ note: the anonymous lifetime defined on the method body at 21:36...
11    |
11    |
12 LL |     fn set_f_bad(&mut self, b: Box<B>) {
+    |                                    ^
+    |                                    ^
14 note: ...does not necessarily outlive the lifetime `'a` as defined on the impl at 16:6
16    |


The actual stderr differed from the expected stderr.
The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/regions/regions-infer-paramd-indirect/regions-infer-paramd-indirect.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args regions/regions-infer-paramd-indirect.rs`
error: 1 errors occurred comparing output.
status: exit code: 1
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/regions/regions-infer-paramd-indirect.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/regions/regions-infer-paramd-indirect" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/regions/regions-infer-paramd-indirect/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0308]: mismatched types
  --> /checkout/src/test/ui/regions/regions-infer-paramd-indirect.rs:22:18
   |
LL |         self.f = b;
   |                  ^ lifetime mismatch
   |
   = note: expected struct `Box<Box<&'a isize>>`
              found struct `Box<Box<&isize>>`
note: the anonymous lifetime defined on the method body at 21:36...
   |
   |
LL |     fn set_f_bad(&mut self, b: Box<B>) {
   |                                    ^
note: ...does not necessarily outlive the lifetime `'a` as defined on the impl at 16:6
   |
   |
LL | impl<'a> SetF<'a> for C<'a> {

error: aborting due to previous error

For more information about this error, try `rustc --explain E0308`.
---
---- [ui] ui/ufcs/ufcs-explicit-self-bad.rs stdout ----
diff of stderr:

33    |
34    = note: expected reference `&'a Bar<T>`
35               found reference `&Bar<T>`
- note: the anonymous lifetime #1 defined on the method body at 37:5...
-   --> $DIR/ufcs-explicit-self-bad.rs:37:5
+ note: the anonymous lifetime defined on the method body at 37:21...
+   --> $DIR/ufcs-explicit-self-bad.rs:37:21
38    |
39 LL |     fn dummy2(self: &Bar<T>) {}
+    |                     ^^^^^^^
+    |                     ^^^^^^^
41 note: ...does not necessarily outlive the lifetime `'a` as defined on the impl at 35:6
42   --> $DIR/ufcs-explicit-self-bad.rs:35:6

57    |
57    |
58 LL | impl<'a, T> SomeTrait for &'a Bar<T> {
59    |      ^^
- note: ...does not necessarily outlive the anonymous lifetime #1 defined on the method body at 37:5
-   --> $DIR/ufcs-explicit-self-bad.rs:37:5
+ note: ...does not necessarily outlive the anonymous lifetime defined on the method body at 37:21
+   --> $DIR/ufcs-explicit-self-bad.rs:37:21
62    |
63 LL |     fn dummy2(self: &Bar<T>) {}
+    |                     ^^^^^^^
65 
65 
66 error[E0308]: mismatched `self` parameter type
67   --> $DIR/ufcs-explicit-self-bad.rs:39:21
71    |
71    |
72    = note: expected reference `&'a Bar<T>`
73               found reference `&Bar<T>`
- note: the anonymous lifetime #2 defined on the method body at 39:5...
-   --> $DIR/ufcs-explicit-self-bad.rs:39:5
+ note: the anonymous lifetime defined on the method body at 39:22...
+   --> $DIR/ufcs-explicit-self-bad.rs:39:22
76    |
77 LL |     fn dummy3(self: &&Bar<T>) {}
+    |                      ^^^^^^^
+    |                      ^^^^^^^
79 note: ...does not necessarily outlive the lifetime `'a` as defined on the impl at 35:6
80   --> $DIR/ufcs-explicit-self-bad.rs:35:6

95    |
95    |
96 LL | impl<'a, T> SomeTrait for &'a Bar<T> {
97    |      ^^
- note: ...does not necessarily outlive the anonymous lifetime #2 defined on the method body at 39:5
-   --> $DIR/ufcs-explicit-self-bad.rs:39:5
+ note: ...does not necessarily outlive the anonymous lifetime defined on the method body at 39:22
+   --> $DIR/ufcs-explicit-self-bad.rs:39:22
100    |
101 LL |     fn dummy3(self: &&Bar<T>) {}
+    |                      ^^^^^^^
103 
104 error: aborting due to 7 previous errors
105 
105 


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/ufcs/ufcs-explicit-self-bad/ufcs-explicit-self-bad.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args ufcs/ufcs-explicit-self-bad.rs`
error: 1 errors occurred comparing output.
status: exit code: 1
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/ufcs/ufcs-explicit-self-bad.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/ufcs/ufcs-explicit-self-bad" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/ufcs/ufcs-explicit-self-bad/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0307]: invalid `self` parameter type: isize
   |
   |
LL |     fn foo(self: isize, x: isize) -> isize {
   |
   |
   = note: type of `self` must be `Self` or a type that dereferences to it
   = help: consider changing to `self`, `&self`, `&mut self`, `self: Box<Self>`, `self: Rc<Self>`, `self: Arc<Self>`, or `self: Pin<P>` (where P is one of the previous types except `Self`)

error[E0307]: invalid `self` parameter type: Bar<isize>
   |
   |
LL |     fn foo(self: Bar<isize>, x: isize) -> isize {
   |
   |
   = note: type of `self` must be `Self` or a type that dereferences to it
   = help: consider changing to `self`, `&self`, `&mut self`, `self: Box<Self>`, `self: Rc<Self>`, `self: Arc<Self>`, or `self: Pin<P>` (where P is one of the previous types except `Self`)

error[E0307]: invalid `self` parameter type: &Bar<usize>
   |
   |
LL |     fn bar(self: &Bar<usize>, x: isize) -> isize {
   |
   |
   = note: type of `self` must be `Self` or a type that dereferences to it
   = help: consider changing to `self`, `&self`, `&mut self`, `self: Box<Self>`, `self: Rc<Self>`, `self: Arc<Self>`, or `self: Pin<P>` (where P is one of the previous types except `Self`)

error[E0308]: mismatched `self` parameter type
   |
   |
LL |     fn dummy2(self: &Bar<T>) {} //~ ERROR mismatched `self` parameter type
   |                     ^^^^^^^ lifetime mismatch
   |
   = note: expected reference `&'a Bar<T>`
              found reference `&Bar<T>`
note: the anonymous lifetime defined on the method body at 37:21...
   |
   |
LL |     fn dummy2(self: &Bar<T>) {} //~ ERROR mismatched `self` parameter type
   |                     ^^^^^^^
note: ...does not necessarily outlive the lifetime `'a` as defined on the impl at 35:6
   |
   |
LL | impl<'a, T> SomeTrait for &'a Bar<T> {


error[E0308]: mismatched `self` parameter type
   |
   |
LL |     fn dummy2(self: &Bar<T>) {} //~ ERROR mismatched `self` parameter type
   |                     ^^^^^^^ lifetime mismatch
   |
   = note: expected reference `&'a Bar<T>`
              found reference `&Bar<T>`
note: the lifetime `'a` as defined on the impl at 35:6...
   |
   |
LL | impl<'a, T> SomeTrait for &'a Bar<T> {
   |      ^^
note: ...does not necessarily outlive the anonymous lifetime defined on the method body at 37:21
   |
   |
LL |     fn dummy2(self: &Bar<T>) {} //~ ERROR mismatched `self` parameter type


error[E0308]: mismatched `self` parameter type
   |
   |
LL |     fn dummy3(self: &&Bar<T>) {}
   |                     ^^^^^^^^ lifetime mismatch
   |
   = note: expected reference `&'a Bar<T>`
              found reference `&Bar<T>`
note: the anonymous lifetime defined on the method body at 39:22...
   |
   |
LL |     fn dummy3(self: &&Bar<T>) {}
   |                      ^^^^^^^
note: ...does not necessarily outlive the lifetime `'a` as defined on the impl at 35:6
   |
   |
LL | impl<'a, T> SomeTrait for &'a Bar<T> {


error[E0308]: mismatched `self` parameter type
   |
   |
LL |     fn dummy3(self: &&Bar<T>) {}
   |                     ^^^^^^^^ lifetime mismatch
   |
   = note: expected reference `&'a Bar<T>`
              found reference `&Bar<T>`
note: the lifetime `'a` as defined on the impl at 35:6...
   |
   |
LL | impl<'a, T> SomeTrait for &'a Bar<T> {
   |      ^^
note: ...does not necessarily outlive the anonymous lifetime defined on the method body at 39:22
   |
   |
LL |     fn dummy3(self: &&Bar<T>) {}

error: aborting due to 7 previous errors

Some errors have detailed explanations: E0307, E0308.
---
test result: FAILED. 11368 passed; 11 failed; 93 ignored; 0 measured; 0 filtered out; finished in 133.86s



command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--suite" "ui" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-9/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python3" "--lldb-python" "/usr/bin/python3" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "9.0.0" "--llvm-components" "aarch64 aarch64asmparser aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets amdgpu amdgpuasmparser amdgpucodegen amdgpudesc amdgpudisassembler amdgpuinfo amdgpuutils analysis arm armasmparser armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter avr avrasmparser avrcodegen avrdesc avrdisassembler avrinfo binaryformat bitreader bitstreamreader bitwriter bpf bpfasmparser bpfcodegen bpfdesc bpfdisassembler bpfinfo codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfogsym debuginfomsf debuginfopdb demangle dlltooldriver engine executionengine fuzzmutate globalisel hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interpreter ipo irreader jitlink lanai lanaiasmparser lanaicodegen lanaidesc lanaidisassembler lanaiinfo libdriver lineeditor linker lto mc mca mcdisassembler mcjit mcparser mips mipsasmparser mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmparser msp430codegen msp430desc msp430disassembler msp430info native nativecodegen nvptx nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcjit passes perfjitevents powerpc powerpcasmparser powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata remarks riscv riscvasmparser riscvcodegen riscvdesc riscvdisassembler riscvinfo riscvutils runtimedyld scalaropts selectiondag sparc sparcasmparser sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target textapi transformutils vectorize webassembly webassemblyasmparser webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo windowsmanifest x86 x86asmparser x86codegen x86desc x86disassembler x86info x86utils xcore xcorecodegen xcoredesc xcoredisassembler xcoreinfo xray" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"


failed to run: /checkout/obj/build/bootstrap/debug/bootstrap --stage 2 test --exclude src/tools/tidy
Build completed unsuccessfully in 0:13:28
