plain
.................................................................................................... 2700/12579
.................................................................................................... 2800/12579
.............................................................i...................................... 2900/12579
.................................................................................................... 3000/12579
........................F.......F..F.........................................iiiii.................. 3100/12579
.................................................................................................... 3300/12579
..........................................................F......................................... 3400/12579
.................................................................................................... 3500/12579
.................................................................................................... 3600/12579
---
.................................................................................................... 10700/12579
.................................................................................................... 10800/12579
.................................................................................................... 10900/12579
.................................................................................................... 11000/12579
......................F............................................................................. 11100/12579
.....F..........................................................F................................... 11200/12579
.................................................................................................... 11400/12579
.................................................................................................... 11500/12579
.................................................................................................... 11600/12579
.................................................................................................... 11700/12579
---
5    |     - ^
6    |     |
7    |     cannot assign to this expression
+    |
+ help: consider dereferencing here to assign a value to the left-hand side
+ LL -     1 = 2;
+ LL +     1 = 2;
+    | 
8 
---
To only update this specific test, also pass `--test-args async-await/issue-73741-type-err.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/async-await/issue-73741-type-err.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/issue-73741-type-err" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--edition=2018" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/issue-73741-type-err/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0070]: invalid left-hand side of assignment
   |
   |
LL |     1 = 2; //~ ERROR invalid left-hand side
   |     - ^
   |     cannot assign to this expression
   |
   |
help: consider dereferencing here to assign a value to the left-hand side
   |
LL -     1 = 2; //~ ERROR invalid left-hand side
LL +     1 = 2; //~ ERROR invalid left-hand side

error: aborting due to previous error

For more information about this error, try `rustc --explain E0070`.
---
5    |     - ^
6    |     |
7    |     cannot assign to this expression
+    |
+ help: consider dereferencing here to assign a value to the left-hand side
+ LL -     1 = 2;
+ LL +     1 = 2;
+    | 
8 
8 
9 error[E0067]: invalid left-hand side of assignment

21    |      -     ^
22    |      |
23    |      cannot assign to this expression
23    |      cannot assign to this expression
+    |
+ help: consider dereferencing here to assign a value to the left-hand side
+    |
+ LL -     (1, 2) = (3, 4);
+ LL +     (1, 2) = (3, 4);
24 
24 
25 error[E0070]: invalid left-hand side of assignment

29    |         -  ^
30    |         |
31    |         cannot assign to this expression
31    |         cannot assign to this expression
+    |
+ help: consider dereferencing here to assign a value to the left-hand side
+    |
+ LL -     (1, 2) = (3, 4);
+ LL +     (1, 2) = (3, 4);
32 
32 
33 error[E0070]: invalid left-hand side of assignment

37    |     ---- ^
38    |     |
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
39    |     cannot assign to this expression
+    |
+ help: consider dereferencing here to assign a value to the left-hand side
+ LL -     None = Some(3);
+ LL +     None = Some(3);
+    | 
40 
---
To only update this specific test, also pass `--test-args destructuring-assignment/bad-expr-lhs.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/destructuring-assignment/bad-expr-lhs.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/destructuring-assignment/bad-expr-lhs" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/destructuring-assignment/bad-expr-lhs/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0070]: invalid left-hand side of assignment
  --> /checkout/src/test/ui/destructuring-assignment/bad-expr-lhs.rs:2:7
   |
LL |     1 = 2; //~ ERROR invalid left-hand side of assignment
   |     - ^
   |     cannot assign to this expression
   |
   |
help: consider dereferencing here to assign a value to the left-hand side
   |
LL -     1 = 2; //~ ERROR invalid left-hand side of assignment
LL +     1 = 2; //~ ERROR invalid left-hand side of assignment


error[E0067]: invalid left-hand side of assignment
  --> /checkout/src/test/ui/destructuring-assignment/bad-expr-lhs.rs:3:7
   |
LL |     1 += 2; //~ ERROR invalid left-hand side of assignment
   |     - ^^
   |     cannot assign to this expression


error[E0070]: invalid left-hand side of assignment
  --> /checkout/src/test/ui/destructuring-assignment/bad-expr-lhs.rs:4:12
   |
