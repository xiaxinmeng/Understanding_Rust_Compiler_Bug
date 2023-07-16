plain
.....................ii............................................................................. 8100/12197
.................................................................................................... 8200/12197
..........................i.................................i....................................... 8300/12197
.....................i.............................................................................. 8400/12197
.........F..................................................i.......................F...........F... 8500/12197
..F.........F....................................................................................... 8600/12197
..............F.F....................i............................F................................. 8700/12197
.....................................................................................F.............. 8800/12197
.............F...................................................................................... 8900/12197
.................................................................................................... 9100/12197
....................................iiii.iiiii...................................................... 9200/12197
...........ii...............i....................................................................... 9300/12197
.................................................................................................... 9400/12197
.................................................................................................... 9400/12197
.................................................................................................... 9500/12197
.................................................................................................... 9600/12197
...............................................................................................F..F. 9700/12197
..................................................................................ii.i.............. 9900/12197
.................................................................................................... 10000/12197
....................................................................iiiiii.i..iiiiii.i.............. 10100/12197
.................................................................................................... 10200/12197
---

Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/parser/issue-10636-1.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/issue-10636-1" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/issue-10636-1/auxiliary"
------------------------------------------

------------------------------------------
stderr:
---
19 error: mismatched closing delimiter: `)`
-   --> $DIR/issue-60075.rs:6:10
+   --> $DIR/issue-60075.rs:4:31
21    |
22 LL |     fn qux() -> Option<usize> {
+    |                               ^ unclosed delimiter
24 LL |         let _ = if true {
25 LL |         });
26    |          ^ mismatched closing delimiter
---
To only update this specific test, also pass `--test-args parser/issue-60075.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/parser/issue-60075.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/issue-60075" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/issue-60075/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: expected one of `.`, `;`, `?`, `else`, or an operator, found `}`
   |
LL |         });
LL |         });
   |          ^ expected one of `.`, `;`, `?`, `else`, or an operator
error: non-item in item list
  --> /checkout/src/test/ui/parser/issue-60075.rs:6:11
   |
LL | trait T {
LL | trait T {
   |         - item list starts here
...
LL |         });
   |           ^ non-item starts here
LL |     }
   |     - item list ends here

error: mismatched closing delimiter: `)`
error: mismatched closing delimiter: `)`
  --> /checkout/src/test/ui/parser/issue-60075.rs:4:31
   |
LL |     fn qux() -> Option<usize> {
   |                               ^ unclosed delimiter
LL |         let _ = if true {
LL |         });

error: aborting due to 3 previous errors


---
15 error: mismatched closing delimiter: `]`
-   --> $DIR/issue-63116.rs:3:16
+   --> $DIR/issue-63116.rs:3:14
17    |
18 LL | impl W <s(f;Y(;]
-    |              - ^ mismatched closing delimiter
+    |              ^ ^ mismatched closing delimiter
21    |              unclosed delimiter
22 



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/issue-63116/issue-63116.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args parser/issue-63116.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/parser/issue-63116.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/issue-63116" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/issue-63116/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: this file contains an unclosed delimiter
  --> /checkout/src/test/ui/parser/issue-63116.rs:3:18
   |
LL | impl W <s(f;Y(;]
   |          -       ^
   |          unclosed delimiter


error: expected one of `!`, `(`, `)`, `+`, `,`, `::`, or `<`, found `;`
   |
   |
LL | impl W <s(f;Y(;]
   |            ^ expected one of 7 possible tokens
error: mismatched closing delimiter: `]`
  --> /checkout/src/test/ui/parser/issue-63116.rs:3:14
   |
   |
LL | impl W <s(f;Y(;]
   |              ^ ^ mismatched closing delimiter
   |              unclosed delimiter

error: aborting due to 3 previous errors



------------------------------------------


---- [ui] ui/parser/issue-62973.rs stdout ----
diff of stderr:

56    |  ^ expected one of `.`, `?`, `{`, or an operator
58 error: mismatched closing delimiter: `)`
-   --> $DIR/issue-62973.rs:6:28
+   --> $DIR/issue-62973.rs:6:27
60    |
60    |
61 LL | fn p() { match s { v, E { [) {) }
-    |                           -^ mismatched closing delimiter
+    |                           ^^ mismatched closing delimiter
64    |                           unclosed delimiter
65 

66 error: mismatched closing delimiter: `)`
66 error: mismatched closing delimiter: `)`
-   --> $DIR/issue-62973.rs:6:31
+   --> $DIR/issue-62973.rs:6:30
68    |
69 LL | fn p() { match s { v, E { [) {) }
-    |                              -^ mismatched closing delimiter
+    |                              ^^ mismatched closing delimiter
72    |                              unclosed delimiter
73 



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/issue-62973/issue-62973.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args parser/issue-62973.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/parser/issue-62973.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/issue-62973" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/issue-62973/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: this file contains an unclosed delimiter
  --> /checkout/src/test/ui/parser/issue-62973.rs:8:2
   |
LL | fn p() { match s { v, E { [) {) }
   |        -         - unclosed delimiter
   |        unclosed delimiter
LL | 
LL | 
   |  ^
   |  ^

error: this file contains an unclosed delimiter
  --> /checkout/src/test/ui/parser/issue-62973.rs:8:2
   |
LL | fn p() { match s { v, E { [) {) }
   |        -         - unclosed delimiter
   |        unclosed delimiter
LL | 
LL | 
   |  ^
   |  ^

error: expected one of `,` or `}`, found `{`
   |
   |
LL | fn p() { match s { v, E { [) {) }
   |        ^       -       -^ expected one of `,` or `}`
   |        |       |       |
   |        |       |       help: `}` may belong here
   |        unclosed delimiter

error: struct literals are not allowed here
  --> /checkout/src/test/ui/parser/issue-62973.rs:6:16
  --> /checkout/src/test/ui/parser/issue-62973.rs:6:16
   |
LL |   fn p() { match s { v, E { [) {) }
LL | |
LL | |
   | |_^
   |
   |
help: surround the struct literal with parentheses
   |
LL ~ fn p() { match (s { v, E { [) {) }
LL ~ )
   |


error: expected one of `.`, `?`, `{`, or an operator, found `}`
   |
   |
LL | fn p() { match s { v, E { [) {) }
   |          ----- while parsing this match expression
LL | 
LL | 
   |  ^ expected one of `.`, `?`, `{`, or an operator
error: mismatched closing delimiter: `)`
  --> /checkout/src/test/ui/parser/issue-62973.rs:6:27
   |
   |
LL | fn p() { match s { v, E { [) {) }
   |                           ^^ mismatched closing delimiter
   |                           unclosed delimiter

error: mismatched closing delimiter: `)`
  --> /checkout/src/test/ui/parser/issue-62973.rs:6:30
  --> /checkout/src/test/ui/parser/issue-62973.rs:6:30
   |
LL | fn p() { match s { v, E { [) {) }
   |                              ^^ mismatched closing delimiter
   |                              unclosed delimiter

error: aborting due to 7 previous errors

---
1 error: mismatched closing delimiter: `]`
-   --> $DIR/issue-67377-invalid-syntax-in-enum-discriminant.rs:5:42
+   --> $DIR/issue-67377-invalid-syntax-in-enum-discriminant.rs:5:27
3    |
4 LL |         V = [PhantomData; { [ () ].len() ].len() as isize,
-    |             -             -              ^ mismatched closing delimiter
+    |             -             ^              ^ mismatched closing delimiter
7    |             |             unclosed delimiter
8    |             closing delimiter possibly meant for this

9 
9 
10 error: mismatched closing delimiter: `]`
-   --> $DIR/issue-67377-invalid-syntax-in-enum-discriminant.rs:15:36
+   --> $DIR/issue-67377-invalid-syntax-in-enum-discriminant.rs:15:24
12    |
13 LL |         V = [Vec::new; { [].len()  ].len() as isize,
-    |             -          -           ^ mismatched closing delimiter
+    |             -          ^           ^ mismatched closing delimiter
16    |             |          unclosed delimiter
17    |             closing delimiter possibly meant for this

18 
18 
19 error: mismatched closing delimiter: `]`
-   --> $DIR/issue-67377-invalid-syntax-in-enum-discriminant.rs:26:36
+   --> $DIR/issue-67377-invalid-syntax-in-enum-discriminant.rs:26:24
21    |
22 LL |         V = [Vec::new; { [0].len() ].len() as isize,
-    |             -          -           ^ mismatched closing delimiter
+    |             -          ^           ^ mismatched closing delimiter
25    |             |          unclosed delimiter
26    |             closing delimiter possibly meant for this

27 
27 
28 error: mismatched closing delimiter: `]`
-   --> $DIR/issue-67377-invalid-syntax-in-enum-discriminant.rs:5:42
+   --> $DIR/issue-67377-invalid-syntax-in-enum-discriminant.rs:5:27
30    |
31 LL |         V = [PhantomData; { [ () ].len() ].len() as isize,
-    |             -             -              ^ mismatched closing delimiter
+    |             -             ^              ^ mismatched closing delimiter
34    |             |             unclosed delimiter
35    |             closing delimiter possibly meant for this

36 
36 
37 error: mismatched closing delimiter: `]`
-   --> $DIR/issue-67377-invalid-syntax-in-enum-discriminant.rs:15:36
+   --> $DIR/issue-67377-invalid-syntax-in-enum-discriminant.rs:15:24
39    |
40 LL |         V = [Vec::new; { [].len()  ].len() as isize,
-    |             -          -           ^ mismatched closing delimiter
+    |             -          ^           ^ mismatched closing delimiter
43    |             |          unclosed delimiter
44    |             closing delimiter possibly meant for this

45 
45 
46 error: mismatched closing delimiter: `]`
-   --> $DIR/issue-67377-invalid-syntax-in-enum-discriminant.rs:26:36
+   --> $DIR/issue-67377-invalid-syntax-in-enum-discriminant.rs:26:24
48    |
49 LL |         V = [Vec::new; { [0].len() ].len() as isize,
-    |             -          -           ^ mismatched closing delimiter
+    |             -          ^           ^ mismatched closing delimiter
52    |             |          unclosed delimiter
53    |             closing delimiter possibly meant for this

54 
54 
55 error: mismatched closing delimiter: `]`
-   --> $DIR/issue-67377-invalid-syntax-in-enum-discriminant.rs:5:42
+   --> $DIR/issue-67377-invalid-syntax-in-enum-discriminant.rs:5:27
57    |
58 LL |         V = [PhantomData; { [ () ].len() ].len() as isize,
-    |             -             -              ^ mismatched closing delimiter
+    |             -             ^              ^ mismatched closing delimiter
61    |             |             unclosed delimiter
62    |             closing delimiter possibly meant for this

63 
63 
64 error: mismatched closing delimiter: `]`
-   --> $DIR/issue-67377-invalid-syntax-in-enum-discriminant.rs:15:36
+   --> $DIR/issue-67377-invalid-syntax-in-enum-discriminant.rs:15:24
66    |
67 LL |         V = [Vec::new; { [].len()  ].len() as isize,
-    |             -          -           ^ mismatched closing delimiter
+    |             -          ^           ^ mismatched closing delimiter
70    |             |          unclosed delimiter
71    |             closing delimiter possibly meant for this

72 
72 
73 error: mismatched closing delimiter: `]`
-   --> $DIR/issue-67377-invalid-syntax-in-enum-discriminant.rs:26:36
+   --> $DIR/issue-67377-invalid-syntax-in-enum-discriminant.rs:26:24
75    |
76 LL |         V = [Vec::new; { [0].len() ].len() as isize,
-    |             -          -           ^ mismatched closing delimiter
+    |             -          ^           ^ mismatched closing delimiter
79    |             |          unclosed delimiter
80    |             closing delimiter possibly meant for this

81 
81 
82 error: mismatched closing delimiter: `]`
-   --> $DIR/issue-67377-invalid-syntax-in-enum-discriminant.rs:5:42
+   --> $DIR/issue-67377-invalid-syntax-in-enum-discriminant.rs:5:27
84    |
85 LL |         V = [PhantomData; { [ () ].len() ].len() as isize,
-    |             -             -              ^ mismatched closing delimiter
+    |             -             ^              ^ mismatched closing delimiter
88    |             |             unclosed delimiter
89    |             closing delimiter possibly meant for this

90 
90 
91 error: mismatched closing delimiter: `]`
-   --> $DIR/issue-67377-invalid-syntax-in-enum-discriminant.rs:15:36
+   --> $DIR/issue-67377-invalid-syntax-in-enum-discriminant.rs:15:24
93    |
94 LL |         V = [Vec::new; { [].len()  ].len() as isize,
-    |             -          -           ^ mismatched closing delimiter
+    |             -          ^           ^ mismatched closing delimiter
97    |             |          unclosed delimiter
98    |             closing delimiter possibly meant for this

99 
99 
100 error: mismatched closing delimiter: `]`
-   --> $DIR/issue-67377-invalid-syntax-in-enum-discriminant.rs:26:36
+   --> $DIR/issue-67377-invalid-syntax-in-enum-discriminant.rs:26:24
102    |
103 LL |         V = [Vec::new; { [0].len() ].len() as isize,
-    |             -          -           ^ mismatched closing delimiter
+    |             -          ^           ^ mismatched closing delimiter
106    |             |          unclosed delimiter
107    |             closing delimiter possibly meant for this



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/issue-67377-invalid-syntax-in-enum-discriminant/issue-67377-invalid-syntax-in-enum-discriminant.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args parser/issue-67377-invalid-syntax-in-enum-discriminant.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/parser/issue-67377-invalid-syntax-in-enum-discriminant.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/issue-67377-invalid-syntax-in-enum-discriminant" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/issue-67377-invalid-syntax-in-enum-discriminant/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: mismatched closing delimiter: `]`
  --> /checkout/src/test/ui/parser/issue-67377-invalid-syntax-in-enum-discriminant.rs:5:27
   |
LL |         V = [PhantomData; { [ () ].len() ].len() as isize,
   |             -             ^              ^ mismatched closing delimiter
   |             |             unclosed delimiter
   |             closing delimiter possibly meant for this

error: mismatched closing delimiter: `]`
error: mismatched closing delimiter: `]`
  --> /checkout/src/test/ui/parser/issue-67377-invalid-syntax-in-enum-discriminant.rs:15:24
   |
LL |         V = [Vec::new; { [].len()  ].len() as isize,
   |             -          ^           ^ mismatched closing delimiter
   |             |          unclosed delimiter
   |             closing delimiter possibly meant for this

error: mismatched closing delimiter: `]`
error: mismatched closing delimiter: `]`
  --> /checkout/src/test/ui/parser/issue-67377-invalid-syntax-in-enum-discriminant.rs:26:24
   |
LL |         V = [Vec::new; { [0].len() ].len() as isize,
   |             -          ^           ^ mismatched closing delimiter
   |             |          unclosed delimiter
   |             closing delimiter possibly meant for this

error: mismatched closing delimiter: `]`
error: mismatched closing delimiter: `]`
  --> /checkout/src/test/ui/parser/issue-67377-invalid-syntax-in-enum-discriminant.rs:5:27
   |
LL |         V = [PhantomData; { [ () ].len() ].len() as isize,
   |             -             ^              ^ mismatched closing delimiter
   |             |             unclosed delimiter
   |             closing delimiter possibly meant for this

error: mismatched closing delimiter: `]`
error: mismatched closing delimiter: `]`
  --> /checkout/src/test/ui/parser/issue-67377-invalid-syntax-in-enum-discriminant.rs:15:24
   |
LL |         V = [Vec::new; { [].len()  ].len() as isize,
   |             -          ^           ^ mismatched closing delimiter
   |             |          unclosed delimiter
   |             closing delimiter possibly meant for this

error: mismatched closing delimiter: `]`
error: mismatched closing delimiter: `]`
  --> /checkout/src/test/ui/parser/issue-67377-invalid-syntax-in-enum-discriminant.rs:26:24
   |
LL |         V = [Vec::new; { [0].len() ].len() as isize,
   |             -          ^           ^ mismatched closing delimiter
   |             |          unclosed delimiter
   |             closing delimiter possibly meant for this

error: mismatched closing delimiter: `]`
error: mismatched closing delimiter: `]`
  --> /checkout/src/test/ui/parser/issue-67377-invalid-syntax-in-enum-discriminant.rs:5:27
   |
LL |         V = [PhantomData; { [ () ].len() ].len() as isize,
   |             -             ^              ^ mismatched closing delimiter
   |             |             unclosed delimiter
   |             closing delimiter possibly meant for this

error: mismatched closing delimiter: `]`
error: mismatched closing delimiter: `]`
  --> /checkout/src/test/ui/parser/issue-67377-invalid-syntax-in-enum-discriminant.rs:15:24
   |
LL |         V = [Vec::new; { [].len()  ].len() as isize,
   |             -          ^           ^ mismatched closing delimiter
   |             |          unclosed delimiter
   |             closing delimiter possibly meant for this

error: mismatched closing delimiter: `]`
error: mismatched closing delimiter: `]`
  --> /checkout/src/test/ui/parser/issue-67377-invalid-syntax-in-enum-discriminant.rs:26:24
   |
LL |         V = [Vec::new; { [0].len() ].len() as isize,
   |             -          ^           ^ mismatched closing delimiter
   |             |          unclosed delimiter
   |             closing delimiter possibly meant for this

error: mismatched closing delimiter: `]`
error: mismatched closing delimiter: `]`
  --> /checkout/src/test/ui/parser/issue-67377-invalid-syntax-in-enum-discriminant.rs:5:27
   |
LL |         V = [PhantomData; { [ () ].len() ].len() as isize,
   |             -             ^              ^ mismatched closing delimiter
   |             |             unclosed delimiter
   |             closing delimiter possibly meant for this

error: mismatched closing delimiter: `]`
error: mismatched closing delimiter: `]`
  --> /checkout/src/test/ui/parser/issue-67377-invalid-syntax-in-enum-discriminant.rs:15:24
   |
LL |         V = [Vec::new; { [].len()  ].len() as isize,
   |             -          ^           ^ mismatched closing delimiter
   |             |          unclosed delimiter
   |             closing delimiter possibly meant for this

error: mismatched closing delimiter: `]`
error: mismatched closing delimiter: `]`
  --> /checkout/src/test/ui/parser/issue-67377-invalid-syntax-in-enum-discriminant.rs:26:24
   |
LL |         V = [Vec::new; { [0].len() ].len() as isize,
   |             -          ^           ^ mismatched closing delimiter
   |             |          unclosed delimiter
   |             closing delimiter possibly meant for this

error[E0282]: type annotations needed
error[E0282]: type annotations needed
  --> /checkout/src/test/ui/parser/issue-67377-invalid-syntax-in-enum-discriminant.rs:15:29
   |
LL |         V = [Vec::new; { [].len()  ].len() as isize,
   |                             ^^^ cannot infer type for type parameter `T`
error[E0282]: type annotations needed
  --> /checkout/src/test/ui/parser/issue-67377-invalid-syntax-in-enum-discriminant.rs:26:14
   |
   |
LL |         V = [Vec::new; { [0].len() ].len() as isize,
   |              ^^^^^^^^ cannot infer type for type parameter `T`
error: aborting due to 14 previous errors

For more information about this error, try `rustc --explain E0282`.

---
+   --> $DIR/macro-mismatched-delim-paren-brace.rs:2:10
14    |
15 LL |     foo! (
-    |          - unclosed delimiter
+    |          ^ unclosed delimiter
17 LL |         bar, "baz", 1, 2.0
19    |     ^ mismatched closing delimiter


The actual stderr differed from the expected stderr.
The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/macro-mismatched-delim-paren-brace/macro-mismatched-delim-paren-brace.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args parser/macro-mismatched-delim-paren-brace.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/parser/macro-mismatched-delim-paren-brace.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/macro-mismatched-delim-paren-brace" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/macro-mismatched-delim-paren-brace/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: unexpected closing delimiter: `}`
  --> /checkout/src/test/ui/parser/macro-mismatched-delim-paren-brace.rs:5:1
   |
LL | fn main() {
   |           - this opening brace...
...
LL |     } //~ ERROR mismatched closing delimiter
   |     - ...matches this closing brace
LL | } //~ ERROR unexpected closing delimiter: `}`

error: mismatched closing delimiter: `}`
  --> /checkout/src/test/ui/parser/macro-mismatched-delim-paren-brace.rs:2:10
   |
   |
LL |     foo! (
   |          ^ unclosed delimiter
LL |         bar, "baz", 1, 2.0
LL |     } //~ ERROR mismatched closing delimiter

error: aborting due to 2 previous errors


---
+   --> $DIR/macro-mismatched-delim-brace-paren.rs:4:10
3    |
4 LL |     foo! {
-    |          - unclosed delimiter
+    |          ^ unclosed delimiter
6 LL |         bar, "baz", 1, 2.0
8    |     ^ mismatched closing delimiter


The actual stderr differed from the expected stderr.
The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/macro-mismatched-delim-brace-paren/macro-mismatched-delim-brace-paren.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args parser/macro-mismatched-delim-brace-paren.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/parser/macro-mismatched-delim-brace-paren.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/macro-mismatched-delim-brace-paren" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/macro-mismatched-delim-brace-paren/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: mismatched closing delimiter: `)`
  --> /checkout/src/test/ui/parser/macro-mismatched-delim-brace-paren.rs:4:10
   |
LL |     foo! {
   |          ^ unclosed delimiter
LL |         bar, "baz", 1, 2.0
LL |     ) //~ ERROR mismatched closing delimiter

error: aborting due to previous error


---
14    |     ^ mismatched closing delimiter


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/parser-recovery-2/parser-recovery-2.stderr
To only update this specific test, also pass `--test-args parser/parser-recovery-2.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/parser/parser-recovery-2.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/parser-recovery-2" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/parser-recovery-2/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: unexpected token: `;`
  --> /checkout/src/test/ui/parser/parser-recovery-2.rs:10:15
   |
