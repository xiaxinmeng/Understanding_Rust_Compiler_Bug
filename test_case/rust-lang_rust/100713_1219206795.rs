plain
diff of stderr:

2   --> $DIR/uppercase-base-prefix.rs:6:13
3    |
4 LL |     let a = 0XABCDEF;
-    |             ^^^^^^^^ help: try making the prefix lowercase (notice the capitalization): `0xABCDEF`
+    |             ^^^^^^^^ help: try making the prefix lowercase
6    |
7    = note: base prefixes (`0xff`, `0b1010`, `0o755`) are lowercase

10   --> $DIR/uppercase-base-prefix.rs:12:13
11    |
12 LL |     let b = 0O755;
12 LL |     let b = 0O755;
-    |             ^^^^^ help: try making the prefix lowercase (notice the capitalization): `0o755`
+    |             ^^^^^ help: try making the prefix lowercase
14    |
15    = note: base prefixes (`0xff`, `0b1010`, `0o755`) are lowercase

18   --> $DIR/uppercase-base-prefix.rs:18:13
19    |
20 LL |     let c = 0B10101010;
20 LL |     let c = 0B10101010;
-    |             ^^^^^^^^^^ help: try making the prefix lowercase: `0b10101010`
+    |             ^^^^^^^^^^ help: try making the prefix lowercase
22    |
23    = note: base prefixes (`0xff`, `0b1010`, `0o755`) are lowercase

26   --> $DIR/uppercase-base-prefix.rs:24:13
27    |
27    |
28 LL |     let d = 0XABC_DEF;
-    |             ^^^^^^^^^ help: try making the prefix lowercase (notice the capitalization): `0xABC_DEF`
+    |             ^^^^^^^^^ help: try making the prefix lowercase
30    |
31    = note: base prefixes (`0xff`, `0b1010`, `0o755`) are lowercase

34   --> $DIR/uppercase-base-prefix.rs:30:13
35    |
36 LL |     let e = 0O7_55;
36 LL |     let e = 0O7_55;
-    |             ^^^^^^ help: try making the prefix lowercase (notice the capitalization): `0o7_55`
+    |             ^^^^^^ help: try making the prefix lowercase
38    |
39    = note: base prefixes (`0xff`, `0b1010`, `0o755`) are lowercase

42   --> $DIR/uppercase-base-prefix.rs:36:13
43    |
44 LL |     let f = 0B1010_1010;
44 LL |     let f = 0B1010_1010;
-    |             ^^^^^^^^^^^ help: try making the prefix lowercase: `0b1010_1010`
+    |             ^^^^^^^^^^^ help: try making the prefix lowercase
46    |
47    = note: base prefixes (`0xff`, `0b1010`, `0o755`) are lowercase

50   --> $DIR/uppercase-base-prefix.rs:42:13
51    |
51    |
52 LL |     let g = 0XABC_DEF_u64;
-    |             ^^^^^^^^^^^^^ help: try making the prefix lowercase (notice the capitalization): `0xABC_DEF_u64`
+    |             ^^^^^^^^^^^^^ help: try making the prefix lowercase
54    |
55    = note: base prefixes (`0xff`, `0b1010`, `0o755`) are lowercase

58   --> $DIR/uppercase-base-prefix.rs:48:13
59    |
59    |
60 LL |     let h = 0O7_55_u32;
-    |             ^^^^^^^^^^ help: try making the prefix lowercase (notice the capitalization): `0o7_55_u32`
+    |             ^^^^^^^^^^ help: try making the prefix lowercase
62    |
63    = note: base prefixes (`0xff`, `0b1010`, `0o755`) are lowercase

66   --> $DIR/uppercase-base-prefix.rs:54:13
67    |
68 LL |     let i = 0B1010_1010_u8;
68 LL |     let i = 0B1010_1010_u8;
-    |             ^^^^^^^^^^^^^^ help: try making the prefix lowercase: `0b1010_1010_u8`
+    |             ^^^^^^^^^^^^^^ help: try making the prefix lowercase
70    |
71    = note: base prefixes (`0xff`, `0b1010`, `0o755`) are lowercase

74   --> $DIR/uppercase-base-prefix.rs:60:13
75    |
75    |
76 LL |     let j = 0XABCDEFu64;
-    |             ^^^^^^^^^^^ help: try making the prefix lowercase (notice the capitalization): `0xABCDEFu64`
+    |             ^^^^^^^^^^^ help: try making the prefix lowercase
78    |
79    = note: base prefixes (`0xff`, `0b1010`, `0o755`) are lowercase

82   --> $DIR/uppercase-base-prefix.rs:66:13
83    |
83    |
84 LL |     let k = 0O755u32;
-    |             ^^^^^^^^ help: try making the prefix lowercase (notice the capitalization): `0o755u32`
+    |             ^^^^^^^^ help: try making the prefix lowercase
86    |
87    = note: base prefixes (`0xff`, `0b1010`, `0o755`) are lowercase

90   --> $DIR/uppercase-base-prefix.rs:72:13
91    |
91    |
92 LL |     let l = 0B10101010u8;
-    |             ^^^^^^^^^^^^ help: try making the prefix lowercase: `0b10101010u8`
+    |             ^^^^^^^^^^^^ help: try making the prefix lowercase
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
95    = note: base prefixes (`0xff`, `0b1010`, `0o755`) are lowercase


