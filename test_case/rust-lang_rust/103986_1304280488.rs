plain
.........................iii............................................................ 13728/13775
...............................................
failures:

---- [ui] src/test/ui/parser/label-after-block-like.rs stdout ----

1 error: labeled expression must be followed by `:`
1 error: labeled expression must be followed by `:`
-   --> $DIR/label-after-loop-kw.rs:2:20
+   --> $DIR/label-after-block-like.rs:2:20
3    |
4 LL |     if let () = () 'a {}
5    |                    ---^^

10    = note: labels are used before loops and blocks, allowing e.g., `break 'label` to them
12 error: expected `{`, found `'a`
12 error: expected `{`, found `'a`
-   --> $DIR/label-after-loop-kw.rs:2:20
+   --> $DIR/label-after-block-like.rs:2:20
14    |
15 LL |     if let () = () 'a {}

17    |
18 note: the `if` expression is missing a block after this condition
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
-   --> $DIR/label-after-loop-kw.rs:2:8
+   --> $DIR/label-after-block-like.rs:2:8
20    |
21 LL |     if let () = () 'a {}

26    |                    +       +
27 
28 error: labeled expression must be followed by `:`
28 error: labeled expression must be followed by `:`
-   --> $DIR/label-after-loop-kw.rs:8:13
+   --> $DIR/label-after-block-like.rs:8:13
30    |
31 LL |     if true 'a {}
32    |             ---^^

37    = note: labels are used before loops and blocks, allowing e.g., `break 'label` to them
39 error: expected `{`, found `'a`
39 error: expected `{`, found `'a`
-   --> $DIR/label-after-loop-kw.rs:8:13
+   --> $DIR/label-after-block-like.rs:8:13
41    |
42 LL |     if true 'a {}

44    |
45 note: the `if` expression is missing a block after this condition
45 note: the `if` expression is missing a block after this condition
-   --> $DIR/label-after-loop-kw.rs:8:8
+   --> $DIR/label-after-block-like.rs:8:8
47    |
48 LL |     if true 'a {}

53    |             +       +
54 
55 error: labeled expression must be followed by `:`
55 error: labeled expression must be followed by `:`
-   --> $DIR/label-after-loop-kw.rs:14:10
+   --> $DIR/label-after-block-like.rs:14:10
57    |
58 LL |     loop 'a {}
59    |          ---^^

64    = note: labels are used before loops and blocks, allowing e.g., `break 'label` to them
66 error: expected `{`, found `'a`
66 error: expected `{`, found `'a`
-   --> $DIR/label-after-loop-kw.rs:14:10
+   --> $DIR/label-after-block-like.rs:14:10
68    |
69 LL |     loop 'a {}
70    |     ---- ^^ expected `{`
77    |          +       +
78 
79 error: labeled expression must be followed by `:`
79 error: labeled expression must be followed by `:`
-   --> $DIR/label-after-loop-kw.rs:20:16
+   --> $DIR/label-after-block-like.rs:20:16
81    |
82 LL |     while true 'a {}
83    |                ---^^

88    = note: labels are used before loops and blocks, allowing e.g., `break 'label` to them
90 error: expected `{`, found `'a`
90 error: expected `{`, found `'a`
-   --> $DIR/label-after-loop-kw.rs:20:16
+   --> $DIR/label-after-block-like.rs:20:16
92    |
93 LL |     while true 'a {}
94    |     ----- ---- ^^ expected `{`
102    |                +       +
103 
104 error: labeled expression must be followed by `:`
104 error: labeled expression must be followed by `:`
-   --> $DIR/label-after-loop-kw.rs:26:23
+   --> $DIR/label-after-block-like.rs:26:23
106    |
107 LL |     while let () = () 'a {}
108    |                       ---^^

113    = note: labels are used before loops and blocks, allowing e.g., `break 'label` to them
115 error: expected `{`, found `'a`
115 error: expected `{`, found `'a`
-   --> $DIR/label-after-loop-kw.rs:26:23
+   --> $DIR/label-after-block-like.rs:26:23
117    |
118 LL |     while let () = () 'a {}
119    |     ----- ----------- ^^ expected `{`
127    |                       +       +
128 
129 error: labeled expression must be followed by `:`
129 error: labeled expression must be followed by `:`
-   --> $DIR/label-after-loop-kw.rs:32:19
+   --> $DIR/label-after-block-like.rs:32:19
131    |
132 LL |     for _ in 0..0 'a {}
133    |                   ---^^

138    = note: labels are used before loops and blocks, allowing e.g., `break 'label` to them
140 error: expected `{`, found `'a`
140 error: expected `{`, found `'a`
-   --> $DIR/label-after-loop-kw.rs:32:19
+   --> $DIR/label-after-block-like.rs:32:19
142    |
143 LL |     for _ in 0..0 'a {}

149    |                   +       +
150 
151 error: labeled expression must be followed by `:`
151 error: labeled expression must be followed by `:`
-   --> $DIR/label-after-loop-kw.rs:38:12
+   --> $DIR/label-after-block-like.rs:38:12
153    |
154 LL |     unsafe 'a {}
155    |            ---^^

160    = note: labels are used before loops and blocks, allowing e.g., `break 'label` to them
162 error: expected `{`, found `'a`
162 error: expected `{`, found `'a`
-   --> $DIR/label-after-loop-kw.rs:38:12
+   --> $DIR/label-after-block-like.rs:38:12
164    |
165 LL |     unsafe 'a {}
166    |     ------ ^^ expected `{`