LL |     (1, 2) = (3, 4);
   |      -     ^
   |      cannot assign to this expression
   |
   |
help: consider dereferencing here to assign a value to the left-hand side
   |
LL -     (1, 2) = (3, 4);
LL +     (1, 2) = (3, 4);


error[E0070]: invalid left-hand side of assignment
  --> /checkout/src/test/ui/destructuring-assignment/bad-expr-lhs.rs:4:12
   |
LL |     (1, 2) = (3, 4);
   |         -  ^
   |         cannot assign to this expression
   |
   |
help: consider dereferencing here to assign a value to the left-hand side
   |
LL -     (1, 2) = (3, 4);
LL +     (1, 2) = (3, 4);


error[E0070]: invalid left-hand side of assignment
  --> /checkout/src/test/ui/destructuring-assignment/bad-expr-lhs.rs:8:10
   |
LL |     None = Some(3); //~ ERROR invalid left-hand side of assignment
   |     ---- ^
   |     cannot assign to this expression
   |
   |
help: consider dereferencing here to assign a value to the left-hand side
   |
LL -     None = Some(3); //~ ERROR invalid left-hand side of assignment
LL +     None = Some(3); //~ ERROR invalid left-hand side of assignment

error: aborting due to 5 previous errors

Some errors have detailed explanations: E0067, E0070.
---
22    |      -      ^
23    |      |
24    |      cannot assign to this expression
+    |
+ help: consider dereferencing here to assign a value to the left-hand side
+    |
+ LL -     (C, ..) = (0,1);
+ LL +     (C, ..) = (0,1);
25 
26 error[E0308]: mismatched types
27   --> $DIR/tuple_destructure_fail.rs:8:5

---
To only update this specific test, also pass `--test-args destructuring-assignment/tuple_destructure_fail.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/destructuring-assignment/tuple_destructure_fail.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/destructuring-assignment/tuple_destructure_fail" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/destructuring-assignment/tuple_destructure_fail/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: `..` can only be used once per tuple pattern
  --> /checkout/src/test/ui/destructuring-assignment/tuple_destructure_fail.rs:5:16
   |
LL |     (a, .., b, ..) = (0, 1); //~ ERROR `..` can only be used once per tuple pattern
   |         --     ^^ can only be used once per tuple pattern
   |         previously used here

error[E0308]: mismatched types
  --> /checkout/src/test/ui/destructuring-assignment/tuple_destructure_fail.rs:6:5
  --> /checkout/src/test/ui/destructuring-assignment/tuple_destructure_fail.rs:6:5
   |
LL |     (a, a, b) = (1, 2); //~ ERROR mismatched types
   |     ^^^^^^^^^ expected a tuple with 2 elements, found one with 3 elements
   |
   = note: expected type `({integer}, {integer})`
             found tuple `(_, _, _)`

error[E0070]: invalid left-hand side of assignment
  --> /checkout/src/test/ui/destructuring-assignment/tuple_destructure_fail.rs:7:13
   |
LL |     (C, ..) = (0,1); //~ ERROR invalid left-hand side of assignment
   |      -      ^
   |      cannot assign to this expression
   |
   |
help: consider dereferencing here to assign a value to the left-hand side
   |
LL -     (C, ..) = (0,1); //~ ERROR invalid left-hand side of assignment
LL +     (C, ..) = (0,1); //~ ERROR invalid left-hand side of assignment

error[E0308]: mismatched types
  --> /checkout/src/test/ui/destructuring-assignment/tuple_destructure_fail.rs:8:5
   |
   |
LL |     (_,) = (1, 2); //~ ERROR mismatched types
   |     ^^^^ expected a tuple with 2 elements, found one with 1 element
   |
   = note: expected type `({integer}, {integer})`
             found tuple `(_,)`
error: aborting due to 4 previous errors

Some errors have detailed explanations: E0070, E0308.
For more information about an error, try `rustc --explain E0070`.
---
75    |     ------ ^
76    |     |
77    |     cannot assign to this expression
+    |
+ help: consider dereferencing here to assign a value to the left-hand side
+    |
+ LL -     test() = TupleStruct(0, 0);
+ LL +     test() = TupleStruct(0, 0);
78 
78 
79 error[E0070]: invalid left-hand side of assignment
80   --> $DIR/tuple_struct_destructure_fail.rs:40:14
83    |     -------- ^
84    |     |
85    |     cannot assign to this expression
+    |
+    |
+ help: consider dereferencing here to assign a value to the left-hand side
+    |
+ LL -     (test)() = TupleStruct(0, 0);
+ LL +     (test)() = TupleStruct(0, 0);
86 
86 
87 error[E0070]: invalid left-hand side of assignment
88   --> $DIR/tuple_struct_destructure_fail.rs:42:38
91    |     -------------------------------- ^
92    |     |
93    |     cannot assign to this expression
+    |
+    |
+ help: consider dereferencing here to assign a value to the left-hand side
+    |
+ LL -     <Alias::<isize> as Test>::test() = TupleStruct(0, 0);
+ LL +     <Alias::<isize> as Test>::test() = TupleStruct(0, 0);
94 
95 error: aborting due to 9 previous errors
96 

---
To only update this specific test, also pass `--test-args destructuring-assignment/tuple_struct_destructure_fail.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/destructuring-assignment/tuple_struct_destructure_fail.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/destructuring-assignment/tuple_struct_destructure_fail" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/destructuring-assignment/tuple_struct_destructure_fail/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: `..` can only be used once per tuple struct or variant pattern
  --> /checkout/src/test/ui/destructuring-assignment/tuple_struct_destructure_fail.rs:23:27
   |