LL |     let x = y.;  //~ ERROR unexpected token

error: mismatched closing delimiter: `)`
  --> /checkout/src/test/ui/parser/parser-recovery-2.rs:4:14
   |
   |
LL |     fn bar() {
   |              ^ unclosed delimiter
LL |         let x = foo(); //~ ERROR cannot find function `foo` in this scope
LL |     ) //~ ERROR mismatched closing delimiter: `)`

error[E0425]: cannot find function `foo` in this scope
  --> /checkout/src/test/ui/parser/parser-recovery-2.rs:5:17
   |
   |
LL |         let x = foo(); //~ ERROR cannot find function `foo` in this scope


error[E0425]: cannot find value `y` in this scope
   |
   |
LL |     let x = y.;  //~ ERROR unexpected token
   |             ^ not found in this scope
error: aborting due to 4 previous errors

For more information about this error, try `rustc --explain E0425`.

---
1 error: mismatched closing delimiter: `}`
-   --> $DIR/unclosed_delim_mod.rs:7:1
+   --> $DIR/unclosed_delim_mod.rs:5:7
3    |
4 LL | pub fn new() -> Result<Value, ()> {

6 LL |     Ok(Value {
-    |       - unclosed delimiter
+    |       ^ unclosed delimiter
---
To only update this specific test, also pass `--test-args parser/unclosed_delim_mod.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/parser/unclosed_delim_mod.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/unclosed_delim_mod" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/unclosed_delim_mod/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: mismatched closing delimiter: `}`
  --> /checkout/src/test/ui/parser/unclosed_delim_mod.rs:5:7
   |
LL | pub fn new() -> Result<Value, ()> {
LL |     Ok(Value {
   |       ^ unclosed delimiter
LL |     }
LL | }
---
1 error: mismatched closing delimiter: `}`
-   --> $DIR/unclosed_delim_mod.rs:7:1
+   --> $DIR/unclosed_delim_mod.rs:5:7
3    |
4 LL | pub fn new() -> Result<Value, ()> {

6 LL |     Ok(Value {
-    |       - unclosed delimiter
+    |       ^ unclosed delimiter
+    |       ^ unclosed delimiter
8 LL |     }
9 LL | }
10    | ^ mismatched closing delimiter


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/unclosed-delimiter-in-dep/unclosed-delimiter-in-dep.stderr
To only update this specific test, also pass `--test-args parser/unclosed-delimiter-in-dep.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/parser/unclosed-delimiter-in-dep.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/unclosed-delimiter-in-dep" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/unclosed-delimiter-in-dep/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: mismatched closing delimiter: `}`
  --> /checkout/src/test/ui/parser/unclosed_delim_mod.rs:5:7
   |
LL | pub fn new() -> Result<Value, ()> {
LL |     Ok(Value {
   |       ^ unclosed delimiter
LL |     }
LL | }
LL | }
   | ^ mismatched closing delimiter

error[E0308]: mismatched types
  --> /checkout/src/test/ui/parser/unclosed-delimiter-in-dep.rs:4:20
   |
LL |     let _: usize = unclosed_delim_mod::new();
   |            -----   ^^^^^^^^^^^^^^^^^^^^^^^^^ expected `usize`, found enum `Result`
   |            expected due to this
   |
   = note: expected type `usize`
   = note: expected type `usize`
              found enum `Result<Value, ()>`
error: aborting due to 2 previous errors

For more information about this error, try `rustc --explain E0308`.

---
8    |     ^ mismatched closing delimiter


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/resolve/token-error-correct-2/token-error-correct-2.stderr
To only update this specific test, also pass `--test-args resolve/token-error-correct-2.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/resolve/token-error-correct-2.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/resolve/token-error-correct-2" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/resolve/token-error-correct-2/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: mismatched closing delimiter: `)`
  --> /checkout/src/test/ui/resolve/token-error-correct-2.rs:4:12
   |
LL |     if foo {
   |            ^ unclosed delimiter
LL |     //~^ ERROR: cannot find value `foo`
LL |     ) //~ ERROR: mismatched closing delimiter: `)`

error[E0425]: cannot find value `foo` in this scope
  --> /checkout/src/test/ui/resolve/token-error-correct-2.rs:4:8
   |
---
3    |
4 LL | fn main() {
5    |           - closing delimiter possibly meant for this

6 LL |     foo(bar(;
+    |            ^ unclosed delimiter
8 LL |
9 LL | }
10    | ^ mismatched closing delimiter
---
To only update this specific test, also pass `--test-args resolve/token-error-correct.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/resolve/token-error-correct.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/resolve/token-error-correct" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/resolve/token-error-correct/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: mismatched closing delimiter: `}`
  --> /checkout/src/test/ui/resolve/token-error-correct.rs:4:12
   |