The actual stderr differed from the expected stderr.
The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/numeric/uppercase-base-prefix/uppercase-base-prefix.stderr
To only update this specific test, also pass `--test-args numeric/uppercase-base-prefix.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/numeric/uppercase-base-prefix.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/numeric/uppercase-base-prefix" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/numeric/uppercase-base-prefix/auxiliary"
stdout: none
--- stderr -------------------------------
error: invalid base prefix for number literal
   |
LL |     let a = 0XABCDEF;
   |             ^^^^^^^^ help: try making the prefix lowercase
   |
   |
   = note: base prefixes (`0xff`, `0b1010`, `0o755`) are lowercase

error: invalid base prefix for number literal
   |
LL |     let b = 0O755;
   |             ^^^^^ help: try making the prefix lowercase
   |
   |
   = note: base prefixes (`0xff`, `0b1010`, `0o755`) are lowercase

error: invalid base prefix for number literal
   |
LL |     let c = 0B10101010;
   |             ^^^^^^^^^^ help: try making the prefix lowercase
   |
   |
   = note: base prefixes (`0xff`, `0b1010`, `0o755`) are lowercase

error: invalid base prefix for number literal
   |
   |
LL |     let d = 0XABC_DEF;
   |             ^^^^^^^^^ help: try making the prefix lowercase
   |
   = note: base prefixes (`0xff`, `0b1010`, `0o755`) are lowercase

error: invalid base prefix for number literal
   |
LL |     let e = 0O7_55;
   |             ^^^^^^ help: try making the prefix lowercase
   |
   |
   = note: base prefixes (`0xff`, `0b1010`, `0o755`) are lowercase

error: invalid base prefix for number literal
   |
LL |     let f = 0B1010_1010;
   |             ^^^^^^^^^^^ help: try making the prefix lowercase
   |
   |
   = note: base prefixes (`0xff`, `0b1010`, `0o755`) are lowercase

error: invalid base prefix for number literal
   |
   |
LL |     let g = 0XABC_DEF_u64;
   |             ^^^^^^^^^^^^^ help: try making the prefix lowercase
   |
   = note: base prefixes (`0xff`, `0b1010`, `0o755`) are lowercase

error: invalid base prefix for number literal
   |
   |
LL |     let h = 0O7_55_u32;
   |             ^^^^^^^^^^ help: try making the prefix lowercase
   |
   = note: base prefixes (`0xff`, `0b1010`, `0o755`) are lowercase

error: invalid base prefix for number literal
   |
LL |     let i = 0B1010_1010_u8;
   |             ^^^^^^^^^^^^^^ help: try making the prefix lowercase
   |
   |
   = note: base prefixes (`0xff`, `0b1010`, `0o755`) are lowercase

error: invalid base prefix for number literal
   |
   |
LL |     let j = 0XABCDEFu64;
   |             ^^^^^^^^^^^ help: try making the prefix lowercase
   |
   = note: base prefixes (`0xff`, `0b1010`, `0o755`) are lowercase

error: invalid base prefix for number literal
   |
   |
LL |     let k = 0O755u32;
   |             ^^^^^^^^ help: try making the prefix lowercase
   |
   = note: base prefixes (`0xff`, `0b1010`, `0o755`) are lowercase

error: invalid base prefix for number literal
   |
   |
LL |     let l = 0B10101010u8;
   |             ^^^^^^^^^^^^ help: try making the prefix lowercase
   |
   = note: base prefixes (`0xff`, `0b1010`, `0o755`) are lowercase
error: aborting due to 12 previous errors
------------------------------------------



---- [ui] src/test/ui/parser/do-catch-suggests-try.rs stdout ----
diff of stderr:

2   --> $DIR/do-catch-suggests-try.rs:4:25
3    |
4 LL |     let _: Option<()> = do catch {};
-    |                         ^^^^^^^^ help: replace with the new syntax: `try`
+    |                         ^^^^^^^^ help: replace with the new syntax
6    |
7    = note: following RFC #2388, the new non-placeholder syntax is `try`


The actual stderr differed from the expected stderr.
The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/do-catch-suggests-try/do-catch-suggests-try.stderr
To only update this specific test, also pass `--test-args parser/do-catch-suggests-try.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/parser/do-catch-suggests-try.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/do-catch-suggests-try" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/do-catch-suggests-try/auxiliary"
stdout: none
--- stderr -------------------------------
error: found removed `do catch` syntax
   |
   |
LL |     let _: Option<()> = do catch {};
   |                         ^^^^^^^^ help: replace with the new syntax
   |
   = note: following RFC #2388, the new non-placeholder syntax is `try`
error[E0308]: mismatched types
  --> /checkout/src/test/ui/parser/do-catch-suggests-try.rs:9:33
   |
   |
LL |     let _recovery_witness: () = 1; //~ ERROR mismatched types
   |                            |
   |                            expected due to this

error: aborting due to 2 previous errors
---
diff of stderr:

2   --> $DIR/issue-52496.rs:4:24
3    |
4 LL |     let _ = Foo { bar: .5, baz: 42 };
-    |                        ^^ help: must have an integer part: `0.5`
+    |                        ^^ help: must have an integer part
6 
7 error: expected one of `,`, `:`, or `}`, found `.`


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/issues/issue-52496/issue-52496.stderr
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/issues/issue-52496/issue-52496.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args parser/issues/issue-52496.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/parser/issues/issue-52496.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/issues/issue-52496" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/issues/issue-52496/auxiliary"
stdout: none
--- stderr -------------------------------
error: float literals must have an integer part
   |
   |
LL |     let _ = Foo { bar: .5, baz: 42 };
   |                        ^^ help: must have an integer part

error: expected one of `,`, `:`, or `}`, found `.`
   |
   |
LL |     let _ = Foo { bar.into(), bat: -1, . };
   |             ---   -  ^ expected one of `,`, `:`, or `}`
   |             |     |
   |             |     help: try naming a field: `bar:`

error: expected identifier, found `.`
  --> /checkout/src/test/ui/parser/issues/issue-52496.rs:8:40
   |
   |
LL |     let _ = Foo { bar.into(), bat: -1, . };
   |             |
   |             while parsing this struct


error[E0063]: missing field `bat` in initializer of `Foo`
   |
   |
LL |     let _ = Foo { bar: .5, baz: 42 };
   |             ^^^ missing `bat`

error[E0063]: missing fields `bar` and `baz` in initializer of `Foo`
   |
   |
LL |     let _ = Foo { bar.into(), bat: -1, . };
   |             ^^^ missing `bar` and `baz`
error: aborting due to 5 previous errors

For more information about this error, try `rustc --explain E0063`.
------------------------------------------
---
4 LL |     'l0 while false {}
-    |     ----^^^^^^^^^^^^^^
+    |     ^^^---------------
6    |     |  |
7    |     |  help: add `:` after the label

13   --> $DIR/labeled-no-colon-expr.rs:5:5
14    |
14    |
15 LL |     'l1 for _ in 0..1 {}
+    |     ^^^-----------------
17    |     |  |
17    |     |  |
18    |     |  help: add `:` after the label

24   --> $DIR/labeled-no-colon-expr.rs:6:5
25    |
25    |
26 LL |     'l2 loop {}
+    |     ^^^--------
28    |     |  |
28    |     |  |
29    |     |  help: add `:` after the label

35   --> $DIR/labeled-no-colon-expr.rs:7:5
36    |
37 LL |     'l3 {}
37 LL |     'l3 {}
-    |     ----^^
+    |     ^^^---
39    |     |  |
40    |     |  help: add `:` after the label

55    |
56 
56 
57 error: labeled expression must be followed by `:`
-   --> $DIR/labeled-no-colon-expr.rs:8:9
59    |
60 LL |     'l4 0;
-    |     ----^
-    |     |  |
-    |     |  |
-    |     |  help: add `:` after the label
+    |     ^^^-- the label
+    |        |
+    |        |
+    |        help: add `:` after the label
65    |
66    = note: labels are used before loops and blocks, allowing e.g., `break 'label` to them

79    = note: this error originates in the macro `m` (in Nightly builds, run with -Z macro-backtrace for more info)
80 
80 
81 error: labeled expression must be followed by `:`
-   --> $DIR/labeled-no-colon-expr.rs:16:8
83    |
84 LL |             'l5 $b;
84 LL |             'l5 $b;
-    |             ---- help: add `:` after the label
-    |             the label
-    |             the label
+    |             ^^^- help: add `:` after the label
88 ...
89 LL |     m!({});
+    |     ------
+    |     |  |
+    |     |  the label
+    |     in this macro invocation
+    |     in this macro invocation
91    |
92    = note: labels are used before loops and blocks, allowing e.g., `break 'label` to them
+    = note: this error originates in the macro `m` (in Nightly builds, run with -Z macro-backtrace for more info)
94 error: aborting due to 8 previous errors
95 



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/labeled-no-colon-expr/labeled-no-colon-expr.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args parser/labeled-no-colon-expr.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/parser/labeled-no-colon-expr.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/labeled-no-colon-expr" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/labeled-no-colon-expr/auxiliary"
stdout: none
--- stderr -------------------------------
error: labeled expression must be followed by `:`
   |
   |
LL |     'l0 while false {} //~ ERROR labeled expression must be followed by `:`
   |     ^^^---------------
   |     |  |
   |     |  help: add `:` after the label
   |
   |
   = note: labels are used before loops and blocks, allowing e.g., `break 'label` to them

error: labeled expression must be followed by `:`
   |
   |
LL |     'l1 for _ in 0..1 {} //~ ERROR labeled expression must be followed by `:`
   |     ^^^-----------------
   |     |  |
   |     |  help: add `:` after the label
   |
   |
   = note: labels are used before loops and blocks, allowing e.g., `break 'label` to them

error: labeled expression must be followed by `:`
   |
   |
LL |     'l2 loop {} //~ ERROR labeled expression must be followed by `:`
   |     ^^^--------
   |     |  |
   |     |  help: add `:` after the label
   |
   |
   = note: labels are used before loops and blocks, allowing e.g., `break 'label` to them