LL |     TupleStruct(a, .., b, ..) = TupleStruct(0, 1);
   |                    --     ^^ can only be used once per tuple struct or variant pattern
   |                    previously used here


error: `..` can only be used once per tuple struct or variant pattern
  --> /checkout/src/test/ui/destructuring-assignment/tuple_struct_destructure_fail.rs:25:35
   |
LL |     Enum::SingleVariant(a, .., b, ..) = Enum::SingleVariant(0, 1);
   |                            --     ^^ can only be used once per tuple struct or variant pattern
   |                            previously used here


error[E0023]: this pattern has 3 fields, but the corresponding tuple struct has 2 fields
  --> /checkout/src/test/ui/destructuring-assignment/tuple_struct_destructure_fail.rs:28:17
   |
LL | struct TupleStruct<S, T>(S, T);
   |                          -  - tuple struct has 2 fields
...
LL |     TupleStruct(a, a, b) = TupleStruct(1, 2);
   |                 ^  ^  ^ expected 2 fields, found 3

error[E0023]: this pattern has 1 field, but the corresponding tuple struct has 2 fields
  --> /checkout/src/test/ui/destructuring-assignment/tuple_struct_destructure_fail.rs:30:17
   |
LL | struct TupleStruct<S, T>(S, T);
   |                          -  - tuple struct has 2 fields
...
LL |     TupleStruct(_) = TupleStruct(1, 2);
   |                 ^ expected 2 fields, found 1
   |
help: use `_` to explicitly ignore each field
   |
LL |     TupleStruct(_, _) = TupleStruct(1, 2);
help: use `..` to ignore all fields
   |
   |
LL |     TupleStruct(..) = TupleStruct(1, 2);


error[E0023]: this pattern has 3 fields, but the corresponding tuple variant has 2 fields
  --> /checkout/src/test/ui/destructuring-assignment/tuple_struct_destructure_fail.rs:32:25
   |
LL |     SingleVariant(S, T)
   |                   -  - tuple variant has 2 fields
...
LL |     Enum::SingleVariant(a, a, b) = Enum::SingleVariant(1, 2);
   |                         ^  ^  ^ expected 2 fields, found 3

error[E0023]: this pattern has 1 field, but the corresponding tuple variant has 2 fields
  --> /checkout/src/test/ui/destructuring-assignment/tuple_struct_destructure_fail.rs:34:25
   |
LL |     SingleVariant(S, T)
   |                   -  - tuple variant has 2 fields