LL | fn main() {
   |           - closing delimiter possibly meant for this
LL |     foo(bar(;
   |            ^ unclosed delimiter
LL |     //~^ ERROR cannot find function `bar` in this scope
LL | }
   | ^ mismatched closing delimiter

error[E0425]: cannot find function `bar` in this scope
   |
   |
LL |     foo(bar(;

error: aborting due to 2 previous errors

For more information about this error, try `rustc --explain E0425`.
---
test result: FAILED. 12083 passed; 12 failed; 102 ignored; 0 measured; 0 filtered out; finished in 126.89s



command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--suite" "ui" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-10/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python3" "--lldb-python" "/usr/bin/python3" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "10.0.0" "--llvm-components" "aarch64 aarch64asmparser aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets amdgpu amdgpuasmparser amdgpucodegen amdgpudesc amdgpudisassembler amdgpuinfo amdgpuutils analysis arm armasmparser armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter avr avrasmparser avrcodegen avrdesc avrdisassembler avrinfo binaryformat bitreader bitstreamreader bitwriter bpf bpfasmparser bpfcodegen bpfdesc bpfdisassembler bpfinfo cfguard codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfogsym debuginfomsf debuginfopdb demangle dlltooldriver dwarflinker engine executionengine frontendopenmp fuzzmutate globalisel hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interpreter ipo irreader jitlink lanai lanaiasmparser lanaicodegen lanaidesc lanaidisassembler lanaiinfo libdriver lineeditor linker lto mc mca mcdisassembler mcjit mcparser mips mipsasmparser mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmparser msp430codegen msp430desc msp430disassembler msp430info native nativecodegen nvptx nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcerror orcjit passes perfjitevents powerpc powerpcasmparser powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata remarks riscv riscvasmparser riscvcodegen riscvdesc riscvdisassembler riscvinfo riscvutils runtimedyld scalaropts selectiondag sparc sparcasmparser sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target textapi transformutils vectorize webassembly webassemblyasmparser webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo windowsmanifest x86 x86asmparser x86codegen x86desc x86disassembler x86info x86utils xcore xcorecodegen xcoredesc xcoredisassembler xcoreinfo xray" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--channel" "nightly" "--color" "always"


Build completed unsuccessfully in 0:13:03