error: labeled expression must be followed by `:`
   |
   |
LL |     'l3 {} //~ ERROR labeled expression must be followed by `:`
   |     ^^^---
   |     |  |
   |     |  help: add `:` after the label
   |
   |
   = note: labels are used before loops and blocks, allowing e.g., `break 'label` to them

error: expected `while`, `for`, `loop` or `{` after a label
   |
   |
LL |     'l4 0; //~ ERROR labeled expression must be followed by `:`
   |         ^ expected `while`, `for`, `loop` or `{` after a label
help: consider removing the label
   |
   |
LL -     'l4 0; //~ ERROR labeled expression must be followed by `:`
LL +     0; //~ ERROR labeled expression must be followed by `:`


error: labeled expression must be followed by `:`
   |
   |
LL |     'l4 0; //~ ERROR labeled expression must be followed by `:`
   |     ^^^-- the label
   |        |
   |        help: add `:` after the label
   |
   = note: labels are used before loops and blocks, allowing e.g., `break 'label` to them

error: cannot use a `block` macro fragment here
   |
   |
LL |             'l5 $b; //~ ERROR cannot use a `block` macro fragment here
   |             |
   |             the `block` fragment is within this context
...
...
LL |     m!({}); //~ ERROR labeled expression must be followed by `:`
   |
   = note: this error originates in the macro `m` (in Nightly builds, run with -Z macro-backtrace for more info)


error: labeled expression must be followed by `:`
   |
   |
LL |             'l5 $b; //~ ERROR cannot use a `block` macro fragment here
   |             ^^^- help: add `:` after the label
...
LL |     m!({}); //~ ERROR labeled expression must be followed by `:`
   |     |  |
   |     |  the label
   |     in this macro invocation
   |
   |
   = note: labels are used before loops and blocks, allowing e.g., `break 'label` to them
   = note: this error originates in the macro `m` (in Nightly builds, run with -Z macro-backtrace for more info)
error: aborting due to 8 previous errors
------------------------------------------



---- [ui] src/test/ui/parser/recover-range-pats.rs stdout ----
diff of stderr:

2   --> $DIR/recover-range-pats.rs:22:12
3    |
4 LL |     if let .0..Y = 0 {}
-    |            ^^ help: must have an integer part: `0.0`
+    |            ^^ help: must have an integer part
7 error: float literals must have an integer part
8   --> $DIR/recover-range-pats.rs:24:16

9    |
9    |
10 LL |     if let X.. .0 = 0 {}
-    |                ^^ help: must have an integer part: `0.0`
+    |                ^^ help: must have an integer part
13 error: float literals must have an integer part
14   --> $DIR/recover-range-pats.rs:35:12

15    |
15    |
16 LL |     if let .0..=Y = 0 {}
-    |            ^^ help: must have an integer part: `0.0`
+    |            ^^ help: must have an integer part
19 error: float literals must have an integer part
20   --> $DIR/recover-range-pats.rs:37:16

21    |
21    |
22 LL |     if let X..=.0 = 0 {}
-    |                ^^ help: must have an integer part: `0.0`
+    |                ^^ help: must have an integer part
25 error: float literals must have an integer part
26   --> $DIR/recover-range-pats.rs:60:12

27    |
27    |
28 LL |     if let .0...Y = 0 {}
-    |            ^^ help: must have an integer part: `0.0`
+    |            ^^ help: must have an integer part
31 error: float literals must have an integer part
32   --> $DIR/recover-range-pats.rs:64:17

33    |
33    |
34 LL |     if let X... .0 = 0 {}
-    |                 ^^ help: must have an integer part: `0.0`
+    |                 ^^ help: must have an integer part
37 error: float literals must have an integer part
38   --> $DIR/recover-range-pats.rs:75:12

39    |
39    |
40 LL |     if let .0.. = 0 {}
-    |            ^^ help: must have an integer part: `0.0`
+    |            ^^ help: must have an integer part
43 error[E0586]: inclusive range with no end
44   --> $DIR/recover-range-pats.rs:81:13

68   --> $DIR/recover-range-pats.rs:85:12
68   --> $DIR/recover-range-pats.rs:85:12
69    |
70 LL |     if let .0..= = 0 {}
-    |            ^^ help: must have an integer part: `0.0`
+    |            ^^ help: must have an integer part
73 error[E0586]: inclusive range with no end
74   --> $DIR/recover-range-pats.rs:85:14

106   --> $DIR/recover-range-pats.rs:95:12
106   --> $DIR/recover-range-pats.rs:95:12
107    |
108 LL |     if let .0... = 0 {}
-    |            ^^ help: must have an integer part: `0.0`
+    |            ^^ help: must have an integer part
111 error[E0586]: inclusive range with no end
112   --> $DIR/recover-range-pats.rs:95:14

120   --> $DIR/recover-range-pats.rs:105:15
120   --> $DIR/recover-range-pats.rs:105:15
121    |
122 LL |     if let .. .0 = 0 {}
-    |               ^^ help: must have an integer part: `0.0`
+    |               ^^ help: must have an integer part
125 error: float literals must have an integer part
126   --> $DIR/recover-range-pats.rs:115:15