...
LL |     Enum::SingleVariant(_) = Enum::SingleVariant(1, 2);
   |                         ^ expected 2 fields, found 1
   |
help: use `_` to explicitly ignore each field
   |
LL |     Enum::SingleVariant(_, _) = Enum::SingleVariant(1, 2);
help: use `..` to ignore all fields
   |
   |
LL |     Enum::SingleVariant(..) = Enum::SingleVariant(1, 2);


error[E0070]: invalid left-hand side of assignment
  --> /checkout/src/test/ui/destructuring-assignment/tuple_struct_destructure_fail.rs:38:12
   |
LL |     test() = TupleStruct(0, 0);
   |     ------ ^
   |     cannot assign to this expression
   |
   |
help: consider dereferencing here to assign a value to the left-hand side
   |
LL -     test() = TupleStruct(0, 0);
LL +     test() = TupleStruct(0, 0);


error[E0070]: invalid left-hand side of assignment
  --> /checkout/src/test/ui/destructuring-assignment/tuple_struct_destructure_fail.rs:40:14
   |
LL |     (test)() = TupleStruct(0, 0);
   |     -------- ^
   |     cannot assign to this expression
   |
   |
help: consider dereferencing here to assign a value to the left-hand side
   |
LL -     (test)() = TupleStruct(0, 0);
LL +     (test)() = TupleStruct(0, 0);


error[E0070]: invalid left-hand side of assignment
  --> /checkout/src/test/ui/destructuring-assignment/tuple_struct_destructure_fail.rs:42:38
   |
LL |     <Alias::<isize> as Test>::test() = TupleStruct(0, 0);
   |     -------------------------------- ^
   |     cannot assign to this expression
   |
   |
help: consider dereferencing here to assign a value to the left-hand side
   |
LL -     <Alias::<isize> as Test>::test() = TupleStruct(0, 0);
LL +     <Alias::<isize> as Test>::test() = TupleStruct(0, 0);

error: aborting due to 9 previous errors

Some errors have detailed explanations: E0023, E0070.
---
5    |     ---------- ^
6    |     |
7    |     cannot assign to this expression
+    |
+ help: consider dereferencing here to assign a value to the left-hand side
+ LL -     SOME_CONST = 14;
+ LL +     SOME_CONST = 14;
+    | 
8 
8 
9 error[E0070]: invalid left-hand side of assignment
10   --> $DIR/E0070.rs:7:7
13    |     - ^
14    |     |
15    |     cannot assign to this expression
+    |
+    |
+ help: consider dereferencing here to assign a value to the left-hand side
+ LL -     1 = 3;
+ LL +     1 = 3;
+    | 
16 
16 
17 error[E0070]: invalid left-hand side of assignment
18   --> $DIR/E0070.rs:8:23

The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/error-codes/E0070/E0070.stderr
To update references, rerun the tests and pass the `--bless` flag
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args error-codes/E0070.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/error-codes/E0070.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/error-codes/E0070" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/error-codes/E0070/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0070]: invalid left-hand side of assignment
   |
   |
LL |     SOME_CONST = 14; //~ ERROR E0070
   |     ---------- ^
   |     cannot assign to this expression
   |
   |
help: consider dereferencing here to assign a value to the left-hand side
   |
LL -     SOME_CONST = 14; //~ ERROR E0070
LL +     SOME_CONST = 14; //~ ERROR E0070


error[E0070]: invalid left-hand side of assignment
   |
   |
LL |     1 = 3; //~ ERROR E0070
   |     - ^
   |     cannot assign to this expression
   |
   |
help: consider dereferencing here to assign a value to the left-hand side
   |
LL -     1 = 3; //~ ERROR E0070
LL +     1 = 3; //~ ERROR E0070


error[E0070]: invalid left-hand side of assignment
   |
   |
LL |     some_other_func() = 4; //~ ERROR E0070
   |     ----------------- ^
   |     cannot assign to this expression

error: aborting due to 3 previous errors

---
diff of stderr:

11    |     in this macro invocation
12    |
13    = note: this error originates in the macro `not_a_place` (in Nightly builds, run with -Z macro-backtrace for more info)
+ help: consider dereferencing here to assign a value to the left-hand side
+    |
+ LL -     not_a_place!(99);
+ LL +     not_a_place!(99);
14 
14 
15 error[E0067]: invalid left-hand side of assignment


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-26093/issue-26093.stderr
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-26093/issue-26093.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args issues/issue-26093.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues/issue-26093.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-26093" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-26093/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0070]: invalid left-hand side of assignment
   |
   |
LL |         $thing = 42;
...
...
LL |     not_a_place!(99);
   |     |            |
   |     |            cannot assign to this expression
   |     in this macro invocation
   |
   |
   = note: this error originates in the macro `not_a_place` (in Nightly builds, run with -Z macro-backtrace for more info)
help: consider dereferencing here to assign a value to the left-hand side
   |
LL -     not_a_place!(99);
LL +     not_a_place!(99);


error[E0067]: invalid left-hand side of assignment
   |
   |
LL |         $thing += 42;
...
...
LL |     not_a_place!(99);
   |     |            |
   |     |            cannot assign to this expression
   |     in this macro invocation
   |
---

---- [ui] ui/issues/issue-53348.rs stdout ----
diff of stderr:

6 LL |     for i in v {
7 LL |         a = *i.to_string();
8    |             ^^^^^^^^^^^^^^ expected struct `String`, found `str`
+    |
+ help: consider dereferencing here to assign a value to the left-hand side
+    |
+ LL |         *a = *i.to_string();
9 
10 error: aborting due to previous error
11 

---
To only update this specific test, also pass `--test-args issues/issue-53348.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues/issue-53348.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-53348" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-53348/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0308]: mismatched types
  --> /checkout/src/test/ui/issues/issue-53348.rs:10:13
   |
LL |     let mut a = String::new(); //~ NOTE expected due to this value
LL |     for i in v {
LL |     for i in v {
LL |         a = *i.to_string();
   |             ^^^^^^^^^^^^^^ expected struct `String`, found `str`
   |
help: consider dereferencing here to assign a value to the left-hand side
   |
LL |         *a = *i.to_string();

error: aborting due to previous error

For more information about this error, try `rustc --explain E0308`.
---
45    |             -  ^
46    |             |
47    |             cannot assign to this expression
+    |
+ help: consider dereferencing here to assign a value to the left-hand side
+    |
+ LL -     if Some(3) = foo {}
+ LL +     if Some(3) = foo {}
48 
49 error[E0308]: mismatched types
50   --> $DIR/if-let-typo.rs:8:8

---
To only update this specific test, also pass `--test-args suggestions/if-let-typo.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/suggestions/if-let-typo.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/suggestions/if-let-typo" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/suggestions/if-let-typo/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0425]: cannot find value `x` in this scope
  --> /checkout/src/test/ui/suggestions/if-let-typo.rs:4:13
   |
LL |     if Some(x) = foo {} //~ ERROR cannot find value `x` in this scope
   |
help: you might have meant to use pattern matching
   |
   |
LL |     if let Some(x) = foo {} //~ ERROR cannot find value `x` in this scope

error[E0425]: cannot find value `x` in this scope
  --> /checkout/src/test/ui/suggestions/if-let-typo.rs:10:8
   |
   |
LL |     if x = 5 {}  //~ ERROR cannot find value `x` in this scope
   |
help: you might have meant to use pattern matching
   |
   |
LL |     if let x = 5 {}  //~ ERROR cannot find value `x` in this scope

error[E0308]: mismatched types
  --> /checkout/src/test/ui/suggestions/if-let-typo.rs:4:8
   |
   |
LL |     if Some(x) = foo {} //~ ERROR cannot find value `x` in this scope
   |        ^^^^^^^^^^^^^ expected `bool`, found `()`
error[E0308]: mismatched types
  --> /checkout/src/test/ui/suggestions/if-let-typo.rs:6:8
   |
   |
LL |     if Some(foo) = bar {} //~ ERROR mismatched types
   |        ^^^^^^^^^^^^^^^ expected `bool`, found `()`
error[E0308]: mismatched types
  --> /checkout/src/test/ui/suggestions/if-let-typo.rs:7:8
   |
   |
LL |     if 3 = foo {} //~ ERROR mismatched types
   |        ^^^^^^^ expected `bool`, found `()`

error[E0070]: invalid left-hand side of assignment
   |
   |
LL |     if Some(3) = foo {} //~ ERROR mismatched types
   |             -  ^
   |             cannot assign to this expression
   |
   |
help: consider dereferencing here to assign a value to the left-hand side
   |
LL -     if Some(3) = foo {} //~ ERROR mismatched types
LL +     if Some(3) = foo {} //~ ERROR mismatched types

error[E0308]: mismatched types
  --> /checkout/src/test/ui/suggestions/if-let-typo.rs:8:8
   |
   |
LL |     if Some(3) = foo {} //~ ERROR mismatched types
   |        ^^^^^^^^^^^^^ expected `bool`, found `()`
error: aborting due to 7 previous errors

Some errors have detailed explanations: E0070, E0308, E0425.
For more information about an error, try `rustc --explain E0070`.
---
---- [ui] ui/suggestions/mut-ref-reassignment.rs stdout ----
diff of stderr:

12    |
13 LL |     *opt = None;
14    |     +
+ help: consider dereferencing here to assign a value to the left-hand side
+    |
+ LL |     *opt = None;
15 
16 error[E0308]: mismatched types
17   --> $DIR/mut-ref-reassignment.rs:6:11


35    = note: expected mutable reference `&mut Option<String>`
36                            found enum `Option<String>`
37 help: consider dereferencing here to assign to the mutable borrowed piece of memory
+    |
+ LL |     *opt = Some(String::new())
+    |     +
+ help: consider dereferencing here to assign a value to the left-hand side
38    |
39 LL |     *opt = Some(String::new())


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/suggestions/mut-ref-reassignment/mut-ref-reassignment.stderr
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/suggestions/mut-ref-reassignment/mut-ref-reassignment.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args suggestions/mut-ref-reassignment.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/suggestions/mut-ref-reassignment.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/suggestions/mut-ref-reassignment" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/suggestions/mut-ref-reassignment/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0308]: mismatched types
  --> /checkout/src/test/ui/suggestions/mut-ref-reassignment.rs:2:11
   |
LL | fn suggestion(opt: &mut Option<String>) {
   |                    ------------------- expected due to this parameter type
LL |     opt = None; //~ ERROR mismatched types
   |
   = note: expected mutable reference `&mut Option<String>`
                           found enum `Option<_>`
                           found enum `Option<_>`
help: consider dereferencing here to assign to the mutable borrowed piece of memory
   |
LL |     *opt = None; //~ ERROR mismatched types
   |     +
help: consider dereferencing here to assign a value to the left-hand side
   |
LL |     *opt = None; //~ ERROR mismatched types

error[E0308]: mismatched types
  --> /checkout/src/test/ui/suggestions/mut-ref-reassignment.rs:6:11
   |
   |
LL | fn no_suggestion(opt: &mut Result<String, ()>) {
   |                       ----------------------- expected due to this parameter type
LL |     opt = None //~ ERROR mismatched types
   |
   = note: expected mutable reference `&mut Result<String, ()>`
                           found enum `Option<_>`


error[E0308]: mismatched types
  --> /checkout/src/test/ui/suggestions/mut-ref-reassignment.rs:10:11
   |
LL | fn suggestion2(opt: &mut Option<String>) {
   |                     ------------------- expected due to this parameter type
LL |     opt = Some(String::new())//~ ERROR mismatched types
   |
   = note: expected mutable reference `&mut Option<String>`
                           found enum `Option<String>`
                           found enum `Option<String>`
help: consider dereferencing here to assign to the mutable borrowed piece of memory
   |
LL |     *opt = Some(String::new())//~ ERROR mismatched types
   |     +
help: consider dereferencing here to assign a value to the left-hand side
   |
LL |     *opt = Some(String::new())//~ ERROR mismatched types

error[E0308]: mismatched types
  --> /checkout/src/test/ui/suggestions/mut-ref-reassignment.rs:14:11
   |
   |
LL | fn no_suggestion2(opt: &mut Option<String>) {
   |                        ------------------- expected due to this parameter type
LL |     opt = Some(42)//~ ERROR mismatched types
   |
   = note: expected mutable reference `&mut Option<String>`
                           found enum `Option<{integer}>`

---

38    |
39 LL |     while let Some(3) = foo {}
40    |           +++
+ help: consider dereferencing here to assign a value to the left-hand side
+    |
+ LL -     while Some(3) = foo {}
+ LL +     while Some(3) = foo {}
41 
42 error: aborting due to 4 previous errors
43 

---
To only update this specific test, also pass `--test-args suggestions/while-let-typo.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/suggestions/while-let-typo.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/suggestions/while-let-typo" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/suggestions/while-let-typo/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0425]: cannot find value `x` in this scope
  --> /checkout/src/test/ui/suggestions/while-let-typo.rs:4:16
   |
LL |     while Some(x) = foo {} //~ ERROR cannot find value `x` in this scope
   |
help: you might have meant to use pattern matching
   |
   |
LL |     while let Some(x) = foo {} //~ ERROR cannot find value `x` in this scope

error[E0425]: cannot find value `x` in this scope
  --> /checkout/src/test/ui/suggestions/while-let-typo.rs:8:11
   |
   |
LL |     while x = 5 {} //~ ERROR cannot find value `x` in this scope
   |
help: you might have meant to use pattern matching
   |
   |
LL |     while let x = 5 {} //~ ERROR cannot find value `x` in this scope

error[E0308]: mismatched types
  --> /checkout/src/test/ui/suggestions/while-let-typo.rs:6:11
   |
   |
LL |     while 3 = foo {} //~ ERROR mismatched types
   |           ^^^^^^^ expected `bool`, found `()`

error[E0070]: invalid left-hand side of assignment
   |
   |
LL |     while Some(3) = foo {} //~ ERROR invalid left-hand side of assignment
   |                -  ^
   |                cannot assign to this expression
   |
   |
help: you might have meant to use pattern destructuring
   |
LL |     while let Some(3) = foo {} //~ ERROR invalid left-hand side of assignment
   |           +++
help: consider dereferencing here to assign a value to the left-hand side
   |
LL -     while Some(3) = foo {} //~ ERROR invalid left-hand side of assignment
LL +     while Some(3) = foo {} //~ ERROR invalid left-hand side of assignment

error: aborting due to 4 previous errors

Some errors have detailed explanations: E0070, E0308, E0425.
---
126    |                    - ^
127    |                    |
128    |                    cannot assign to this expression
+    |
+ help: consider dereferencing here to assign a value to the left-hand side
+    |
+ LL -     let _: usize = 0 = 0;
+ LL +     let _: usize = 0 = 0;
129 
130 error[E0308]: mismatched types
131   --> $DIR/assignment-expected-bool.rs:31:20

---
To only update this specific test, also pass `--test-args type/type-check/assignment-expected-bool.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/type/type-check/assignment-expected-bool.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/type/type-check/assignment-expected-bool" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/type/type-check/assignment-expected-bool/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0308]: mismatched types
  --> /checkout/src/test/ui/type/type-check/assignment-expected-bool.rs:6:19
   |
LL |     let _: bool = 0 = 0; //~ ERROR mismatched types [E0308]
   |                   ^^^^^ expected `bool`, found `()`
help: you might have meant to compare for equality
   |
   |
LL |     let _: bool = 0 == 0; //~ ERROR mismatched types [E0308]

error[E0308]: mismatched types
  --> /checkout/src/test/ui/type/type-check/assignment-expected-bool.rs:9:14
   |
   |
LL |         0 => 0 = 0, //~ ERROR mismatched types [E0308]
   |              ^^^^^ expected `bool`, found `()`
help: you might have meant to compare for equality
   |
   |
LL |         0 => 0 == 0, //~ ERROR mismatched types [E0308]

error[E0308]: mismatched types
  --> /checkout/src/test/ui/type/type-check/assignment-expected-bool.rs:10:14
   |
   |
LL |         _ => 0 = 0, //~ ERROR mismatched types [E0308]
   |              ^^^^^ expected `bool`, found `()`
help: you might have meant to compare for equality
   |
   |
LL |         _ => 0 == 0, //~ ERROR mismatched types [E0308]

error[E0308]: mismatched types
  --> /checkout/src/test/ui/type/type-check/assignment-expected-bool.rs:14:17
   |
   |
LL |         true => 0 = 0, //~ ERROR mismatched types [E0308]
   |                 ^^^^^ expected `bool`, found `()`
help: you might have meant to compare for equality
   |
   |
LL |         true => 0 == 0, //~ ERROR mismatched types [E0308]

error[E0308]: mismatched types
  --> /checkout/src/test/ui/type/type-check/assignment-expected-bool.rs:18:8
   |
   |
LL |     if 0 = 0 {} //~ ERROR mismatched types [E0308]
   |        ^^^^^ expected `bool`, found `()`
help: you might have meant to compare for equality
   |
   |
LL |     if 0 == 0 {} //~ ERROR mismatched types [E0308]

error[E0308]: mismatched types
  --> /checkout/src/test/ui/type/type-check/assignment-expected-bool.rs:20:24
   |
   |
LL |     let _: bool = if { 0 = 0 } { //~ ERROR mismatched types [E0308]
   |                        ^^^^^ expected `bool`, found `()`
help: you might have meant to compare for equality
   |
   |
LL |     let _: bool = if { 0 == 0 } { //~ ERROR mismatched types [E0308]

error[E0308]: mismatched types
  --> /checkout/src/test/ui/type/type-check/assignment-expected-bool.rs:21:9
   |
   |
LL |         0 = 0 //~ ERROR mismatched types [E0308]
   |         ^^^^^ expected `bool`, found `()`
help: you might have meant to compare for equality
   |
   |
LL |         0 == 0 //~ ERROR mismatched types [E0308]

error[E0308]: mismatched types
  --> /checkout/src/test/ui/type/type-check/assignment-expected-bool.rs:23:9
   |
   |
LL |         0 = 0 //~ ERROR mismatched types [E0308]
   |         ^^^^^ expected `bool`, found `()`
help: you might have meant to compare for equality
   |
   |
LL |         0 == 0 //~ ERROR mismatched types [E0308]

error[E0308]: mismatched types
  --> /checkout/src/test/ui/type/type-check/assignment-expected-bool.rs:26:13
   |
   |
LL |     let _ = (0 = 0) //~ ERROR mismatched types [E0308]
   |             ^^^^^^^ expected `bool`, found `()`
help: you might have meant to compare for equality
   |
   |
LL |     let _ = (0 == 0) //~ ERROR mismatched types [E0308]

error[E0308]: mismatched types
  --> /checkout/src/test/ui/type/type-check/assignment-expected-bool.rs:27:14
   |
   |
LL |         && { 0 = 0 } //~ ERROR mismatched types [E0308]
   |              ^^^^^ expected `bool`, found `()`
help: you might have meant to compare for equality
   |
   |
LL |         && { 0 == 0 } //~ ERROR mismatched types [E0308]

error[E0308]: mismatched types
  --> /checkout/src/test/ui/type/type-check/assignment-expected-bool.rs:28:12
   |
   |
LL |         || (0 = 0); //~ ERROR mismatched types [E0308]
   |            ^^^^^^^ expected `bool`, found `()`
help: you might have meant to compare for equality
   |
   |
LL |         || (0 == 0); //~ ERROR mismatched types [E0308]


error[E0070]: invalid left-hand side of assignment
   |
   |
LL |     let _: usize = 0 = 0;
   |                    - ^
   |                    cannot assign to this expression
   |
   |
help: consider dereferencing here to assign a value to the left-hand side
   |
LL -     let _: usize = 0 = 0;
LL +     let _: usize = 0 = 0;

error[E0308]: mismatched types
  --> /checkout/src/test/ui/type/type-check/assignment-expected-bool.rs:31:20
   |
   |
LL |     let _: usize = 0 = 0;
   |            -----   ^^^^^ expected `usize`, found `()`
   |            expected due to this

error: aborting due to 13 previous errors

