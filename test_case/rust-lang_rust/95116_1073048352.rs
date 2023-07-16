plain
Check compiletest suite=ui mode=ui (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 12749 tests
.................................................................................................... 100/12749
............................................iiiiiiiiiiii..............Fii.......F.....F...i...i..... 200/12749
.................................................................................................... 400/12749
.................................................................................................... 500/12749
.................................................................................................... 600/12749
.................................................................................................... 700/12749
---
............................................iii..................................................... 12700/12749
.................................................
failures:

---- [ui] ui/asm/inline-syntax.rs#x86_64 stdout ----


1 warning: avoid using `.intel_syntax`, Intel syntax is the default
-   --> $DIR/inline-syntax.rs:57:14
3    |
3    |
4 LL | global_asm!(".intel_syntax noprefix", "nop");

7    = note: `#[warn(bad_asm_style)]` on by default
8 
8 
9 warning: avoid using `.intel_syntax`, Intel syntax is the default
-   --> $DIR/inline-syntax.rs:31:15
11    |
11    |
12 LL |         asm!(".intel_syntax noprefix", "nop");
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu

14 
14 
15 warning: avoid using `.intel_syntax`, Intel syntax is the default
-   --> $DIR/inline-syntax.rs:34:15
17    |
17    |
18 LL |         asm!(".intel_syntax aaa noprefix", "nop");

20 
20 
21 warning: avoid using `.att_syntax`, prefer using `options(att_syntax)` instead
-   --> $DIR/inline-syntax.rs:37:15
23    |
23    |
24 LL |         asm!(".att_syntax noprefix", "nop");

26 
26 
27 warning: avoid using `.att_syntax`, prefer using `options(att_syntax)` instead
-   --> $DIR/inline-syntax.rs:40:15
29    |
29    |
30 LL |         asm!(".att_syntax bbb noprefix", "nop");

32 
32 
33 warning: avoid using `.intel_syntax`, Intel syntax is the default
-   --> $DIR/inline-syntax.rs:43:15
35    |
35    |
36 LL |         asm!(".intel_syntax noprefix; nop");

38 
38 
39 warning: avoid using `.intel_syntax`, Intel syntax is the default
-   --> $DIR/inline-syntax.rs:49:13
41    |
41    |
42 LL |             .intel_syntax noprefix


The actual stderr differed from the expected stderr.
The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/asm/inline-syntax.x86_64/inline-syntax.x86_64.stderr
To only update this specific test, also pass `--test-args asm/inline-syntax.rs`


error in revision `x86_64`: 1 errors occurred comparing output.
status: exit status: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/asm/inline-syntax.rs" "-Zthreads=1" "--cfg" "x86_64" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/asm/inline-syntax.x86_64" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target" "x86_64-unknown-linux-gnu" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/asm/inline-syntax.x86_64/auxiliary"
stdout: none
--- stderr -------------------------------
warning: avoid using `.intel_syntax`, Intel syntax is the default
   |
   |
LL | global_asm!(".intel_syntax noprefix", "nop");
   |
   = note: `#[warn(bad_asm_style)]` on by default


warning: avoid using `.intel_syntax`, Intel syntax is the default
   |
   |
LL |         asm!(".intel_syntax noprefix", "nop");


warning: avoid using `.intel_syntax`, Intel syntax is the default
   |
   |
LL |         asm!(".intel_syntax aaa noprefix", "nop");


warning: avoid using `.att_syntax`, prefer using `options(att_syntax)` instead
   |
   |
LL |         asm!(".att_syntax noprefix", "nop");


warning: avoid using `.att_syntax`, prefer using `options(att_syntax)` instead
   |
   |
LL |         asm!(".att_syntax bbb noprefix", "nop");


warning: avoid using `.intel_syntax`, Intel syntax is the default
   |
   |
LL |         asm!(".intel_syntax noprefix; nop");


warning: avoid using `.intel_syntax`, Intel syntax is the default
   |
   |
LL |             .intel_syntax noprefix

warning: 7 warnings emitted
------------------------------------------

---
15 error: unknown directive
-   --> $DIR/inline-syntax.rs:31:15
+   --> $DIR/inline-syntax.rs:32:15
17    |
18 LL |         asm!(".intel_syntax noprefix", "nop");

25    |     ^
26 
27 error: unknown directive
27 error: unknown directive
-   --> $DIR/inline-syntax.rs:34:15
+   --> $DIR/inline-syntax.rs:35:15
29    |
30 LL |         asm!(".intel_syntax aaa noprefix", "nop");

37    |     ^
38 
39 error: unknown directive
39 error: unknown directive
-   --> $DIR/inline-syntax.rs:37:15
+   --> $DIR/inline-syntax.rs:38:15
41    |
42 LL |         asm!(".att_syntax noprefix", "nop");

49    |     ^
50 
51 error: unknown directive
51 error: unknown directive
-   --> $DIR/inline-syntax.rs:40:15
+   --> $DIR/inline-syntax.rs:41:15
53    |
54 LL |         asm!(".att_syntax bbb noprefix", "nop");

61    |     ^
62 
63 error: unknown directive
63 error: unknown directive
-   --> $DIR/inline-syntax.rs:43:15
+   --> $DIR/inline-syntax.rs:44:15
65    |
66 LL |         asm!(".intel_syntax noprefix; nop");

73    |     ^
74 
75 error: unknown directive
75 error: unknown directive
-   --> $DIR/inline-syntax.rs:49:13
+   --> $DIR/inline-syntax.rs:50:13
77    |
78 LL |             .intel_syntax noprefix


The actual stderr differed from the expected stderr.
The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/asm/inline-syntax.arm/inline-syntax.arm.stderr
To only update this specific test, also pass `--test-args asm/inline-syntax.rs`


error in revision `arm`: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/asm/inline-syntax.rs" "-Zthreads=1" "--cfg" "arm" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/asm/inline-syntax.arm" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target" "armv7-unknown-linux-gnueabihf" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/asm/inline-syntax.arm/auxiliary"
stdout: none
--- stderr -------------------------------
error: unknown directive
.intel_syntax noprefix
error: unknown directive
error: unknown directive
.intel_syntax noprefix
error: unknown directive
   |
note: instantiated into assembly here
note: instantiated into assembly here
  --> <inline asm>:1:1
   |
LL | .intel_syntax noprefix

error: unknown directive
  --> /checkout/src/test/ui/asm/inline-syntax.rs:32:15
   |
   |
LL |         asm!(".intel_syntax noprefix", "nop");
   |
note: instantiated into assembly here
note: instantiated into assembly here
  --> <inline asm>:1:2
   |
LL |     .intel_syntax noprefix

error: unknown directive
  --> /checkout/src/test/ui/asm/inline-syntax.rs:35:15
   |
   |
LL |         asm!(".intel_syntax aaa noprefix", "nop");
   |
note: instantiated into assembly here
note: instantiated into assembly here
  --> <inline asm>:1:2
   |
LL |     .intel_syntax aaa noprefix

error: unknown directive
  --> /checkout/src/test/ui/asm/inline-syntax.rs:38:15
   |
   |
LL |         asm!(".att_syntax noprefix", "nop");
   |
note: instantiated into assembly here
note: instantiated into assembly here
  --> <inline asm>:1:2
   |
LL |     .att_syntax noprefix

error: unknown directive
  --> /checkout/src/test/ui/asm/inline-syntax.rs:41:15
   |
   |
LL |         asm!(".att_syntax bbb noprefix", "nop");
   |
note: instantiated into assembly here
note: instantiated into assembly here
  --> <inline asm>:1:2
   |
LL |     .att_syntax bbb noprefix

error: unknown directive
  --> /checkout/src/test/ui/asm/inline-syntax.rs:44:15
   |
   |
LL |         asm!(".intel_syntax noprefix; nop");
   |
note: instantiated into assembly here
note: instantiated into assembly here
  --> <inline asm>:1:2
   |
LL |     .intel_syntax noprefix; nop

error: unknown directive
  --> /checkout/src/test/ui/asm/inline-syntax.rs:50:13
   |
   |
LL |             .intel_syntax noprefix
   |
note: instantiated into assembly here
note: instantiated into assembly here
  --> <inline asm>:2:13
   |
LL |             .intel_syntax noprefix

error: aborting due to 7 previous errors
------------------------------------------



---- [ui] ui/asm/naked-functions-unused.rs#x86_64 stdout ----

1 error: unused variable: `a`
-   --> $DIR/naked-functions-unused.rs:16:32
+   --> $DIR/naked-functions-unused.rs:17:32
+   --> $DIR/naked-functions-unused.rs:17:32
3    |
4 LL |     pub extern "C" fn function(a: usize, b: usize) -> usize {
5    |                                ^ help: if this is intentional, prefix it with an underscore: `_a`
6    |
7 note: the lint level is defined here
-   --> $DIR/naked-functions-unused.rs:4:9
+   --> $DIR/naked-functions-unused.rs:5:9
+   --> $DIR/naked-functions-unused.rs:5:9
9    |
10 LL | #![deny(unused)]
11    |         ^^^^^^

12    = note: `#[deny(unused_variables)]` implied by `#[deny(unused)]`
14 error: unused variable: `b`
-   --> $DIR/naked-functions-unused.rs:16:42
+   --> $DIR/naked-functions-unused.rs:17:42
16    |
16    |
17 LL |     pub extern "C" fn function(a: usize, b: usize) -> usize {
18    |                                          ^ help: if this is intentional, prefix it with an underscore: `_b`
19 
20 error: unused variable: `a`
-   --> $DIR/naked-functions-unused.rs:25:38
+   --> $DIR/naked-functions-unused.rs:26:38
+   --> $DIR/naked-functions-unused.rs:26:38
22    |
23 LL |         pub extern "C" fn associated(a: usize, b: usize) -> usize {
24    |                                      ^ help: if this is intentional, prefix it with an underscore: `_a`
25 
26 error: unused variable: `b`
-   --> $DIR/naked-functions-unused.rs:25:48
+   --> $DIR/naked-functions-unused.rs:26:48
+   --> $DIR/naked-functions-unused.rs:26:48
28    |
29 LL |         pub extern "C" fn associated(a: usize, b: usize) -> usize {
30    |                                                ^ help: if this is intentional, prefix it with an underscore: `_b`
31 
32 error: unused variable: `a`
-   --> $DIR/naked-functions-unused.rs:31:41
+   --> $DIR/naked-functions-unused.rs:32:41
+   --> $DIR/naked-functions-unused.rs:32:41
34    |
35 LL |         pub extern "C" fn method(&self, a: usize, b: usize) -> usize {
36    |                                         ^ help: if this is intentional, prefix it with an underscore: `_a`
37 
38 error: unused variable: `b`
-   --> $DIR/naked-functions-unused.rs:31:51
+   --> $DIR/naked-functions-unused.rs:32:51
+   --> $DIR/naked-functions-unused.rs:32:51
40    |
41 LL |         pub extern "C" fn method(&self, a: usize, b: usize) -> usize {
42    |                                                   ^ help: if this is intentional, prefix it with an underscore: `_b`
43 
44 error: unused variable: `a`
-   --> $DIR/naked-functions-unused.rs:39:40
+   --> $DIR/naked-functions-unused.rs:40:40
+   --> $DIR/naked-functions-unused.rs:40:40
46    |
47 LL |         extern "C" fn trait_associated(a: usize, b: usize) -> usize {
48    |                                        ^ help: if this is intentional, prefix it with an underscore: `_a`
49 
50 error: unused variable: `b`
-   --> $DIR/naked-functions-unused.rs:39:50
+   --> $DIR/naked-functions-unused.rs:40:50
+   --> $DIR/naked-functions-unused.rs:40:50
52    |
53 LL |         extern "C" fn trait_associated(a: usize, b: usize) -> usize {
54    |                                                  ^ help: if this is intentional, prefix it with an underscore: `_b`
55 
56 error: unused variable: `a`
-   --> $DIR/naked-functions-unused.rs:45:43
+   --> $DIR/naked-functions-unused.rs:46:43
+   --> $DIR/naked-functions-unused.rs:46:43
58    |
59 LL |         extern "C" fn trait_method(&self, a: usize, b: usize) -> usize {
60    |                                           ^ help: if this is intentional, prefix it with an underscore: `_a`
61 
62 error: unused variable: `b`
-   --> $DIR/naked-functions-unused.rs:45:53
+   --> $DIR/naked-functions-unused.rs:46:53
+   --> $DIR/naked-functions-unused.rs:46:53
64    |
65 LL |         extern "C" fn trait_method(&self, a: usize, b: usize) -> usize {
66    |                                                     ^ help: if this is intentional, prefix it with an underscore: `_b`

The actual stderr differed from the expected stderr.
The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/asm/naked-functions-unused.x86_64/naked-functions-unused.x86_64.stderr
To only update this specific test, also pass `--test-args asm/naked-functions-unused.rs`


error in revision `x86_64`: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/asm/naked-functions-unused.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "x86_64" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/asm/naked-functions-unused.x86_64" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/asm/naked-functions-unused.x86_64/auxiliary"
stdout: none
--- stderr -------------------------------
error: unused variable: `a`
   |
   |
LL |     pub extern "C" fn function(a: usize, b: usize) -> usize {
   |                                ^ help: if this is intentional, prefix it with an underscore: `_a`
note: the lint level is defined here
  --> /checkout/src/test/ui/asm/naked-functions-unused.rs:5:9
   |
LL | #![deny(unused)]
LL | #![deny(unused)]
   |         ^^^^^^
   = note: `#[deny(unused_variables)]` implied by `#[deny(unused)]`
error: unused variable: `b`
  --> /checkout/src/test/ui/asm/naked-functions-unused.rs:17:42
   |
   |
LL |     pub extern "C" fn function(a: usize, b: usize) -> usize {
   |                                          ^ help: if this is intentional, prefix it with an underscore: `_b`
error: unused variable: `a`
  --> /checkout/src/test/ui/asm/naked-functions-unused.rs:26:38
   |
   |
LL |         pub extern "C" fn associated(a: usize, b: usize) -> usize {
   |                                      ^ help: if this is intentional, prefix it with an underscore: `_a`
error: unused variable: `b`
  --> /checkout/src/test/ui/asm/naked-functions-unused.rs:26:48
   |
   |
LL |         pub extern "C" fn associated(a: usize, b: usize) -> usize {
   |                                                ^ help: if this is intentional, prefix it with an underscore: `_b`
error: unused variable: `a`
  --> /checkout/src/test/ui/asm/naked-functions-unused.rs:32:41
   |
   |
LL |         pub extern "C" fn method(&self, a: usize, b: usize) -> usize {
   |                                         ^ help: if this is intentional, prefix it with an underscore: `_a`
error: unused variable: `b`
  --> /checkout/src/test/ui/asm/naked-functions-unused.rs:32:51
   |
   |
LL |         pub extern "C" fn method(&self, a: usize, b: usize) -> usize {
   |                                                   ^ help: if this is intentional, prefix it with an underscore: `_b`
error: unused variable: `a`
  --> /checkout/src/test/ui/asm/naked-functions-unused.rs:40:40
   |
   |
LL |         extern "C" fn trait_associated(a: usize, b: usize) -> usize {
   |                                        ^ help: if this is intentional, prefix it with an underscore: `_a`
error: unused variable: `b`
  --> /checkout/src/test/ui/asm/naked-functions-unused.rs:40:50
   |
   |
LL |         extern "C" fn trait_associated(a: usize, b: usize) -> usize {
   |                                                  ^ help: if this is intentional, prefix it with an underscore: `_b`
error: unused variable: `a`
  --> /checkout/src/test/ui/asm/naked-functions-unused.rs:46:43
   |
   |
LL |         extern "C" fn trait_method(&self, a: usize, b: usize) -> usize {
   |                                           ^ help: if this is intentional, prefix it with an underscore: `_a`
error: unused variable: `b`
  --> /checkout/src/test/ui/asm/naked-functions-unused.rs:46:53
   |
   |
LL |         extern "C" fn trait_method(&self, a: usize, b: usize) -> usize {
   |                                                     ^ help: if this is intentional, prefix it with an underscore: `_b`
error: aborting due to 10 previous errors
------------------------------------------