127    |
127    |
128 LL |     if let ..=.0 = 0 {}
-    |               ^^ help: must have an integer part: `0.0`
+    |               ^^ help: must have an integer part
130 
131 error: range-to patterns with `...` are not allowed

150   --> $DIR/recover-range-pats.rs:128:15
151    |
152 LL |     if let ....3 = 0 {}
152 LL |     if let ....3 = 0 {}
-    |               ^^ help: must have an integer part: `0.3`
+    |               ^^ help: must have an integer part
154 
155 error: range-to patterns with `...` are not allowed


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/recover-range-pats/recover-range-pats.stderr
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/recover-range-pats/recover-range-pats.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args parser/recover-range-pats.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/parser/recover-range-pats.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/recover-range-pats" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/recover-range-pats/auxiliary"
stdout: none
--- stderr -------------------------------
error: float literals must have an integer part
   |
   |
LL |     if let .0..Y = 0 {} //~ ERROR mismatched types
   |            ^^ help: must have an integer part
error: float literals must have an integer part
  --> /checkout/src/test/ui/parser/recover-range-pats.rs:24:16
   |
   |
LL |     if let X.. .0 = 0 {} //~ ERROR mismatched types
   |                ^^ help: must have an integer part
error: float literals must have an integer part
  --> /checkout/src/test/ui/parser/recover-range-pats.rs:35:12
   |
   |
LL |     if let .0..=Y = 0 {} //~ ERROR mismatched types
   |            ^^ help: must have an integer part
error: float literals must have an integer part
  --> /checkout/src/test/ui/parser/recover-range-pats.rs:37:16
   |
   |
LL |     if let X..=.0 = 0 {} //~ ERROR mismatched types
   |                ^^ help: must have an integer part
error: float literals must have an integer part
  --> /checkout/src/test/ui/parser/recover-range-pats.rs:60:12
   |
   |
LL |     if let .0...Y = 0 {} //~ ERROR mismatched types
   |            ^^ help: must have an integer part
error: float literals must have an integer part
  --> /checkout/src/test/ui/parser/recover-range-pats.rs:64:17
   |
   |
LL |     if let X... .0 = 0 {} //~ ERROR mismatched types
   |                 ^^ help: must have an integer part
error: float literals must have an integer part
  --> /checkout/src/test/ui/parser/recover-range-pats.rs:75:12
   |
LL |     if let .0.. = 0 {}
LL |     if let .0.. = 0 {}
   |            ^^ help: must have an integer part

error[E0586]: inclusive range with no end
  --> /checkout/src/test/ui/parser/recover-range-pats.rs:81:13
   |
LL |     if let 0..= = 0 {} //~ ERROR inclusive range with no end
   |             ^^^ help: use `..` instead
   |
   = note: inclusive ranges must be bounded at the end (`..=b` or `a..=b`)
error[E0586]: inclusive range with no end
  --> /checkout/src/test/ui/parser/recover-range-pats.rs:82:13
   |
   |
LL |     if let X..= = 0 {} //~ ERROR inclusive range with no end
   |             ^^^ help: use `..` instead
   |
   = note: inclusive ranges must be bounded at the end (`..=b` or `a..=b`)
error[E0586]: inclusive range with no end
  --> /checkout/src/test/ui/parser/recover-range-pats.rs:83:16
   |
   |
LL |     if let true..= = 0 {} //~ ERROR inclusive range with no end
   |                ^^^ help: use `..` instead
   |
   = note: inclusive ranges must be bounded at the end (`..=b` or `a..=b`)
error: float literals must have an integer part
  --> /checkout/src/test/ui/parser/recover-range-pats.rs:85:12
   |
   |
LL |     if let .0..= = 0 {} //~ ERROR inclusive range with no end
   |            ^^ help: must have an integer part
error[E0586]: inclusive range with no end
  --> /checkout/src/test/ui/parser/recover-range-pats.rs:85:14
   |
   |
LL |     if let .0..= = 0 {} //~ ERROR inclusive range with no end
   |              ^^^ help: use `..` instead
   |
   = note: inclusive ranges must be bounded at the end (`..=b` or `a..=b`)
error[E0586]: inclusive range with no end
  --> /checkout/src/test/ui/parser/recover-range-pats.rs:91:13
   |
   |
LL |     if let 0... = 0 {} //~ ERROR inclusive range with no end
   |             ^^^ help: use `..` instead
   |
   = note: inclusive ranges must be bounded at the end (`..=b` or `a..=b`)
error[E0586]: inclusive range with no end
  --> /checkout/src/test/ui/parser/recover-range-pats.rs:92:13
   |
   |
LL |     if let X... = 0 {} //~ ERROR inclusive range with no end
   |             ^^^ help: use `..` instead
   |
   = note: inclusive ranges must be bounded at the end (`..=b` or `a..=b`)
error[E0586]: inclusive range with no end
  --> /checkout/src/test/ui/parser/recover-range-pats.rs:93:16
   |
   |
LL |     if let true... = 0 {} //~ ERROR inclusive range with no end
   |                ^^^ help: use `..` instead
   |
   = note: inclusive ranges must be bounded at the end (`..=b` or `a..=b`)