The actual stderr differed from the expected stderr.
The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/label-after-block-like/label-after-block-like.stderr
To only update this specific test, also pass `--test-args parser/label-after-block-like.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/parser/label-after-block-like.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/label-after-block-like" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/label-after-block-like/auxiliary"
stdout: none
--- stderr -------------------------------
error: labeled expression must be followed by `:`
  --> /checkout/src/test/ui/parser/label-after-block-like.rs:2:20
   |
LL |     if let () = () 'a {}
   |                    ---^^
   |                    | |
   |                    | help: add `:` after the label
   |
   |
   = note: labels are used before loops and blocks, allowing e.g., `break 'label` to them
error: expected `{`, found `'a`
  --> /checkout/src/test/ui/parser/label-after-block-like.rs:2:20
   |
   |
LL |     if let () = () 'a {}
   |
note: the `if` expression is missing a block after this condition
  --> /checkout/src/test/ui/parser/label-after-block-like.rs:2:8
   |
   |
LL |     if let () = () 'a {}
help: try placing this code inside a block
   |
   |
LL |     if let () = () { 'a {} }

error: labeled expression must be followed by `:`
  --> /checkout/src/test/ui/parser/label-after-block-like.rs:8:13
   |
   |
LL |     if true 'a {}
   |             ---^^
   |             | |
   |             | help: add `:` after the label
   |
   |
   = note: labels are used before loops and blocks, allowing e.g., `break 'label` to them
error: expected `{`, found `'a`
  --> /checkout/src/test/ui/parser/label-after-block-like.rs:8:13
   |
   |
LL |     if true 'a {}
   |
note: the `if` expression is missing a block after this condition
  --> /checkout/src/test/ui/parser/label-after-block-like.rs:8:8
   |
   |
LL |     if true 'a {}
help: try placing this code inside a block
   |
   |
LL |     if true { 'a {} }

error: labeled expression must be followed by `:`
  --> /checkout/src/test/ui/parser/label-after-block-like.rs:14:10
   |
   |
LL |     loop 'a {}
   |          ---^^
   |          | |
   |          | help: add `:` after the label
   |
   |
   = note: labels are used before loops and blocks, allowing e.g., `break 'label` to them
error: expected `{`, found `'a`
  --> /checkout/src/test/ui/parser/label-after-block-like.rs:14:10
   |
   |
LL |     loop 'a {}
   |     ---- ^^ expected `{`
   |     |
   |     while parsing this `loop` expression
help: try placing this code inside a block
   |
   |
LL |     loop { 'a {} }

error: labeled expression must be followed by `:`
  --> /checkout/src/test/ui/parser/label-after-block-like.rs:20:16
   |
   |
LL |     while true 'a {}
   |                ---^^
   |                | |
   |                | help: add `:` after the label
   |
   |
   = note: labels are used before loops and blocks, allowing e.g., `break 'label` to them
error: expected `{`, found `'a`
  --> /checkout/src/test/ui/parser/label-after-block-like.rs:20:16
   |
   |
LL |     while true 'a {}
   |     ----- ---- ^^ expected `{`
   |     |     this `while` condition successfully parsed
   |     |     this `while` condition successfully parsed
   |     while parsing the body of this `while` expression
help: try placing this code inside a block
   |
   |
LL |     while true { 'a {} }

error: labeled expression must be followed by `:`
  --> /checkout/src/test/ui/parser/label-after-block-like.rs:26:23
   |
   |
LL |     while let () = () 'a {}
   |                       ---^^
   |                       | |
   |                       | help: add `:` after the label
   |
   |
   = note: labels are used before loops and blocks, allowing e.g., `break 'label` to them
error: expected `{`, found `'a`
  --> /checkout/src/test/ui/parser/label-after-block-like.rs:26:23
   |
   |
LL |     while let () = () 'a {}
   |     ----- ----------- ^^ expected `{`
   |     |     this `while` condition successfully parsed
   |     |     this `while` condition successfully parsed
   |     while parsing the body of this `while` expression
help: try placing this code inside a block
   |
   |
LL |     while let () = () { 'a {} }

error: labeled expression must be followed by `:`
  --> /checkout/src/test/ui/parser/label-after-block-like.rs:32:19
   |
   |
LL |     for _ in 0..0 'a {}
   |                   ---^^
   |                   | |
   |                   | help: add `:` after the label
   |
   |
   = note: labels are used before loops and blocks, allowing e.g., `break 'label` to them
error: expected `{`, found `'a`
  --> /checkout/src/test/ui/parser/label-after-block-like.rs:32:19
   |
   |
LL |     for _ in 0..0 'a {}
   |
help: try placing this code inside a block
   |
   |
LL |     for _ in 0..0 { 'a {} }

error: labeled expression must be followed by `:`
  --> /checkout/src/test/ui/parser/label-after-block-like.rs:38:12
   |
   |
LL |     unsafe 'a {}
   |            ---^^
   |            | |
   |            | help: add `:` after the label
   |
   |
   = note: labels are used before loops and blocks, allowing e.g., `break 'label` to them
error: expected `{`, found `'a`
  --> /checkout/src/test/ui/parser/label-after-block-like.rs:38:12
   |
   |
LL |     unsafe 'a {}
   |     ------ ^^ expected `{`
   |     |
   |     while parsing this `unsafe` expression
help: try placing this code inside a block
   |
   |
LL |     unsafe { 'a {} }

error: aborting due to 14 previous errors
------------------------------------------