error: float literals must have an integer part
  --> /checkout/src/test/ui/parser/recover-range-pats.rs:95:12
   |
   |
LL |     if let .0... = 0 {} //~ ERROR inclusive range with no end
   |            ^^ help: must have an integer part
error[E0586]: inclusive range with no end
  --> /checkout/src/test/ui/parser/recover-range-pats.rs:95:14
   |
   |
LL |     if let .0... = 0 {} //~ ERROR inclusive range with no end
   |              ^^^ help: use `..` instead
   |
   = note: inclusive ranges must be bounded at the end (`..=b` or `a..=b`)
error: float literals must have an integer part
  --> /checkout/src/test/ui/parser/recover-range-pats.rs:105:15
   |
   |
LL |     if let .. .0 = 0 {}
   |               ^^ help: must have an integer part
error: float literals must have an integer part
  --> /checkout/src/test/ui/parser/recover-range-pats.rs:115:15
   |
   |
LL |     if let ..=.0 = 0 {}
   |               ^^ help: must have an integer part

error: range-to patterns with `...` are not allowed
   |
LL |     if let ...3 = 0 {}
   |            ^^^ help: use `..=` instead


error: range-to patterns with `...` are not allowed
   |
   |
LL |     if let ...Y = 0 {}
   |            ^^^ help: use `..=` instead

error: range-to patterns with `...` are not allowed
   |
   |
LL |     if let ...true = 0 {}
   |            ^^^ help: use `..=` instead
error: float literals must have an integer part
  --> /checkout/src/test/ui/parser/recover-range-pats.rs:128:15
   |
LL |     if let ....3 = 0 {}
LL |     if let ....3 = 0 {}
   |               ^^ help: must have an integer part

error: range-to patterns with `...` are not allowed
   |
LL |     if let ....3 = 0 {}
   |            ^^^ help: use `..=` instead


error: range-to patterns with `...` are not allowed
   |
LL |             let ...$e;
   |                 ^^^ help: use `..=` instead
...
---

error[E0586]: inclusive range with no end
  --> /checkout/src/test/ui/parser/recover-range-pats.rs:154:19
   |
LL |             let $e...; //~ ERROR inclusive range with no end
   |                   ^^^ help: use `..` instead
LL |     mac!(0);
   |     ------- in this macro invocation
   |
   |
   = note: inclusive ranges must be bounded at the end (`..=b` or `a..=b`)
   = note: this error originates in the macro `mac` (in Nightly builds, run with -Z macro-backtrace for more info)
error[E0586]: inclusive range with no end
  --> /checkout/src/test/ui/parser/recover-range-pats.rs:155:19
   |
   |
LL |             let $e..=; //~ ERROR inclusive range with no end
   |                   ^^^ help: use `..` instead
LL |     mac!(0);
   |     ------- in this macro invocation
   |
   |
   = note: inclusive ranges must be bounded at the end (`..=b` or `a..=b`)
   = note: this error originates in the macro `mac` (in Nightly builds, run with -Z macro-backtrace for more info)
error: `...` range patterns are deprecated
  --> /checkout/src/test/ui/parser/recover-range-pats.rs:42:13
   |
LL |     if let 0...3 = 0 {}
---

error: `...` range patterns are deprecated
  --> /checkout/src/test/ui/parser/recover-range-pats.rs:45:13
   |
LL |     if let 0...Y = 0 {}
   |
   = warning: this is accepted in the current edition (Rust 2015) but is a hard error in Rust 2021!
   = note: for more information, see <https://doc.rust-lang.org/nightly/edition-guide/rust-2021/warnings-promoted-to-error.html>


error: `...` range patterns are deprecated
  --> /checkout/src/test/ui/parser/recover-range-pats.rs:48:13
   |
LL |     if let X...3 = 0 {}
   |
   = warning: this is accepted in the current edition (Rust 2015) but is a hard error in Rust 2021!
   = note: for more information, see <https://doc.rust-lang.org/nightly/edition-guide/rust-2021/warnings-promoted-to-error.html>


error: `...` range patterns are deprecated
  --> /checkout/src/test/ui/parser/recover-range-pats.rs:51:13
   |
LL |     if let X...Y = 0 {}
   |
   = warning: this is accepted in the current edition (Rust 2015) but is a hard error in Rust 2021!
   = note: for more information, see <https://doc.rust-lang.org/nightly/edition-guide/rust-2021/warnings-promoted-to-error.html>


error: `...` range patterns are deprecated
  --> /checkout/src/test/ui/parser/recover-range-pats.rs:54:16
   |
LL |     if let true...Y = 0 {} //~ ERROR only `char` and numeric types
   |
   = warning: this is accepted in the current edition (Rust 2015) but is a hard error in Rust 2021!
   = note: for more information, see <https://doc.rust-lang.org/nightly/edition-guide/rust-2021/warnings-promoted-to-error.html>


error: `...` range patterns are deprecated
  --> /checkout/src/test/ui/parser/recover-range-pats.rs:57:13
   |
LL |     if let X...true = 0 {} //~ ERROR only `char` and numeric types
   |
   = warning: this is accepted in the current edition (Rust 2015) but is a hard error in Rust 2021!
   = note: for more information, see <https://doc.rust-lang.org/nightly/edition-guide/rust-2021/warnings-promoted-to-error.html>


error: `...` range patterns are deprecated
  --> /checkout/src/test/ui/parser/recover-range-pats.rs:60:14
   |
LL |     if let .0...Y = 0 {} //~ ERROR mismatched types
   |
   = warning: this is accepted in the current edition (Rust 2015) but is a hard error in Rust 2021!
   = note: for more information, see <https://doc.rust-lang.org/nightly/edition-guide/rust-2021/warnings-promoted-to-error.html>


error: `...` range patterns are deprecated
  --> /checkout/src/test/ui/parser/recover-range-pats.rs:64:13
   |
LL |     if let X... .0 = 0 {} //~ ERROR mismatched types
   |
   = warning: this is accepted in the current edition (Rust 2015) but is a hard error in Rust 2021!
   = note: for more information, see <https://doc.rust-lang.org/nightly/edition-guide/rust-2021/warnings-promoted-to-error.html>


error: `...` range patterns are deprecated
  --> /checkout/src/test/ui/parser/recover-range-pats.rs:138:20
   |
LL |             let $e1...$e2;
...
LL |     mac2!(0, 1);
   |     ----------- in this macro invocation
   |
   |
   = warning: this is accepted in the current edition (Rust 2015) but is a hard error in Rust 2021!
   = note: for more information, see <https://doc.rust-lang.org/nightly/edition-guide/rust-2021/warnings-promoted-to-error.html>
   = note: this error originates in the macro `mac2` (in Nightly builds, run with -Z macro-backtrace for more info)

error[E0029]: only `char` and numeric types are allowed in range patterns
   |
   |
LL |     if let true..Y = 0 {} //~ ERROR only `char` and numeric types
   |            ^^^^  - this is of type `u8`
   |            |
   |            this is of type `bool` but it should be `char` or numeric

error[E0029]: only `char` and numeric types are allowed in range patterns
   |
   |
LL |     if let X..true = 0 {} //~ ERROR only `char` and numeric types
   |            -  ^^^^ this is of type `bool` but it should be `char` or numeric
   |            this is of type `u8`

error[E0308]: mismatched types
  --> /checkout/src/test/ui/parser/recover-range-pats.rs:22:12
  --> /checkout/src/test/ui/parser/recover-range-pats.rs:22:12
   |
LL |     if let .0..Y = 0 {} //~ ERROR mismatched types
   |            ^^  -   - this expression has type `{integer}`
   |            |   |
   |            expected integer, found floating-point number

error[E0308]: mismatched types
  --> /checkout/src/test/ui/parser/recover-range-pats.rs:24:16
  --> /checkout/src/test/ui/parser/recover-range-pats.rs:24:16
   |
LL |     if let X.. .0 = 0 {} //~ ERROR mismatched types
   |            -   ^^   - this expression has type `u8`
   |            |   |
   |            |   expected integer, found floating-point number
   |
   = note: expected type `u8`
              found type `{float}`


error[E0029]: only `char` and numeric types are allowed in range patterns
   |
   |
LL |     if let true..=Y = 0 {} //~ ERROR only `char` and numeric types
   |            |
   |            |
   |            this is of type `bool` but it should be `char` or numeric

error[E0029]: only `char` and numeric types are allowed in range patterns
   |
   |
LL |     if let X..=true = 0 {} //~ ERROR only `char` and numeric types
   |            -   ^^^^ this is of type `bool` but it should be `char` or numeric
   |            this is of type `u8`

error[E0308]: mismatched types
  --> /checkout/src/test/ui/parser/recover-range-pats.rs:35:12
  --> /checkout/src/test/ui/parser/recover-range-pats.rs:35:12
   |
LL |     if let .0..=Y = 0 {} //~ ERROR mismatched types
   |            ^^   -   - this expression has type `{integer}`
   |            |    this is of type `u8`
   |            expected integer, found floating-point number

error[E0308]: mismatched types
error[E0308]: mismatched types
  --> /checkout/src/test/ui/parser/recover-range-pats.rs:37:16
   |
LL |     if let X..=.0 = 0 {} //~ ERROR mismatched types
   |            -   ^^   - this expression has type `u8`
   |            |   |
   |            |   expected integer, found floating-point number
   |
   = note: expected type `u8`
              found type `{float}`


error[E0029]: only `char` and numeric types are allowed in range patterns
   |
   |
LL |     if let true...Y = 0 {} //~ ERROR only `char` and numeric types
   |            |
   |            |
   |            this is of type `bool` but it should be `char` or numeric

error[E0029]: only `char` and numeric types are allowed in range patterns
   |
   |
LL |     if let X...true = 0 {} //~ ERROR only `char` and numeric types
   |            -   ^^^^ this is of type `bool` but it should be `char` or numeric
   |            this is of type `u8`

error[E0308]: mismatched types
  --> /checkout/src/test/ui/parser/recover-range-pats.rs:60:12
  --> /checkout/src/test/ui/parser/recover-range-pats.rs:60:12
   |
LL |     if let .0...Y = 0 {} //~ ERROR mismatched types
   |            ^^   -   - this expression has type `{integer}`
   |            |    this is of type `u8`
   |            expected integer, found floating-point number

error[E0308]: mismatched types
error[E0308]: mismatched types
  --> /checkout/src/test/ui/parser/recover-range-pats.rs:64:17
   |
LL |     if let X... .0 = 0 {} //~ ERROR mismatched types
   |            -    ^^   - this expression has type `u8`
   |            |    expected integer, found floating-point number
   |            this is of type `u8`
   |
   = note: expected type `u8`
   = note: expected type `u8`
              found type `{float}`

error[E0029]: only `char` and numeric types are allowed in range patterns
   |
   |
LL |     if let true.. = 0 {}
   |            ^^^^ this is of type `bool` but it should be `char` or numeric
error[E0308]: mismatched types
  --> /checkout/src/test/ui/parser/recover-range-pats.rs:75:12
   |
LL |     if let .0.. = 0 {}
LL |     if let .0.. = 0 {}
   |            ^^     - this expression has type `{integer}`
   |            |
   |            expected integer, found floating-point number

error[E0029]: only `char` and numeric types are allowed in range patterns
   |
   |
LL |     if let true..= = 0 {} //~ ERROR inclusive range with no end
   |            ^^^^ this is of type `bool` but it should be `char` or numeric
error[E0308]: mismatched types
  --> /checkout/src/test/ui/parser/recover-range-pats.rs:85:12
   |
   |
LL |     if let .0..= = 0 {} //~ ERROR inclusive range with no end
   |            ^^      - this expression has type `{integer}`
   |            expected integer, found floating-point number


error[E0029]: only `char` and numeric types are allowed in range patterns
   |
   |
LL |     if let true... = 0 {} //~ ERROR inclusive range with no end
   |            ^^^^ this is of type `bool` but it should be `char` or numeric
error[E0308]: mismatched types
  --> /checkout/src/test/ui/parser/recover-range-pats.rs:95:12
   |
   |
LL |     if let .0... = 0 {} //~ ERROR inclusive range with no end
   |            ^^      - this expression has type `{integer}`
   |            expected integer, found floating-point number


error[E0029]: only `char` and numeric types are allowed in range patterns
   |
   |
LL |     if let ..true = 0 {}
   |              ^^^^ this is of type `bool` but it should be `char` or numeric
error[E0308]: mismatched types
  --> /checkout/src/test/ui/parser/recover-range-pats.rs:105:15
   |
   |
LL |     if let .. .0 = 0 {}
   |               ^^   - this expression has type `{integer}`
   |               expected integer, found floating-point number


error[E0029]: only `char` and numeric types are allowed in range patterns
   |
   |
LL |     if let ..=true = 0 {}
   |               ^^^^ this is of type `bool` but it should be `char` or numeric
error[E0308]: mismatched types
  --> /checkout/src/test/ui/parser/recover-range-pats.rs:115:15
   |
   |
LL |     if let ..=.0 = 0 {}
   |               ^^   - this expression has type `{integer}`
   |               expected integer, found floating-point number


error[E0029]: only `char` and numeric types are allowed in range patterns
   |
   |
LL |     if let ...true = 0 {}
   |               ^^^^ this is of type `bool` but it should be `char` or numeric
error[E0308]: mismatched types
  --> /checkout/src/test/ui/parser/recover-range-pats.rs:128:15
   |
LL |     if let ....3 = 0 {}
---
diff of stderr:

2   --> $DIR/recover-invalid-float.rs:4:18
3    |
4 LL |     let _: f32 = .3;
-    |                  ^^ help: must have an integer part: `0.3`
+    |                  ^^ help: must have an integer part
7 error: float literals must have an integer part
8   --> $DIR/recover-invalid-float.rs:6:18

9    |
9    |
10 LL |     let _: f32 = .42f32;
-    |                  ^^^^^^ help: must have an integer part: `0.42f32`
+    |                  ^^^^^^ help: must have an integer part
13 error: float literals must have an integer part
14   --> $DIR/recover-invalid-float.rs:8:18

15    |
15    |
16 LL |     let _: f64 = .5f64;
-    |                  ^^^^^ help: must have an integer part: `0.5f64`
+    |                  ^^^^^ help: must have an integer part
19 error: aborting due to 3 previous errors
20 



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/suggestions/recover-invalid-float/recover-invalid-float.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args suggestions/recover-invalid-float.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/suggestions/recover-invalid-float.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/suggestions/recover-invalid-float" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/suggestions/recover-invalid-float/auxiliary"
stdout: none
--- stderr -------------------------------
error: float literals must have an integer part
   |
   |
LL |     let _: f32 = .3;
   |                  ^^ help: must have an integer part
error: float literals must have an integer part
  --> /checkout/src/test/ui/suggestions/recover-invalid-float.rs:6:18
   |
   |
LL |     let _: f32 = .42f32;
   |                  ^^^^^^ help: must have an integer part
error: float literals must have an integer part
  --> /checkout/src/test/ui/suggestions/recover-invalid-float.rs:8:18
   |
   |
LL |     let _: f64 = .5f64;
   |                  ^^^^^ help: must have an integer part
error: aborting due to 3 previous errors
------------------------------------------


