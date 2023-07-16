plain
.................................................................................................... 400/12284
.................................................................................................... 500/12284
.................................................................................................... 600/12284
..........................................................................i......................... 700/12284
..........F........................................ii............................................F.. 800/12284
.................................................................................................... 1000/12284
.................................................................................................... 1100/12284
....................................................................i.................F............. 1200/12284
.................................................................................................... 1300/12284
---
.................................................................................................... 2600/12284
.................................................................................................... 2700/12284
........................i..i........................................................................ 2800/12284
.................................................................................................... 2900/12284
.........................................iiiii................FF.................................... 3000/12284
................................................................................F..F.........F...... 3100/12284
.................................................................................................... 3300/12284
.................................................................................................... 3400/12284
..............................................................................................i..... 3500/12284
....i.........i................................................F.................................... 3600/12284
....i.........i................................................F.................................... 3600/12284
..................................................................ii................................ 3700/12284
............................F......................................................i................ 3800/12284
............................i....................................................................... 3900/12284
.................................................................................................... 4000/12284
.................................................................................................... 4100/12284
.................................................................................................... 4200/12284
F....F...F..F..........F............................................................................ 4300/12284
.................................................................................................... 4400/12284
.................................F.....F............................................................ 4500/12284
.........................F...F...............F..........iiF......................................... 4600/12284
.................................................................................................... 4800/12284
.................................................................................................... 4900/12284
.................................................................................................... 5000/12284
.................................................................................................... 5100/12284
.................................................................................................... 5100/12284
.................................................................................................... 5200/12284
.................................................................................................... 5300/12284
.................................................................i...i.............................. 5400/12284
.................................................................................................... 5500/12284
..................................................................F.F............................... 5600/12284
.................................................................................................... 5800/12284
.................................................................................................... 5900/12284
...........................................................i........................................ 6000/12284
............................................................F....................................... 6100/12284
............................................................F....................................... 6100/12284
.........................................................i.......................................... 6200/12284
..............................i..................................................................... 6300/12284
.........................................................................................ii.ii...... 6400/12284
.i...i.............................................................................................. 6500/12284
...........................................i....i.........................................i......... 6600/12284
.......i.............i................................................i............................. 6700/12284
.......................................................................i....................F....... 6800/12284
..................................FF.........................F...................................... 6900/12284
.i.................................................................................................. 7000/12284
.............F.......ii..........F............................................i..................... 7100/12284
...F........................................F.............................F..F.F..F................. 7200/12284
........................................................................................F........... 7300/12284
...........................................................................ii...............Fi....i. 7500/12284
.ii................................................................................................. 7600/12284
.................................................................................................... 7700/12284
.................................................................................................... 7800/12284
.................................................................................................... 7800/12284
.................................................................................................... 7900/12284
...............................................i..ii................................................ 8000/12284
..............ii.................................................................................... 8100/12284
.................................................................................................... 8200/12284
..................i.................................i............................................... 8300/12284
.............i...................................................................................... 8400/12284
....................................................i.......F....................................... 8500/12284
...................F................................................................................ 8600/12284
.............................F........i..............................F.............................. 8700/12284
.................................................................................................... 8900/12284
.................................................................................................... 9000/12284
.................................................................................................... 9100/12284
.................................................................................................... 9100/12284
...............................................iiii.iiiii.F............F......FFF....F.............. 9200/12284
......F...............ii..F............i....................................F....................... 9300/12284
...........................................................................................F........ 9400/12284
.................................................................................................... 9600/12284
.................................................................................................... 9700/12284
.................................................................................................... 9800/12284
.................................................................................................... 9900/12284
.................................................................................................... 9900/12284
.......i..ii.i...................................................................................... 10000/12284
.......................................................F.F.......................................... 10100/12284
.................................................................................................... 10300/12284
.................................................................................................... 10400/12284
.........................F.......................................................................... 10500/12284
.................................................................................................... 10600/12284
---
diff of stderr:

5    |                      ^^
6 ...
7 LL | pass_nonterminal!(n!());
+    | ----------------------- in this macro invocation
9    |
9    |
10    = note: this error originates in the macro `pass_nonterminal` (in Nightly builds, run with -Z macro-backtrace for more info)

16    |                ^^^^^^^^^
17 ...
17 ...
18 LL | pass_nonterminal!(n!());
+    | ----------------------- in this macro invocation
20    |
20    |
21    = note: this error originates in the macro `pass_nonterminal` (in Nightly builds, run with -Z macro-backtrace for more info)


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/attributes/nonterminal-expansion/nonterminal-expansion.stderr
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/attributes/nonterminal-expansion/nonterminal-expansion.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args attributes/nonterminal-expansion.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/attributes/nonterminal-expansion.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/attributes/nonterminal-expansion" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/attributes/nonterminal-expansion/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: expected unsuffixed literal or identifier, found `n!()`
   |
   |
LL |         #[repr(align($n))]
...
...
LL | pass_nonterminal!(n!());
   |
   |
   = note: this error originates in the macro `pass_nonterminal` (in Nightly builds, run with -Z macro-backtrace for more info)

error[E0693]: incorrect `repr(align)` attribute format: `align` takes exactly one argument in parentheses
   |
   |
LL |         #[repr(align($n))]
...
...
LL | pass_nonterminal!(n!());
   |
   |
   = note: this error originates in the macro `pass_nonterminal` (in Nightly builds, run with -Z macro-backtrace for more info)
error: aborting due to 2 previous errors

For more information about this error, try `rustc --explain E0693`.

---
diff of stderr:

11    |              ^^^^^^^^^^^^^^^^^^^^^^^^^
12 ...
13 LL | bug!();
+    | ------ in this macro invocation
15    |
15    |
16    = note: this error originates in the macro `bug` (in Nightly builds, run with -Z macro-backtrace for more info)

31    |                       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
32 ...
32 ...
33 LL | some_macro!(u8);
+    | --------------- in this macro invocation
35    |
36    = note: this error originates in the macro `some_macro` (in Nightly builds, run with -Z macro-backtrace for more info)
37 
---
To only update this specific test, also pass `--test-args attributes/key-value-expansion.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/attributes/key-value-expansion.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/attributes/key-value-expansion" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/attributes/key-value-expansion/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: unexpected token: `(7u32)`
  --> /checkout/src/test/ui/attributes/key-value-expansion.rs:21:6
   |
LL | bug!((column!())); //~ ERROR unexpected token: `(7u32)`


error: unexpected token: `"bug" + "found"`
   |
   |
LL |         bug!("bug" + stringify!(found)); //~ ERROR unexpected token: `"bug" + "found"`
...
...
LL | bug!();
   |
   |
   = note: this error originates in the macro `bug` (in Nightly builds, run with -Z macro-backtrace for more info)
error: unexpected token: `{
    let res =
    let res =
        ::alloc::fmt::format(::core::fmt::Arguments::new_v1(&[""],
                                                            &match (&"u8",) {
                                                                 (arg0,) =>
                                                                 [::core::fmt::ArgumentV1::new(arg0,
                                                                                               ::core::fmt::Display::fmt)],
    res
}.as_str()`
  --> /checkout/src/test/ui/attributes/key-value-expansion.rs:48:23
   |
   |
LL |         doc_comment! {format!("{coor}", coor = stringify!($t1)).as_str()}
...
...
LL | some_macro!(u8);
   |
   = note: this error originates in the macro `some_macro` (in Nightly builds, run with -Z macro-backtrace for more info)

error: aborting due to 3 previous errors
---
diff of stderr:

10   ::: $DIR/move-error-snippets.rs:21:1
11    |
12 LL | sss!();
+    | ------ in this macro invocation
14    |
14    |
15    = note: this error originates in the macro `aaa` (in Nightly builds, run with -Z macro-backtrace for more info)


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/borrowck/move-error-snippets/move-error-snippets.stderr
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/borrowck/move-error-snippets/move-error-snippets.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args borrowck/move-error-snippets.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/borrowck/move-error-snippets.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/borrowck/move-error-snippets" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--test" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/borrowck/move-error-snippets/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0507]: cannot move out of static item `D`
   |
LL |         let a = $c;
   |                 ^^
   |                 |
   |                 |
   |                 move occurs because `D` has type `A`, which does not implement the `Copy` trait
   |                 help: consider borrowing here: `&$c`
  ::: /checkout/src/test/ui/borrowck/move-error-snippets.rs:21:1
   |
   |
LL | sss!();
   |
   |
   = note: this error originates in the macro `aaa` (in Nightly builds, run with -Z macro-backtrace for more info)
error: aborting due to previous error

For more information about this error, try `rustc --explain E0507`.


------------------------------------------


---- [ui] ui/dollar-crate/dollar-crate-is-keyword-2.rs stdout ----
diff of stderr:

5    |                ^^^^^^ `$crate` in paths can only be used in start position
6 ...
7 LL | m!();
+    | ---- in this macro invocation
9    |
9    |
10    = note: this error originates in the macro `m` (in Nightly builds, run with -Z macro-backtrace for more info)

16    |             ^^^^^^^^^ no `$crate` in `a`
17 ...
17 ...
18 LL | m!();
+    | ---- in this macro invocation
20    |
20    |
21    = note: this error originates in the macro `m` (in Nightly builds, run with -Z macro-backtrace for more info)


27    |                     ^^^^^^ `$crate` in paths can only be used in start position
28 ...
29 LL | m!();
+    | ---- in this macro invocation
31    |
31    |
32    = note: this error originates in the macro `m` (in Nightly builds, run with -Z macro-backtrace for more info)


The actual stderr differed from the expected stderr.
The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/dollar-crate/dollar-crate-is-keyword-2/dollar-crate-is-keyword-2.stderr
To only update this specific test, also pass `--test-args dollar-crate/dollar-crate-is-keyword-2.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/dollar-crate/dollar-crate-is-keyword-2.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/dollar-crate/dollar-crate-is-keyword-2" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/dollar-crate/dollar-crate-is-keyword-2/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0433]: failed to resolve: `$crate` in paths can only be used in start position
  --> /checkout/src/test/ui/dollar-crate/dollar-crate-is-keyword-2.rs:6:16
   |
LL |         use a::$crate::b; //~ ERROR `$crate` in paths can only be used in start position
   |                ^^^^^^ `$crate` in paths can only be used in start position
...
LL | m!();
   |
   |
   = note: this error originates in the macro `m` (in Nightly builds, run with -Z macro-backtrace for more info)

error[E0432]: unresolved import `a::$crate`
  --> /checkout/src/test/ui/dollar-crate/dollar-crate-is-keyword-2.rs:5:13
   |
LL |         use a::$crate; //~ ERROR unresolved import `a::$crate`
   |             ^^^^^^^^^ no `$crate` in `a`
...
LL | m!();
   |
   |
   = note: this error originates in the macro `m` (in Nightly builds, run with -Z macro-backtrace for more info)

error[E0433]: failed to resolve: `$crate` in paths can only be used in start position
  --> /checkout/src/test/ui/dollar-crate/dollar-crate-is-keyword-2.rs:7:21
   |
LL |         type A = a::$crate; //~ ERROR `$crate` in paths can only be used in start position
   |                     ^^^^^^ `$crate` in paths can only be used in start position
...
LL | m!();
   |
   |
   = note: this error originates in the macro `m` (in Nightly builds, run with -Z macro-backtrace for more info)
error: aborting due to 3 previous errors

Some errors have detailed explanations: E0432, E0433.
For more information about an error, try `rustc --explain E0432`.
---
diff of stderr:

5    |                    ^^^^^^ expected identifier, found reserved identifier
6 ...
7 LL | m!();
+    | ---- in this macro invocation
9    |
9    |
10    = note: this error originates in the macro `m` (in Nightly builds, run with -Z macro-backtrace for more info)

16    |                       ^^^^^^ expected identifier, found reserved identifier
17 ...
17 ...
18 LL | m!();
+    | ---- in this macro invocation
20    |
20    |
21    = note: this error originates in the macro `m` (in Nightly builds, run with -Z macro-backtrace for more info)

27    |         ^^^^^^^^^^^
28 ...
28 ...
29 LL | m!();
+    | ---- in this macro invocation
31    |
31    |
32    = note: this error originates in the macro `m` (in Nightly builds, run with -Z macro-backtrace for more info)

38    |         ^^^^^^^^^^^^^^^^^^^^^
39 ...
39 ...
40 LL | m!();
+    | ---- in this macro invocation
42    |
42    |
43    = note: this error originates in the macro `m` (in Nightly builds, run with -Z macro-backtrace for more info)


The actual stderr differed from the expected stderr.
The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/dollar-crate/dollar-crate-is-keyword/dollar-crate-is-keyword.stderr
To only update this specific test, also pass `--test-args dollar-crate/dollar-crate-is-keyword.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/dollar-crate/dollar-crate-is-keyword.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/dollar-crate/dollar-crate-is-keyword" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/dollar-crate/dollar-crate-is-keyword/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: expected identifier, found reserved identifier `$crate`
  --> /checkout/src/test/ui/dollar-crate/dollar-crate-is-keyword.rs:6:20
   |
LL |             struct $crate {} //~ ERROR expected identifier, found reserved identifier `$crate`
...
...
LL | m!();
   |
   |
   = note: this error originates in the macro `m` (in Nightly builds, run with -Z macro-backtrace for more info)
error: expected identifier, found reserved identifier `$crate`
  --> /checkout/src/test/ui/dollar-crate/dollar-crate-is-keyword.rs:10:23
   |
   |
LL |         use $crate as $crate; //~ ERROR expected identifier, found reserved identifier `$crate`
...
...
LL | m!();
   |
   |
   = note: this error originates in the macro `m` (in Nightly builds, run with -Z macro-backtrace for more info)
error: `$crate` may not be imported
  --> /checkout/src/test/ui/dollar-crate/dollar-crate-is-keyword.rs:9:9
   |
   |
LL |         use $crate; //~ ERROR `$crate` may not be imported
...
...
LL | m!();
   |
   |
   = note: this error originates in the macro `m` (in Nightly builds, run with -Z macro-backtrace for more info)
error: `$crate` may not be imported
  --> /checkout/src/test/ui/dollar-crate/dollar-crate-is-keyword.rs:10:9
   |
   |
LL |         use $crate as $crate; //~ ERROR expected identifier, found reserved identifier `$crate`
...
...
LL | m!();
   |
   |
   = note: this error originates in the macro `m` (in Nightly builds, run with -Z macro-backtrace for more info)
error: aborting due to 4 previous errors


------------------------------------------
------------------------------------------


---- [ui] ui/editions/edition-imports-2018.rs stdout ----
diff of stderr:

2   --> $DIR/edition-imports-2018.rs:24:5
3    |
4 LL |     gen_glob!();
+    |     ^^^^^^^^^^^
6    |
6    |
7    = note: this error originates in the macro `gen_glob` (in Nightly builds, run with -Z macro-backtrace for more info)


The actual stderr differed from the expected stderr.
The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/editions/edition-imports-2018/edition-imports-2018.stderr
To only update this specific test, also pass `--test-args editions/edition-imports-2018.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/editions/edition-imports-2018.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/editions/edition-imports-2018" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--edition=2018" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/editions/edition-imports-2018/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: cannot glob-import all possible crates
  --> /checkout/src/test/ui/editions/edition-imports-2018.rs:24:5
   |
LL |     gen_glob!(); //~ ERROR cannot glob-import all possible crates
   |
   |
   = note: this error originates in the macro `gen_glob` (in Nightly builds, run with -Z macro-backtrace for more info)
error: aborting due to previous error


------------------------------------------
------------------------------------------


---- [ui] ui/editions/edition-imports-virtual-2015-gated.rs stdout ----
diff of stderr:

2   --> $DIR/edition-imports-virtual-2015-gated.rs:8:5
3    |
4 LL |     gen_gated!();
-    |     ^^^^^^^^^^^^^ could not find `E` in the list of imported crates
+    |     ^^^^^^^^^^^^ could not find `E` in the list of imported crates
6    |
7    = note: this error originates in the macro `gen_gated` (in Nightly builds, run with -Z macro-backtrace for more info)


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/editions/edition-imports-virtual-2015-gated/edition-imports-virtual-2015-gated.stderr
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/editions/edition-imports-virtual-2015-gated/edition-imports-virtual-2015-gated.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args editions/edition-imports-virtual-2015-gated.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/editions/edition-imports-virtual-2015-gated.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/editions/edition-imports-virtual-2015-gated" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--edition=2018" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/editions/edition-imports-virtual-2015-gated/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0432]: unresolved import `E`
  --> /checkout/src/test/ui/editions/edition-imports-virtual-2015-gated.rs:8:5
   |
LL |     gen_gated!(); //~ ERROR unresolved import `E`
   |     ^^^^^^^^^^^^ could not find `E` in the list of imported crates
   |
   = note: this error originates in the macro `gen_gated` (in Nightly builds, run with -Z macro-backtrace for more info)
error: aborting due to previous error

For more information about this error, try `rustc --explain E0432`.

---
diff of stderr:

2   --> $DIR/edition-imports-2015.rs:23:5
3    |
4 LL |     gen_glob!();
+    |     ^^^^^^^^^^^
6    |
6    |
7    = note: this error originates in the macro `gen_glob` (in Nightly builds, run with -Z macro-backtrace for more info)


The actual stderr differed from the expected stderr.
The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/editions/edition-imports-2015/edition-imports-2015.stderr
To only update this specific test, also pass `--test-args editions/edition-imports-2015.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/editions/edition-imports-2015.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/editions/edition-imports-2015" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--edition=2015" "--extern" "absolute" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/editions/edition-imports-2015/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: cannot glob-import all possible crates
  --> /checkout/src/test/ui/editions/edition-imports-2015.rs:23:5
   |
LL |     gen_glob!(); //~ ERROR cannot glob-import all possible crates
   |
   |
   = note: this error originates in the macro `gen_glob` (in Nightly builds, run with -Z macro-backtrace for more info)
error: aborting due to previous error


------------------------------------------
------------------------------------------


---- [ui] ui/feature-gates/feature-gate-allow-internal-unsafe-nested-macro.rs stdout ----
diff of stderr:

5    |         ^^^^^^^^^^^^^^^^^^^^^^^^
6 ...
7 LL | bar!();
+    | ------ in this macro invocation
9    |
9    |
10    = help: add `#![feature(allow_internal_unsafe)]` to the crate attributes to enable
11    = note: this error originates in the macro `bar` (in Nightly builds, run with -Z macro-backtrace for more info)

The actual stderr differed from the expected stderr.
The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/feature-gates/feature-gate-allow-internal-unsafe-nested-macro/feature-gate-allow-internal-unsafe-nested-macro.stderr
To only update this specific test, also pass `--test-args feature-gates/feature-gate-allow-internal-unsafe-nested-macro.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/feature-gates/feature-gate-allow-internal-unsafe-nested-macro.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/feature-gates/feature-gate-allow-internal-unsafe-nested-macro" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/feature-gates/feature-gate-allow-internal-unsafe-nested-macro/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0658]: allow_internal_unsafe side-steps the unsafe_code lint
   |
   |
LL |         #[allow_internal_unsafe] //~ ERROR allow_internal_unsafe side-steps
...
...
LL | bar!();
   |
   |
   = help: add `#![feature(allow_internal_unsafe)]` to the crate attributes to enable
   = note: this error originates in the macro `bar` (in Nightly builds, run with -Z macro-backtrace for more info)
error: aborting due to previous error

For more information about this error, try `rustc --explain E0658`.

---
diff of stderr:

2   --> $DIR/thread-local-const-init.rs:1:1
3    |
4 LL | thread_local!(static X: u32 = const { 0 });
+    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
6    |
7    = note: see issue #84223 <https://github.com/rust-lang/rust/issues/84223> for more information
8    = help: add `#![feature(thread_local_const_init)]` to the crate attributes to enable
---
To only update this specific test, also pass `--test-args feature-gates/thread-local-const-init.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/feature-gates/thread-local-const-init.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/feature-gates/thread-local-const-init" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/feature-gates/thread-local-const-init/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0658]: use of unstable library feature 'thread_local_const_init'
  --> /checkout/src/test/ui/feature-gates/thread-local-const-init.rs:1:1
   |
LL | thread_local!(static X: u32 = const { 0 });
   |
   = note: see issue #84223 <https://github.com/rust-lang/rust/issues/84223> for more information
   = help: add `#![feature(thread_local_const_init)]` to the crate attributes to enable
   = help: add `#![feature(thread_local_const_init)]` to the crate attributes to enable
   = note: this error originates in the macro `$crate::__thread_local_inner` (in Nightly builds, run with -Z macro-backtrace for more info)
error: aborting due to previous error

For more information about this error, try `rustc --explain E0658`.

---
diff of stderr:

8    |             not a member of trait `Tr`
9 ...
10 LL |     mac_trait_impl!();
+    |     ----------------- in this macro invocation
12    |
12    |
13    = note: this error originates in the macro `mac_trait_impl` (in Nightly builds, run with -Z macro-backtrace for more info)


22    |         ^^^^^^^^^^^^^^ missing `method` in implementation
23 ...
24 LL |     mac_trait_impl!();
+    |     ----------------- in this macro invocation
26    |
26    |
27    = note: this error originates in the macro `mac_trait_impl` (in Nightly builds, run with -Z macro-backtrace for more info)


The actual stderr differed from the expected stderr.
The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/hygiene/assoc_item_ctxt/assoc_item_ctxt.stderr
To only update this specific test, also pass `--test-args hygiene/assoc_item_ctxt.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/hygiene/assoc_item_ctxt.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/hygiene/assoc_item_ctxt" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/hygiene/assoc_item_ctxt/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0407]: method `method` is not a member of trait `Tr`
   |
   |
LL |             fn method() {} //~ ERROR method `method` is not a member of trait `Tr`
   |             ^^^------^^^^^
   |             |  |
   |             |  help: there is an associated function with a similar name: `method`
   |             not a member of trait `Tr`
...
LL |     mac_trait_impl!();
   |
   |
   = note: this error originates in the macro `mac_trait_impl` (in Nightly builds, run with -Z macro-backtrace for more info)
error[E0046]: not all trait items implemented, missing: `method`
  --> /checkout/src/test/ui/hygiene/assoc_item_ctxt.rs:34:9
   |
LL |         fn method();
LL |         fn method();
   |         ------------ `method` from trait
...
LL |         impl Tr for u8 { //~ ERROR not all trait items implemented, missing: `method`
   |         ^^^^^^^^^^^^^^ missing `method` in implementation
...
LL |     mac_trait_impl!();
   |
   |
   = note: this error originates in the macro `mac_trait_impl` (in Nightly builds, run with -Z macro-backtrace for more info)
error: aborting due to 2 previous errors

Some errors have detailed explanations: E0046, E0407.
For more information about an error, try `rustc --explain E0046`.
---
diff of stderr:

5    |              ^^ declared twice
6 ...
7 LL | m!('a);
+    | ------
9    | |  |
10    | |  previous declaration here
11    | in this macro invocation
11    | in this macro invocation

19    |              ^^ declared twice
20 ...
21 LL | n!('a);
+    | ------
23    | |  |
24    | |  previous declaration here
25    | in this macro invocation
---
To only update this specific test, also pass `--test-args hygiene/duplicate_lifetimes.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/hygiene/duplicate_lifetimes.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/hygiene/duplicate_lifetimes" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/hygiene/duplicate_lifetimes/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0263]: lifetime name `'a` declared twice in the same scope
   |
   |
LL |     fn g<$a, 'a>() {} //~ ERROR lifetime name `'a` declared twice
   |              ^^ declared twice
...
LL | m!('a);
   | |  |
   | |  previous declaration here
   | in this macro invocation
   |
   |
   = note: this error originates in the macro `m` (in Nightly builds, run with -Z macro-backtrace for more info)

error[E0263]: lifetime name `'a` declared twice in the same scope
   |
   |
LL |     fn h<$a, 'a>() {} //~ ERROR lifetime name `'a` declared twice
   |              ^^ declared twice
...
LL | n!('a);
   | |  |
   | |  previous declaration here
   | in this macro invocation
   |
   |
   = note: this error originates in the macro `n` (in Nightly builds, run with -Z macro-backtrace for more info)
error: aborting due to 2 previous errors

For more information about this error, try `rustc --explain E0263`.


------------------------------------------


---- [ui] ui/hygiene/fields-definition.rs stdout ----
diff of stderr:

7    |             ^^^^^^ field already declared
8 ...
9 LL | legacy!(a);
+    | ---------- in this macro invocation
11    |
11    |
12    = note: this error originates in the macro `legacy` (in Nightly builds, run with -Z macro-backtrace for more info)


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/hygiene/fields-definition/fields-definition.stderr
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/hygiene/fields-definition/fields-definition.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args hygiene/fields-definition.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/hygiene/fields-definition.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/hygiene/fields-definition" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/hygiene/fields-definition/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0124]: field `a` is already declared
   |
LL |             a: u8,
LL |             a: u8,
   |             ----- `a` first declared here
LL |             $a: u8, //~ ERROR field `a` is already declared
   |             ^^^^^^ field already declared
...
LL | legacy!(a);
   |
   |
   = note: this error originates in the macro `legacy` (in Nightly builds, run with -Z macro-backtrace for more info)
error: aborting due to previous error

For more information about this error, try `rustc --explain E0124`.


------------------------------------------


---- [ui] ui/hygiene/extern-prelude-from-opaque-fail.rs stdout ----
diff of stderr:

14    |             ^^^^^^^ no `my_core` in the root
15 ...
16 LL | a!();
+    | ---- in this macro invocation
18    |
18    |
19    = note: this error originates in the macro `a` (in Nightly builds, run with -Z macro-backtrace for more info)

25    |                  ^^^^^^^ use of undeclared crate or module `my_core`
26 ...
26 ...
27 LL | a!();
+    | ---- in this macro invocation
29    |
29    |
30    = note: this error originates in the macro `a` (in Nightly builds, run with -Z macro-backtrace for more info)


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/hygiene/extern-prelude-from-opaque-fail/extern-prelude-from-opaque-fail.stderr
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/hygiene/extern-prelude-from-opaque-fail/extern-prelude-from-opaque-fail.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args hygiene/extern-prelude-from-opaque-fail.rs`
error: 1 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/hygiene/extern-prelude-from-opaque-fail.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/hygiene/extern-prelude-from-opaque-fail" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/hygiene/extern-prelude-from-opaque-fail/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0432]: unresolved import `my_core`
   |
   |
LL |     use my_core; //~ ERROR unresolved import `my_core`
   |         |
   |         |
   |         no `my_core` in the root
   |         help: a similar name exists in the module: `my_core`

error[E0432]: unresolved import `my_core`
   |
   |
LL |         use my_core; //~ ERROR unresolved import `my_core`
   |             ^^^^^^^ no `my_core` in the root
...
LL | a!();
   |
   |
   = note: this error originates in the macro `a` (in Nightly builds, run with -Z macro-backtrace for more info)
error[E0433]: failed to resolve: use of undeclared crate or module `my_core`
  --> /checkout/src/test/ui/hygiene/extern-prelude-from-opaque-fail.rs:11:18
   |
   |
LL |         fn f() { my_core::mem::drop(0); }
   |                  ^^^^^^^ use of undeclared crate or module `my_core`
...
LL | a!();
   |
   |
   = note: this error originates in the macro `a` (in Nightly builds, run with -Z macro-backtrace for more info)
error[E0433]: failed to resolve: use of undeclared crate or module `my_core`
  --> /checkout/src/test/ui/hygiene/extern-prelude-from-opaque-fail.rs:24:14
   |
   |
LL |     fn f() { my_core::mem::drop(0); }
   |              ^^^^^^^ use of undeclared crate or module `my_core`
error: aborting due to 4 previous errors

Some errors have detailed explanations: E0432, E0433.
For more information about an error, try `rustc --explain E0432`.
---
diff of stderr:

32   --> $DIR/globs.rs:61:12
33    |
34 LL | n!(f);
+    | ----- in this macro invocation
36 ...
36 ...
37 LL |         n!(f);
38    |            ^ not found in this scope
45   --> $DIR/globs.rs:65:17
46    |
46    |
47 LL | n!(f);
+    | ----- in this macro invocation
49 ...
50 LL |                 f
51    |                 ^ not found in this scope
---
To only update this specific test, also pass `--test-args hygiene/globs.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/hygiene/globs.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/hygiene/globs" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/hygiene/globs/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0425]: cannot find function `f` in this scope
  --> /checkout/src/test/ui/hygiene/globs.rs:22:9
   |
LL |         f(); //~ ERROR cannot find function `f` in this scope
   |         ^ not found in this scope
help: consider importing this function
   |
LL | use foo::f;
   |
   |

error[E0425]: cannot find function `g` in this scope
  --> /checkout/src/test/ui/hygiene/globs.rs:15:5
   |
LL |       g(); //~ ERROR cannot find function `g` in this scope
   |       ^ not found in this scope
LL | /     m! {
LL | /     m! {
LL | |         use bar::*;
LL | |         g();
LL | |         f(); //~ ERROR cannot find function `f` in this scope
LL | |     }
   |
   |
   = note: this error originates in the macro `m` (in Nightly builds, run with -Z macro-backtrace for more info)
   |
   |
LL | use bar::g;

error[E0425]: cannot find function `f` in this scope
  --> /checkout/src/test/ui/hygiene/globs.rs:61:12
   |
   |
LL | n!(f);
...
...
LL |         n!(f); //~ ERROR cannot find function `f` in this scope
   |            ^ not found in this scope
   = note: consider importing this function:
           foo::f
           foo::f
   = note: this error originates in the macro `n` (in Nightly builds, run with -Z macro-backtrace for more info)
error[E0425]: cannot find function `f` in this scope
  --> /checkout/src/test/ui/hygiene/globs.rs:65:17
   |
   |
LL | n!(f);
...
...
LL |                 f //~ ERROR cannot find function `f` in this scope
   |                 ^ not found in this scope
   = note: consider importing this function:
           foo::f
           foo::f
   = note: this error originates in the macro `n` (in Nightly builds, run with -Z macro-backtrace for more info)
error: aborting due to 4 previous errors

For more information about this error, try `rustc --explain E0425`.

---
diff of stderr:

5    |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
6 ...
7 LL | define_std_as_non_existent!();
+    | ----------------------------- in this macro invocation
9    |
10    = note: this error originates in the macro `define_std_as_non_existent` (in Nightly builds, run with -Z macro-backtrace for more info)
11 
---
To only update this specific test, also pass `--test-args imports/extern-prelude-extern-crate-fail.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/imports/extern-prelude-extern-crate-fail.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/imports/extern-prelude-extern-crate-fail" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--extern" "non_existent" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/imports/extern-prelude-extern-crate-fail/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: macro-expanded `extern crate` items cannot shadow names passed with `--extern`
   |
LL |         extern crate std as non_existent;
   |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
...
...
LL | define_std_as_non_existent!();
   |
   = note: this error originates in the macro `define_std_as_non_existent` (in Nightly builds, run with -Z macro-backtrace for more info)


error[E0433]: failed to resolve: use of undeclared crate or module `two_macros`
   |
   |
LL |         two_macros::m!(); //~ ERROR failed to resolve: use of undeclared crate or module `two_macros`
   |         ^^^^^^^^^^ use of undeclared crate or module `two_macros`
error: aborting due to 2 previous errors

For more information about this error, try `rustc --explain E0433`.

---
diff of stderr:

5    |         ^^^^^^^^^^^^^^^^^^^^^^^^^
6 ...
7 LL | define_other_core!();
+    | -------------------- in this macro invocation
9    |
10    = note: this error originates in the macro `define_other_core` (in Nightly builds, run with -Z macro-backtrace for more info)
11 
11 

22    |         ^^^^^^^^^^^^^^^^^^^^^^^^
23 ...
24 LL | define_vec!();
-    | -------------- in this macro invocation
+    | ------------- in this macro invocation
26 note: `Vec` could also refer to the struct defined here
27   --> $SRC_DIR/std/src/prelude/mod.rs:LL:COL


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/imports/extern-prelude-extern-crate-restricted-shadowing/extern-prelude-extern-crate-restricted-shadowing.stderr
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/imports/extern-prelude-extern-crate-restricted-shadowing/extern-prelude-extern-crate-restricted-shadowing.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args imports/extern-prelude-extern-crate-restricted-shadowing.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/imports/extern-prelude-extern-crate-restricted-shadowing.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/imports/extern-prelude-extern-crate-restricted-shadowing" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/imports/extern-prelude-extern-crate-restricted-shadowing/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: macro-expanded `extern crate` items cannot shadow names passed with `--extern`
   |
LL |         extern crate std as core;
   |         ^^^^^^^^^^^^^^^^^^^^^^^^^
...
...
LL | define_other_core!();
   |
   = note: this error originates in the macro `define_other_core` (in Nightly builds, run with -Z macro-backtrace for more info)


error[E0659]: `Vec` is ambiguous (macro-expanded name vs less macro-expanded name from outer scope during import/macro resolution)
   |
   |
LL |         Vec::panic!(); //~ ERROR `Vec` is ambiguous
   |         ^^^ ambiguous name
   |
note: `Vec` could refer to the crate imported here
   |
   |
LL |         extern crate std as Vec;
...
LL | define_vec!();
   | ------------- in this macro invocation
   | ------------- in this macro invocation
note: `Vec` could also refer to the struct defined here
   |
   |
LL |     pub use super::v1::*;
   = note: this error originates in the macro `define_vec` (in Nightly builds, run with -Z macro-backtrace for more info)

error: aborting due to 2 previous errors

---
17 ...
18 LL |   define_exported!();
-    |   ------------------- in this macro invocation
+    |   ------------------ in this macro invocation
20    = note: this error originates in the macro `define_exported` (in Nightly builds, run with -Z macro-backtrace for more info)
21 
22 error: macro-expanded `macro_export` macros from the current crate cannot be referred to by absolute paths
36    | |_____^
37 ...
38 LL |   define_exported!();
-    |   ------------------- in this macro invocation
-    |   ------------------- in this macro invocation
+    |   ------------------ in this macro invocation
40    = note: this error originates in the macro `define_exported` (in Nightly builds, run with -Z macro-backtrace for more info)
42 error: aborting due to 2 previous errors


The actual stderr differed from the expected stderr.
The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/imports/local-modularized-tricky-fail-2/local-modularized-tricky-fail-2.stderr
To only update this specific test, also pass `--test-args imports/local-modularized-tricky-fail-2.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/imports/local-modularized-tricky-fail-2.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/imports/local-modularized-tricky-fail-2" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/imports/local-modularized-tricky-fail-2/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: macro-expanded `macro_export` macros from the current crate cannot be referred to by absolute paths
   |
LL |     use exported;
   |         ^^^^^^^^
   |
   |
   = note: `#[deny(macro_expanded_macro_exports_accessed_by_absolute_paths)]` on by default
   = note: for more information, see issue #52234 <https://github.com/rust-lang/rust/issues/52234>
note: the macro is defined here
  --> /checkout/src/test/ui/imports/local-modularized-tricky-fail-2.rs:5:5
   |
   |
LL | /     macro_rules! exported {
LL | |         () => ()
LL | |     }
...
LL |   define_exported!();
   |   ------------------ in this macro invocation
   |   ------------------ in this macro invocation
   = note: this error originates in the macro `define_exported` (in Nightly builds, run with -Z macro-backtrace for more info)

error: macro-expanded `macro_export` macros from the current crate cannot be referred to by absolute paths
   |
   |
LL |     ::exported!();
   |
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
   = note: for more information, see issue #52234 <https://github.com/rust-lang/rust/issues/52234>
note: the macro is defined here
note: the macro is defined here
  --> /checkout/src/test/ui/imports/local-modularized-tricky-fail-2.rs:5:5
   |
LL | /     macro_rules! exported {
LL | |         () => ()
LL | |     }
...
LL |   define_exported!();
   |   ------------------ in this macro invocation
   |   ------------------ in this macro invocation
   = note: this error originates in the macro `define_exported` (in Nightly builds, run with -Z macro-backtrace for more info)
error: aborting due to 2 previous errors


------------------------------------------
---
14 ...
15 LL |       define_exported!();
-    |       ------------------- in this macro invocation
+    |       ------------------ in this macro invocation
17 note: `exported` could also refer to the macro imported here
19    |

37    | |_____^
38 ...
38 ...
39 LL |       define_exported!();
-    |       ------------------- in this macro invocation
+    |       ------------------ in this macro invocation
41 note: `exported` could also refer to the macro imported here
43    |

62    | |_____^
63 ...
63 ...
64 LL |       define_panic!();
-    |       ---------------- in this macro invocation
+    |       --------------- in this macro invocation
66    = help: use `crate::panic` to refer to this macro unambiguously
67    = note: this error originates in the macro `define_panic` (in Nightly builds, run with -Z macro-backtrace for more info)

82    | |_____^
83 ...
84 LL |       define_include!();
84 LL |       define_include!();
-    |       ------------------ in this macro invocation
+    |       ----------------- in this macro invocation
86    = help: use `crate::include` to refer to this macro unambiguously
87    = note: this error originates in the macro `define_include` (in Nightly builds, run with -Z macro-backtrace for more info)


The actual stderr differed from the expected stderr.
The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/imports/local-modularized-tricky-fail-1/local-modularized-tricky-fail-1.stderr
To only update this specific test, also pass `--test-args imports/local-modularized-tricky-fail-1.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/imports/local-modularized-tricky-fail-1.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/imports/local-modularized-tricky-fail-1" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/imports/local-modularized-tricky-fail-1/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0659]: `exported` is ambiguous (glob import vs macro-expanded name in the same module during import/macro resolution)
   |
   |
LL | exported!(); //~ ERROR `exported` is ambiguous
   | ^^^^^^^^ ambiguous name
   |
note: `exported` could refer to the macro defined here
   |
   |
LL | /     macro_rules! exported {
LL | |         () => ()
LL | |     }
...
LL |       define_exported!();
   |       ------------------ in this macro invocation
   |       ------------------ in this macro invocation
note: `exported` could also refer to the macro imported here
   |
LL | use inner1::*;
   |     ^^^^^^^^^
   |     ^^^^^^^^^
   = help: consider adding an explicit import of `exported` to disambiguate
   = note: this error originates in the macro `define_exported` (in Nightly builds, run with -Z macro-backtrace for more info)

error[E0659]: `exported` is ambiguous (glob import vs macro-expanded name in the same module during import/macro resolution)
   |
   |
LL | exported!(); //~ ERROR `exported` is ambiguous
   | ^^^^^^^^ ambiguous name
   |
note: `exported` could refer to the macro defined here
   |
   |
LL | /     macro_rules! exported {
LL | |         () => ()
LL | |     }
...
LL |       define_exported!();
   |       ------------------ in this macro invocation
   |       ------------------ in this macro invocation
note: `exported` could also refer to the macro imported here
   |
LL | use inner1::*;
   |     ^^^^^^^^^
   |     ^^^^^^^^^
   = help: consider adding an explicit import of `exported` to disambiguate
   = note: this error originates in the macro `define_exported` (in Nightly builds, run with -Z macro-backtrace for more info)

error[E0659]: `panic` is ambiguous (macro-expanded name vs less macro-expanded name from outer scope during import/macro resolution)
   |
   |
LL |     panic!(); //~ ERROR `panic` is ambiguous
   |     ^^^^^ ambiguous name
   |
   = note: `panic` could refer to a macro from prelude
note: `panic` could also refer to the macro defined here
   |
LL | /     macro_rules! panic {
LL | |         () => ()
LL | |     }
LL | |     }
   | |_____^
...
LL |       define_panic!();
   |       --------------- in this macro invocation
   = help: use `crate::panic` to refer to this macro unambiguously
   = note: this error originates in the macro `define_panic` (in Nightly builds, run with -Z macro-backtrace for more info)

error[E0659]: `include` is ambiguous (macro-expanded name vs less macro-expanded name from outer scope during import/macro resolution)
   |
   |
LL | include!(); //~ ERROR `include` is ambiguous
   | ^^^^^^^ ambiguous name
   |
   = note: `include` could refer to a macro from prelude
note: `include` could also refer to the macro defined here
   |
LL | /     macro_rules! include {
LL | |         () => ()
LL | |     }
LL | |     }
   | |_____^
...
LL |       define_include!();
   |       ----------------- in this macro invocation
   = help: use `crate::include` to refer to this macro unambiguously
   = note: this error originates in the macro `define_include` (in Nightly builds, run with -Z macro-backtrace for more info)
error: aborting due to 4 previous errors

For more information about this error, try `rustc --explain E0659`.

---
28 LL |     } }
29 LL |     m!();
-    |     ----- in this macro invocation
+    |     ---- in this macro invocation
31    = note: this error originates in the macro `m` (in Nightly builds, run with -Z macro-backtrace for more info)
32 
33 error[E0659]: `n` is ambiguous (glob import vs any other name from outer scope during import/macro resolution)

The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/imports/shadow_builtin_macros/shadow_builtin_macros.stderr
To update references, rerun the tests and pass the `--bless` flag
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args imports/shadow_builtin_macros.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/imports/shadow_builtin_macros.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/imports/shadow_builtin_macros" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/imports/shadow_builtin_macros/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0659]: `panic` is ambiguous (glob import vs any other name from outer scope during import/macro resolution)
   |
   |
LL |     fn f() { panic!(); } //~ ERROR ambiguous
   |              ^^^^^ ambiguous name
   |
   = note: `panic` could refer to a macro from prelude
note: `panic` could also refer to the macro imported here
   |
LL |     use foo::*;
   |         ^^^^^^
   = help: consider adding an explicit import of `panic` to disambiguate
   = help: consider adding an explicit import of `panic` to disambiguate
   = help: or use `self::panic` to refer to this macro unambiguously

error[E0659]: `panic` is ambiguous (macro-expanded name vs less macro-expanded name from outer scope during import/macro resolution)
   |
   |
LL |     panic!(); //~ ERROR `panic` is ambiguous
   |     ^^^^^ ambiguous name
   |
   = note: `panic` could refer to a macro from prelude
note: `panic` could also refer to the macro defined here
   |
   |
LL |         macro_rules! panic { () => {} }
LL |     } }
LL |     m!();
   |     ---- in this macro invocation
   |     ---- in this macro invocation
   = note: this error originates in the macro `m` (in Nightly builds, run with -Z macro-backtrace for more info)

error[E0659]: `n` is ambiguous (glob import vs any other name from outer scope during import/macro resolution)
   |
   |
LL |     n!(); //~ ERROR ambiguous
   |     ^ ambiguous name
   |
note: `n` could refer to the macro imported here
   |
   |
LL |     use bar::*;
   = help: consider adding an explicit import of `n` to disambiguate
   = help: consider adding an explicit import of `n` to disambiguate
   = help: or use `self::n` to refer to this macro unambiguously
note: `n` could also refer to the macro imported here
   |
LL | #[macro_use(n)]
   |             ^


error[E0659]: `panic` is ambiguous (macro-expanded name vs less macro-expanded name from outer scope during import/macro resolution)
   |
   |
LL |     fn f() { panic!(); } //~ ERROR ambiguous
   |              ^^^^^ ambiguous name
   |
   = note: `panic` could refer to a macro from prelude
note: `panic` could also refer to the macro imported here
   |
   |
LL |     ::two_macros::m!(use foo::panic;);
   |                          ^^^^^^^^^^
   = help: use `self::panic` to refer to this macro unambiguously
error: aborting due to 4 previous errors

For more information about this error, try `rustc --explain E0659`.


------------------------------------------


---- [ui] ui/in-band-lifetimes/elided-lifetimes.rs stdout ----
diff of stderr:

35    |                                    ^^^^^^^^^^- help: indicate the anonymous lifetime: `<'_>`
36 ...
37 LL | autowrapper!(Autowrapped, autowrap_gift, 'a);
+    | -------------------------------------------- in this macro invocation
39    |
39    |
40    = note: this error originates in the macro `autowrapper` (in Nightly builds, run with -Z macro-backtrace for more info)


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/in-band-lifetimes/elided-lifetimes/elided-lifetimes.stderr
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/in-band-lifetimes/elided-lifetimes/elided-lifetimes.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args in-band-lifetimes/elided-lifetimes.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/in-band-lifetimes/elided-lifetimes.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/in-band-lifetimes/elided-lifetimes" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--edition=2018" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/in-band-lifetimes/elided-lifetimes/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: hidden lifetime parameters in types are deprecated
   |
   |
LL | fn foo(x: &Foo) {
   |            ^^^- help: indicate the anonymous lifetime: `<'_>`
note: the lint level is defined here
  --> /checkout/src/test/ui/in-band-lifetimes/elided-lifetimes.rs:5:9
   |
   |
LL | #![deny(elided_lifetimes_in_paths)]


error: hidden lifetime parameters in types are deprecated
   |
   |
LL | fn wrap_gift(gift: &str) -> Wrapped {
   |                             ^^^^^^^- help: indicate the anonymous lifetime: `<'_>`

error: hidden lifetime parameters in types are deprecated
   |
   |
LL | fn wrap_gift_with_bow(gift: &str) -> WrappedWithBow {
   |                                      ^^^^^^^^^^^^^^- help: indicate the anonymous lifetime: `<'_>`

error: hidden lifetime parameters in types are deprecated
   |
   |
LL | fn inspect_matched_set(set: MatchedSet) {
   |                             ^^^^^^^^^^- help: indicate the anonymous lifetimes: `<'_, '_>`

error: hidden lifetime parameters in types are deprecated
   |
   |
LL |         fn $fn_name(gift: &str) -> $type_name {
   |                                    ^^^^^^^^^^- help: indicate the anonymous lifetime: `<'_>`
...
LL | autowrapper!(Autowrapped, autowrap_gift, 'a);
   |
   |
   = note: this error originates in the macro `autowrapper` (in Nightly builds, run with -Z macro-backtrace for more info)

error: hidden lifetime parameters in types are deprecated
   |
   |
LL |     let loyalty: Ref<(u32, char)> = honesty.borrow();
   |                  ^^^^^^^^^^^^^^^^ help: indicate the anonymous lifetime: `Ref<'_, (u32, char)>`

error: hidden lifetime parameters in types are deprecated
   |
   |
LL |         Ref<($($types),*)>
   |         ^^^^^^^^^^^^^^^^^^ help: indicate the anonymous lifetime: `Ref<'_, ($($types),*)>`
...
LL |     let yellow: anytuple_ref_ty!(bool, &str) = laughter.borrow();
   |
   |
   = note: this error originates in the macro `anytuple_ref_ty` (in Nightly builds, run with -Z macro-backtrace for more info)
error: aborting due to 7 previous errors


------------------------------------------
------------------------------------------


---- [ui] ui/issues/issue-32655.rs stdout ----
diff of stderr:

5    |           ^^^^^^^^^^^^
6 ...
7 LL | foo!();
+    | ------ in this macro invocation
9    |
10    = note: this error originates in the macro `foo` (in Nightly builds, run with -Z macro-backtrace for more info)
11 
---
To only update this specific test, also pass `--test-args issues/issue-32655.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues/issue-32655.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-32655" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-32655/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: cannot find attribute `derive_Clone` in this scope
  --> /checkout/src/test/ui/issues/issue-32655.rs:3:11
   |
LL |         #[derive_Clone] //~ ERROR cannot find attribute `derive_Clone` in this scope
...
...
LL | foo!();
   |
   = note: this error originates in the macro `foo` (in Nightly builds, run with -Z macro-backtrace for more info)

error: cannot find attribute `derive_Clone` in this scope
error: cannot find attribute `derive_Clone` in this scope
  --> /checkout/src/test/ui/issues/issue-32655.rs:15:7
   |
LL |     #[derive_Clone] //~ ERROR cannot find attribute `derive_Clone` in this scope

error: aborting due to 2 previous errors


---
diff of stderr:

5    |         ^^^^^^^^^^^^^^^^^^^^^^^^^^
6 ...
7 LL | foo!();
+    | ------ in this macro invocation
9    |
10    = help: add `#![feature(allow_internal_unstable)]` to the crate attributes to enable
11    = note: this error originates in the macro `foo` (in Nightly builds, run with -Z macro-backtrace for more info)
---
To only update this specific test, also pass `--test-args issues/issue-32782.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues/issue-32782.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-32782" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-32782/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0658]: allow_internal_unstable side-steps feature gating and stability checks
   |
   |
LL |         #[allow_internal_unstable] //~ ERROR allow_internal_unstable side-steps
...
...
LL | foo!();
   |
   = help: add `#![feature(allow_internal_unstable)]` to the crate attributes to enable
   = note: this error originates in the macro `foo` (in Nightly builds, run with -Z macro-backtrace for more info)

---
diff of stderr:

7    |                    expected 0 generic arguments
8 ...
9 LL | impl_add!(a b);
+    | -------------- in this macro invocation
11    |
12 note: associated function defined here, with 0 generic parameters
13   --> $DIR/issue-53251.rs:4:8
13   --> $DIR/issue-53251.rs:4:8

25    |                    expected 0 generic arguments
26 ...
27 LL | impl_add!(a b);
+    | -------------- in this macro invocation
29    |
30 note: associated function defined here, with 0 generic parameters
31   --> $DIR/issue-53251.rs:4:8
---
To only update this specific test, also pass `--test-args issues/issue-53251.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues/issue-53251.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-53251" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-53251/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0107]: this associated function takes 0 generic arguments but 1 generic argument was supplied
   |
   |
LL |                 S::f::<i64>();
   |                    ^------- help: remove these generics
   |                    expected 0 generic arguments
...
...
LL | impl_add!(a b);
   |
note: associated function defined here, with 0 generic parameters
  --> /checkout/src/test/ui/issues/issue-53251.rs:4:8
   |
   |
LL |     fn f() {}
   |        ^
   = note: this error originates in the macro `impl_add` (in Nightly builds, run with -Z macro-backtrace for more info)

error[E0107]: this associated function takes 0 generic arguments but 1 generic argument was supplied
   |
   |
LL |                 S::f::<i64>();
   |                    ^------- help: remove these generics
   |                    expected 0 generic arguments
...
...
LL | impl_add!(a b);
   |
note: associated function defined here, with 0 generic parameters
  --> /checkout/src/test/ui/issues/issue-53251.rs:4:8
   |
   |
LL |     fn f() {}
   |        ^
   = note: this error originates in the macro `impl_add` (in Nightly builds, run with -Z macro-backtrace for more info)
error: aborting due to 2 previous errors

For more information about this error, try `rustc --explain E0107`.

---
diff of stderr:

5    |                ^^^^^^^^^^^^^^^^^^^^^
6 ...
7 LL | mod a { foo!(); }
+    |         ------ in this macro invocation
9    |
10 note: the lint level is defined here
11   --> $DIR/lints-in-foreign-macros.rs:4:9
---
To only update this specific test, also pass `--test-args lint/lints-in-foreign-macros.rs`

error: 1 errors occurred comparing output.
status: exit status: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/lint/lints-in-foreign-macros.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/lints-in-foreign-macros" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/lints-in-foreign-macros/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
warning: unused import: `std::string::ToString`
  --> /checkout/src/test/ui/lint/lints-in-foreign-macros.rs:11:16
   |
LL |     () => {use std::string::ToString;} //~ WARN: unused import
...
...
LL | mod a { foo!(); }
   |
note: the lint level is defined here
  --> /checkout/src/test/ui/lint/lints-in-foreign-macros.rs:4:9
   |
   |
LL | #![warn(unused_imports)] //~ missing documentation for the crate [missing_docs]
   = note: this warning originates in the macro `foo` (in Nightly builds, run with -Z macro-backtrace for more info)

warning: unused import: `std::string::ToString`
  --> /checkout/src/test/ui/lint/lints-in-foreign-macros.rs:16:18
  --> /checkout/src/test/ui/lint/lints-in-foreign-macros.rs:16:18
   |
LL | mod c { baz!(use std::string::ToString;); } //~ WARN: unused import

warning: unused import: `std::string::ToString`
  --> /checkout/src/test/ui/lint/lints-in-foreign-macros.rs:17:19
   |
   |
LL | mod d { baz2!(use std::string::ToString;); } //~ WARN: unused import

warning: missing documentation for the crate
  --> /checkout/src/test/ui/lint/lints-in-foreign-macros.rs:4:1
   |
   |
LL | / #![warn(unused_imports)] //~ missing documentation for the crate [missing_docs]
LL | | #![warn(missing_docs)]
LL | |
LL | | #[macro_use]
LL | |
LL | | fn main() {}
   | |____________^
   |
   |
note: the lint level is defined here
  --> /checkout/src/test/ui/lint/lints-in-foreign-macros.rs:5:9
   |
LL | #![warn(missing_docs)]

warning: missing documentation for a function
  --> /checkout/src/test/ui/lint/lints-in-foreign-macros.rs:18:6
   |
   |
LL | baz!(pub fn undocumented() {}); //~ WARN: missing documentation for a function

warning: missing documentation for a function
  --> /checkout/src/test/ui/lint/lints-in-foreign-macros.rs:19:7
   |
   |
LL | baz2!(pub fn undocumented2() {}); //~ WARN: missing documentation for a function

warning: 6 warnings emitted


---
diff of stderr:

126    |                                               ^^^^^^^^^^^^^^^^^^^^^^^^
127 ...
128 LL |     define_empty_struct_with_visibility!(pub, Fluorine);
+    |     ---------------------------------------------------
130    |     |                                    |
130    |     |                                    |
131    |     |                                    help: consider restricting its visibility: `crate`


The actual stderr differed from the expected stderr.
The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/unreachable_pub/unreachable_pub.stderr
To only update this specific test, also pass `--test-args lint/unreachable_pub.rs`

error: 1 errors occurred comparing output.
status: exit status: 0
status: exit status: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/lint/unreachable_pub.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/unreachable_pub" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/unreachable_pub/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
warning: unreachable `pub` item
   |
   |
LL |     pub use std::fmt; //~ WARNING unreachable_pub
   |     ---^^^^^^^^^^^^^^
   |     |
   |     help: consider restricting its visibility: `crate`
note: the lint level is defined here
  --> /checkout/src/test/ui/lint/unreachable_pub.rs:6:9
   |
LL | #![warn(unreachable_pub)]
LL | #![warn(unreachable_pub)]
   |         ^^^^^^^^^^^^^^^
   = help: or consider exporting it for use by other crates

warning: unreachable `pub` item
   |
   |
LL |     pub use std::env::{Args}; // braced-use has different item spans than unbraced
   |     |
   |     |
   |     help: consider restricting its visibility: `crate`
   = help: or consider exporting it for use by other crates


warning: unreachable `pub` item
   |
   |
LL |     pub struct Hydrogen { //~ WARNING unreachable_pub
   |     ---^^^^^^^^^^^^^^^^
   |     |
   |     help: consider restricting its visibility: `crate`
   = help: or consider exporting it for use by other crates


warning: unreachable `pub` field
   |
   |
LL |         pub neutrons: usize, //~ WARNING unreachable_pub
   |         ---^^^^^^^^^^^^^^^^
   |         |
   |         help: consider restricting its visibility: `crate`

warning: unreachable `pub` item
   |
   |
LL |         pub fn count_neutrons(&self) -> usize { self.neutrons } //~ WARNING unreachable_pub
   |         ---^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
   |         |
   |         help: consider restricting its visibility: `crate`

warning: unreachable `pub` item
   |
   |
LL |     pub enum Helium {} //~ WARNING unreachable_pub
   |     ---^^^^^^^^^^^^
   |     |
   |     help: consider restricting its visibility: `crate`
   = help: or consider exporting it for use by other crates


warning: unreachable `pub` item
   |
   |
LL |     pub union Lithium { c1: usize, c2: u8 } //~ WARNING unreachable_pub
   |     ---^^^^^^^^^^^^^^
   |     |
   |     help: consider restricting its visibility: `crate`
   = help: or consider exporting it for use by other crates


warning: unreachable `pub` item
   |
   |
LL |     pub fn beryllium() {} //~ WARNING unreachable_pub
   |     ---^^^^^^^^^^^^^^^
   |     |
   |     help: consider restricting its visibility: `crate`
   = help: or consider exporting it for use by other crates


warning: unreachable `pub` item
   |
   |
LL |     pub trait Boron {} //~ WARNING unreachable_pub
   |     ---^^^^^^^^^^^^
   |     |
   |     help: consider restricting its visibility: `crate`
   = help: or consider exporting it for use by other crates


warning: unreachable `pub` item
   |
   |
LL |     pub const CARBON: usize = 1; //~ WARNING unreachable_pub
   |     ---^^^^^^^^^^^^^^^^^^^^^^^^^
   |     |
   |     help: consider restricting its visibility: `crate`
   = help: or consider exporting it for use by other crates


warning: unreachable `pub` item
   |
   |
LL |     pub static NITROGEN: usize = 2; //~ WARNING unreachable_pub
   |     ---^^^^^^^^^^^^^^^^^^^^^^^^^^^^
   |     |
   |     help: consider restricting its visibility: `crate`
   = help: or consider exporting it for use by other crates


warning: unreachable `pub` item
   |
   |
LL |     pub type Oxygen = bool; //~ WARNING unreachable_pub
   |     ---^^^^^^^^^^^^^^^^^^^^
   |     |
   |     help: consider restricting its visibility: `crate`
   = help: or consider exporting it for use by other crates


warning: unreachable `pub` item
   |
   |
LL |         ($visibility: vis, $name: ident) => { $visibility struct $name {} }
...
...
LL |     define_empty_struct_with_visibility!(pub, Fluorine);
   |     |                                    |
   |     |                                    |
   |     |                                    help: consider restricting its visibility: `crate`
   |
   = help: or consider exporting it for use by other crates
   = help: or consider exporting it for use by other crates
   = note: this warning originates in the macro `define_empty_struct_with_visibility` (in Nightly builds, run with -Z macro-backtrace for more info)

warning: unreachable `pub` item
   |
   |
LL |         pub fn catalyze() -> bool; //~ WARNING unreachable_pub
   |         ---^^^^^^^^^^^^^^^^^^^^^^^
   |         |
   |         help: consider restricting its visibility: `crate`
   = help: or consider exporting it for use by other crates

warning: 14 warnings emitted

---
diff of stderr:

126    |                                               ^^^^^^^^^^^^^^^^^^^^^^^^
127 ...
128 LL |     define_empty_struct_with_visibility!(pub, Fluorine);
+    |     ---------------------------------------------------
130    |     |                                    |
130    |     |                                    |
131    |     |                                    help: consider restricting its visibility: `pub(crate)`


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/unreachable_pub-pub_crate/unreachable_pub-pub_crate.stderr
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/unreachable_pub-pub_crate/unreachable_pub-pub_crate.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args lint/unreachable_pub-pub_crate.rs`

error: 1 errors occurred comparing output.
status: exit status: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/lint/unreachable_pub-pub_crate.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/unreachable_pub-pub_crate" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/unreachable_pub-pub_crate/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
warning: unreachable `pub` item
   |
   |
LL |     pub use std::fmt; //~ WARNING unreachable_pub
   |     ---^^^^^^^^^^^^^^
   |     |
   |     help: consider restricting its visibility: `pub(crate)`
note: the lint level is defined here
  --> /checkout/src/test/ui/lint/unreachable_pub-pub_crate.rs:10:9
   |
LL | #![warn(unreachable_pub)]
LL | #![warn(unreachable_pub)]
   |         ^^^^^^^^^^^^^^^
   = help: or consider exporting it for use by other crates

warning: unreachable `pub` item
   |
   |
LL |     pub use std::env::{Args}; // braced-use has different item spans than unbraced
   |     |
   |     |
   |     help: consider restricting its visibility: `pub(crate)`
   = help: or consider exporting it for use by other crates


warning: unreachable `pub` item
   |
   |
LL |     pub struct Hydrogen { //~ WARNING unreachable_pub
   |     ---^^^^^^^^^^^^^^^^
   |     |
   |     help: consider restricting its visibility: `pub(crate)`
   = help: or consider exporting it for use by other crates
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu


warning: unreachable `pub` field
   |
   |
LL |         pub neutrons: usize, //~ WARNING unreachable_pub
   |         ---^^^^^^^^^^^^^^^^
   |         |
   |         help: consider restricting its visibility: `pub(crate)`

warning: unreachable `pub` item
   |
   |
LL |         pub fn count_neutrons(&self) -> usize { self.neutrons } //~ WARNING unreachable_pub
   |         ---^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
   |         |
   |         help: consider restricting its visibility: `pub(crate)`

warning: unreachable `pub` item
   |
   |
LL |     pub enum Helium {} //~ WARNING unreachable_pub
   |     ---^^^^^^^^^^^^
   |     |
   |     help: consider restricting its visibility: `pub(crate)`
   = help: or consider exporting it for use by other crates


warning: unreachable `pub` item
   |
   |
LL |     pub union Lithium { c1: usize, c2: u8 } //~ WARNING unreachable_pub
   |     ---^^^^^^^^^^^^^^
   |     |
   |     help: consider restricting its visibility: `pub(crate)`
   = help: or consider exporting it for use by other crates


warning: unreachable `pub` item
   |
   |
LL |     pub fn beryllium() {} //~ WARNING unreachable_pub
   |     ---^^^^^^^^^^^^^^^
   |     |
   |     help: consider restricting its visibility: `pub(crate)`
   = help: or consider exporting it for use by other crates


warning: unreachable `pub` item
   |
   |
LL |     pub trait Boron {} //~ WARNING unreachable_pub
   |     ---^^^^^^^^^^^^
   |     |
   |     help: consider restricting its visibility: `pub(crate)`
   = help: or consider exporting it for use by other crates


warning: unreachable `pub` item
   |
   |
LL |     pub const CARBON: usize = 1; //~ WARNING unreachable_pub
   |     ---^^^^^^^^^^^^^^^^^^^^^^^^^
   |     |
   |     help: consider restricting its visibility: `pub(crate)`
   = help: or consider exporting it for use by other crates


warning: unreachable `pub` item
   |
   |
LL |     pub static NITROGEN: usize = 2; //~ WARNING unreachable_pub
   |     ---^^^^^^^^^^^^^^^^^^^^^^^^^^^^
   |     |
   |     help: consider restricting its visibility: `pub(crate)`
   = help: or consider exporting it for use by other crates


warning: unreachable `pub` item
   |
   |
LL |     pub type Oxygen = bool; //~ WARNING unreachable_pub
   |     ---^^^^^^^^^^^^^^^^^^^^
   |     |
   |     help: consider restricting its visibility: `pub(crate)`
   = help: or consider exporting it for use by other crates


warning: unreachable `pub` item
   |
   |
LL |         ($visibility: vis, $name: ident) => { $visibility struct $name {} }
...
...
LL |     define_empty_struct_with_visibility!(pub, Fluorine);
   |     |                                    |
   |     |                                    |
   |     |                                    help: consider restricting its visibility: `pub(crate)`
   |
   = help: or consider exporting it for use by other crates
   = help: or consider exporting it for use by other crates
   = note: this warning originates in the macro `define_empty_struct_with_visibility` (in Nightly builds, run with -Z macro-backtrace for more info)

warning: unreachable `pub` item
   |
   |
LL |         pub fn catalyze() -> bool; //~ WARNING unreachable_pub
   |         ---^^^^^^^^^^^^^^^^^^^^^^^
   |         |
   |         help: consider restricting its visibility: `pub(crate)`
   = help: or consider exporting it for use by other crates

warning: 14 warnings emitted

---
To only update this specific test, also pass `--test-args lint/unused/unused-macro-rules.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/lint/unused/unused-macro-rules.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/unused/unused-macro-rules" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/unused/unused-macro-rules/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: unused macro definition
  --> /checkout/src/test/ui/lint/unused/unused-macro-rules.rs:4:1
   |
LL | / macro_rules! unused { //~ ERROR: unused macro definition
LL | |     () => {};
LL | | }
   |
note: the lint level is defined here
  --> /checkout/src/test/ui/lint/unused/unused-macro-rules.rs:1:9
   |
   |
LL | #![deny(unused_macros)]
   |         ^^^^^^^^^^^^^

error: unused macro definition
  --> /checkout/src/test/ui/lint/unused/unused-macro-rules.rs:11:9
   |
LL | /         macro_rules! m { //~ ERROR: unused macro definition
LL | |             () => {};
LL | |         }
...
LL |   create_macro!();
   |   --------------- in this macro invocation
   |
   |
   = note: this error originates in the macro `create_macro` (in Nightly builds, run with -Z macro-backtrace for more info)

error: unused macro definition
  --> /checkout/src/test/ui/lint/unused/unused-macro-rules.rs:24:5
   |
LL | /     macro_rules! unused { //~ ERROR: unused macro definition
LL | |         () => {};
LL | |     }
   |
note: the lint level is defined here
  --> /checkout/src/test/ui/lint/unused/unused-macro-rules.rs:23:12
   |
---
7 LL |     m!();
-    |     ----- caused by the macro expansion here
+    |     ---- caused by the macro expansion here
9    |
10    = note: the usage of `m!` is likely invalid in foreign item context


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/macros/issue-54441/issue-54441.stderr
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/macros/issue-54441/issue-54441.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args macros/issue-54441.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/macros/issue-54441.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/macros/issue-54441" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/macros/issue-54441/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: macro expansion ignores token `let` and any following
   |
   |
LL |         let //~ ERROR macro expansion ignores token `let` and any following
...
LL |     m!();
   |     ---- caused by the macro expansion here
   |
   |
   = note: the usage of `m!` is likely invalid in foreign item context
error: aborting due to previous error


------------------------------------------
------------------------------------------


---- [ui] ui/macros/issue-78325-inconsistent-resolution.rs stdout ----
diff of stderr:

5    |         ^^^^^^^^^^^^^^^^^^^^^^^^^
6 ...
7 LL | define_other_core!();
+    | -------------------- in this macro invocation
9    |
10    = note: this error originates in the macro `define_other_core` (in Nightly builds, run with -Z macro-backtrace for more info)
11 
---
To only update this specific test, also pass `--test-args macros/issue-78325-inconsistent-resolution.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/macros/issue-78325-inconsistent-resolution.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/macros/issue-78325-inconsistent-resolution" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/macros/issue-78325-inconsistent-resolution/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: macro-expanded `extern crate` items cannot shadow names passed with `--extern`
   |
LL |         extern crate std as core;
   |         ^^^^^^^^^^^^^^^^^^^^^^^^^
...
...
LL | define_other_core!();
   |
   = note: this error originates in the macro `define_other_core` (in Nightly builds, run with -Z macro-backtrace for more info)

error: aborting due to previous error
---
diff of stderr:

8   --> $DIR/macro-local-data-key-priv.rs:4:5
9    |
10 LL |     thread_local!(static baz: f64 = 0.0);
+    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
+    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
12    = note: this error originates in the macro `$crate::__thread_local_inner` (in Nightly builds, run with -Z macro-backtrace for more info)
14 error: aborting due to previous error


The actual stderr differed from the expected stderr.
The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/macros/macro-local-data-key-priv/macro-local-data-key-priv.stderr
To only update this specific test, also pass `--test-args macros/macro-local-data-key-priv.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/macros/macro-local-data-key-priv.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/macros/macro-local-data-key-priv" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/macros/macro-local-data-key-priv/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0603]: constant `baz` is private
   |
   |
LL |     bar::baz.with(|_| ());
   |
   |
note: the constant `baz` is defined here
   |
   |
LL |     thread_local!(static baz: f64 = 0.0);
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
   = note: this error originates in the macro `$crate::__thread_local_inner` (in Nightly builds, run with -Z macro-backtrace for more info)
error: aborting due to previous error

For more information about this error, try `rustc --explain E0603`.

---
diff of stderr:

5    |     ^^^^^^^^^^^^
6 ...
7 LL | m1!();
+    | ----- in this macro invocation
9    |
9    |
10    = note: macro-expanded `#[macro_use]`s may not shadow existing macros (see RFC 1560)
11    = note: this error originates in the macro `m1` (in Nightly builds, run with -Z macro-backtrace for more info)
23    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
24 ...
24 ...
25 LL | m1!();
+    | ----- in this macro invocation
27 note: `foo` could also refer to the macro defined here
28   --> $DIR/macro-shadowing.rs:5:1
29    |
---
To only update this specific test, also pass `--test-args macros/macro-shadowing.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/macros/macro-shadowing.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/macros/macro-shadowing" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/macros/macro-shadowing/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: `macro_two` is already in scope
   |
   |
LL |     #[macro_use] //~ ERROR `macro_two` is already in scope
...
...
LL | m1!();
   |
   |
   = note: macro-expanded `#[macro_use]`s may not shadow existing macros (see RFC 1560)
   = note: this error originates in the macro `m1` (in Nightly builds, run with -Z macro-backtrace for more info)

error[E0659]: `foo` is ambiguous (macro-expanded name vs less macro-expanded name from outer scope during import/macro resolution)
   |
   |
LL | foo!(); //~ ERROR `foo` is ambiguous
   | ^^^ ambiguous name
note: `foo` could refer to the macro defined here
  --> /checkout/src/test/ui/macros/macro-shadowing.rs:10:5
   |
   |
LL |     macro_rules! foo { () => {} }
...
...
LL | m1!();
note: `foo` could also refer to the macro defined here
  --> /checkout/src/test/ui/macros/macro-shadowing.rs:5:1
   |
   |
LL | macro_rules! foo { () => {} }
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
   = note: this error originates in the macro `m1` (in Nightly builds, run with -Z macro-backtrace for more info)
error: aborting due to 2 previous errors

For more information about this error, try `rustc --explain E0659`.

---
diff of stderr:

8    |          ^^^^^^^^ no rules expected this token in macro call
9 ...
10 LL | complex_nonterminal!(enum E {});
+    | ------------------------------- in this macro invocation
12    |
13    = note: this error originates in the macro `complex_nonterminal` (in Nightly builds, run with -Z macro-backtrace for more info)
14 
---
To only update this specific test, also pass `--test-args macros/nonterminal-matching.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/macros/nonterminal-matching.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/macros/nonterminal-matching" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/macros/nonterminal-matching/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: no rules expected the token `enum E { }`
   |
   |
LL |     macro n(a $nt_item b) {
   |     --------------------- when calling this macro
...
LL |     n!(a $nt_item b); //~ ERROR no rules expected the token `enum E { }`
   |          ^^^^^^^^ no rules expected this token in macro call
...
LL | complex_nonterminal!(enum E {});
   |
   = note: this error originates in the macro `complex_nonterminal` (in Nightly builds, run with -Z macro-backtrace for more info)

error: aborting due to previous error
---
diff of stderr:

8   --> $DIR/out-of-order-shadowing.rs:4:1
9    |
10 LL | define_macro!(bar);
+    | ^^^^^^^^^^^^^^^^^^
+    | ^^^^^^^^^^^^^^^^^^
12 note: `bar` could also refer to the macro defined here
14    |


The actual stderr differed from the expected stderr.
The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/macros/out-of-order-shadowing/out-of-order-shadowing.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args macros/out-of-order-shadowing.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/macros/out-of-order-shadowing.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/macros/out-of-order-shadowing" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/macros/out-of-order-shadowing/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0659]: `bar` is ambiguous (macro-expanded name vs less macro-expanded name from outer scope during import/macro resolution)
   |
   |
LL | bar!(); //~ ERROR `bar` is ambiguous
   | ^^^ ambiguous name
   |
note: `bar` could refer to the macro defined here
   |
   |
LL | define_macro!(bar);
   | ^^^^^^^^^^^^^^^^^^
note: `bar` could also refer to the macro defined here
   |
   |
LL | macro_rules! bar { () => {} }
   = note: this error originates in the macro `define_macro` (in Nightly builds, run with -Z macro-backtrace for more info)

error: aborting due to previous error

---
diff of stderr:

5    |             ^ ambiguous name
6 ...
7 LL | include!();
+    | ---------- in this macro invocation
9    |
9    |
10 note: `m` could refer to the macro defined here

14    |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
15 ...
15 ...
16 LL | include!();
+    | ---------- in this macro invocation
+    | ---------- in this macro invocation
18 note: `m` could also refer to the macro defined here
20    |

22    |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^
23 ...
23 ...
24 LL | include!();
+    | ---------- in this macro invocation
+    | ---------- in this macro invocation
26    = note: this error originates in the macro `gen_gen_inner_invoc` (in Nightly builds, run with -Z macro-backtrace for more info)
27 
28 error[E0659]: `m` is ambiguous (macro-expanded name vs less macro-expanded name from outer scope during import/macro resolution)
32    |                                          ^ ambiguous name
33 ...
33 ...
34 LL | include!();
+    | ---------- in this macro invocation
36    |
36    |
37 note: `m` could refer to the macro defined here

41    |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
42 ...
42 ...
43 LL | include!();
+    | ---------- in this macro invocation
+    | ---------- in this macro invocation
45 note: `m` could also refer to the macro defined here
47    |

49    |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^
50 ...
50 ...
51 LL | include!();
+    | ---------- in this macro invocation
+    | ---------- in this macro invocation
53    = note: this error originates in the macro `gen_invoc` (in Nightly builds, run with -Z macro-backtrace for more info)
54 
55 error[E0659]: `m` is ambiguous (macro-expanded name vs less macro-expanded name from outer scope during import/macro resolution)
59    |         ^ ambiguous name
60 ...
60 ...
61 LL | include!();
+    | ---------- in this macro invocation
63    |
63    |
64 note: `m` could refer to the macro defined here

68    |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
69 ...
69 ...
70 LL | include!();
+    | ---------- in this macro invocation
+    | ---------- in this macro invocation
72 note: `m` could also refer to the macro defined here
74    |

76    |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^
77 ...
77 ...
78 LL | include!();
+    | ---------- in this macro invocation
+    | ---------- in this macro invocation
80    = note: this error originates in the macro `include` (in Nightly builds, run with -Z macro-backtrace for more info)
81 
82 error[E0659]: `m` is ambiguous (macro-expanded name vs less macro-expanded name from outer scope during import/macro resolution)
86    |         ^ ambiguous name
87 ...
87 ...
88 LL | include!();
+    | ---------- in this macro invocation
90    |
90    |
91 note: `m` could refer to the macro defined here

95    |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
96 ...
96 ...
97 LL | include!();
+    | ---------- in this macro invocation
+    | ---------- in this macro invocation
99 note: `m` could also refer to the macro defined here
101    |

103    |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
104 ...
104 ...
105 LL | include!();
+    | ---------- in this macro invocation
+    | ---------- in this macro invocation
107    = note: this error originates in the macro `include` (in Nightly builds, run with -Z macro-backtrace for more info)
108 
109 error[E0659]: `m` is ambiguous (macro-expanded name vs less macro-expanded name from outer scope during import/macro resolution)
113    |             ^ ambiguous name
114 ...
114 ...
115 LL | include!();
+    | ---------- in this macro invocation
117    |
117    |
118 note: `m` could refer to the macro defined here

122    |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
123 ...
123 ...
124 LL | include!();
+    | ---------- in this macro invocation
+    | ---------- in this macro invocation
126 note: `m` could also refer to the macro defined here
128    |

130    |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
131 ...
131 ...
132 LL | include!();
+    | ---------- in this macro invocation
+    | ---------- in this macro invocation
134    = note: this error originates in the macro `gen_gen_inner_invoc` (in Nightly builds, run with -Z macro-backtrace for more info)
135 
136 error[E0659]: `m` is ambiguous (macro-expanded name vs less macro-expanded name from outer scope during import/macro resolution)
140    |                                          ^ ambiguous name
141 ...
141 ...
142 LL | include!();
+    | ---------- in this macro invocation
144    |
144    |
145 note: `m` could refer to the macro defined here

149    |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
150 ...
150 ...
151 LL | include!();
+    | ---------- in this macro invocation
+    | ---------- in this macro invocation
153 note: `m` could also refer to the macro defined here
155    |

157    |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
158 ...
158 ...
159 LL | include!();
+    | ---------- in this macro invocation
+    | ---------- in this macro invocation
161    = note: this error originates in the macro `gen_invoc` (in Nightly builds, run with -Z macro-backtrace for more info)
162 
163 error[E0659]: `m` is ambiguous (macro-expanded name vs less macro-expanded name from outer scope during import/macro resolution)
167    |         ^ ambiguous name
168 ...
168 ...
169 LL | include!();
+    | ---------- in this macro invocation
171    |
171    |
172 note: `m` could refer to the macro defined here

176    |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
177 ...
177 ...
178 LL | include!();
+    | ---------- in this macro invocation
+    | ---------- in this macro invocation
180 note: `m` could also refer to the macro defined here
182    |

184    |             ^^^^^^^^^^^^^^^^^^^^^^^^^^^
185 ...
185 ...
186 LL | include!();
+    | ---------- in this macro invocation
+    | ---------- in this macro invocation
188    = note: this error originates in the macro `include` (in Nightly builds, run with -Z macro-backtrace for more info)
189 
190 error[E0659]: `m` is ambiguous (macro-expanded name vs less macro-expanded name from outer scope during import/macro resolution)
194    |                                          ^ ambiguous name
195 ...
195 ...
196 LL | include!();
+    | ---------- in this macro invocation
198    |
198    |
199 note: `m` could refer to the macro defined here

203    |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
204 ...
204 ...
205 LL | include!();
+    | ---------- in this macro invocation
+    | ---------- in this macro invocation
207 note: `m` could also refer to the macro defined here
209    |

211    |             ^^^^^^^^^^^^^^^^^^^^^^^^^^^
212 ...
212 ...
213 LL | include!();
+    | ---------- in this macro invocation
+    | ---------- in this macro invocation
215    = note: this error originates in the macro `gen_invoc` (in Nightly builds, run with -Z macro-backtrace for more info)
217 error: aborting due to 8 previous errors


The actual stderr differed from the expected stderr.
The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/macros/restricted-shadowing-legacy/restricted-shadowing-legacy.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args macros/restricted-shadowing-legacy.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/macros/restricted-shadowing-legacy.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/macros/restricted-shadowing-legacy" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/macros/restricted-shadowing-legacy/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0659]: `m` is ambiguous (macro-expanded name vs less macro-expanded name from outer scope during import/macro resolution)
   |
   |
LL |             m!(); //~ ERROR `m` is ambiguous
   |             ^ ambiguous name
...
LL | include!();
   |
   |
note: `m` could refer to the macro defined here
   |
   |
LL |         macro_rules! m { () => { Right } }
...
...
LL | include!();
   | ---------- in this macro invocation
note: `m` could also refer to the macro defined here
   |
   |
LL |         macro_rules! m { () => {} }
...
...
LL | include!();
   | ---------- in this macro invocation
   = note: this error originates in the macro `gen_gen_inner_invoc` (in Nightly builds, run with -Z macro-backtrace for more info)

error[E0659]: `m` is ambiguous (macro-expanded name vs less macro-expanded name from outer scope during import/macro resolution)
   |
   |
LL |         macro_rules! gen_invoc { () => { m!() } } //~ ERROR `m` is ambiguous
   |                                          ^ ambiguous name
...
LL | include!();
   |
   |
note: `m` could refer to the macro defined here
   |
   |
LL |         macro_rules! m { () => { Right } }
...
...
LL | include!();
   | ---------- in this macro invocation
note: `m` could also refer to the macro defined here
   |
   |
LL |         macro_rules! m { () => {} }
...
...
LL | include!();
   | ---------- in this macro invocation
   = note: this error originates in the macro `gen_invoc` (in Nightly builds, run with -Z macro-backtrace for more info)

error[E0659]: `m` is ambiguous (macro-expanded name vs less macro-expanded name from outer scope during import/macro resolution)
   |
   |
LL |         m!(); //~ ERROR `m` is ambiguous
   |         ^ ambiguous name
...
LL | include!();
   |
   |
note: `m` could refer to the macro defined here
   |
   |
LL |         macro_rules! m { () => { Right } }
...
...
LL | include!();
   | ---------- in this macro invocation
note: `m` could also refer to the macro defined here
   |
   |
LL |         macro_rules! m { () => {} }
...
...
LL | include!();
   | ---------- in this macro invocation
   = note: this error originates in the macro `include` (in Nightly builds, run with -Z macro-backtrace for more info)

error[E0659]: `m` is ambiguous (macro-expanded name vs less macro-expanded name from outer scope during import/macro resolution)
   |
   |
LL |         m!(); //~ ERROR `m` is ambiguous
   |         ^ ambiguous name
...
LL | include!();
   |
   |
note: `m` could refer to the macro defined here
   |
   |
LL |         macro_rules! m { () => { Right } }
...
...
LL | include!();
   | ---------- in this macro invocation
note: `m` could also refer to the macro defined here
   |
   |
LL |         macro_rules! m { () => { Wrong } }
...
...
LL | include!();
   | ---------- in this macro invocation
   = note: this error originates in the macro `include` (in Nightly builds, run with -Z macro-backtrace for more info)

error[E0659]: `m` is ambiguous (macro-expanded name vs less macro-expanded name from outer scope during import/macro resolution)
   |
   |
LL |             m!(); //~ ERROR `m` is ambiguous
   |             ^ ambiguous name
...
LL | include!();
   |
   |
note: `m` could refer to the macro defined here
   |
   |
LL |         macro_rules! m { () => { Right } }
...
...
LL | include!();
   | ---------- in this macro invocation
note: `m` could also refer to the macro defined here
   |
   |
LL |         macro_rules! m { () => { Wrong } }
...
...
LL | include!();
   | ---------- in this macro invocation
   = note: this error originates in the macro `gen_gen_inner_invoc` (in Nightly builds, run with -Z macro-backtrace for more info)

error[E0659]: `m` is ambiguous (macro-expanded name vs less macro-expanded name from outer scope during import/macro resolution)
   |
   |
LL |         macro_rules! gen_invoc { () => { m!() } } //~ ERROR `m` is ambiguous
   |                                          ^ ambiguous name
...
LL | include!();
   |
   |
note: `m` could refer to the macro defined here
   |
   |
LL |         macro_rules! m { () => { Right } }
...
...
LL | include!();
   | ---------- in this macro invocation
note: `m` could also refer to the macro defined here
   |
   |
LL |         macro_rules! m { () => { Wrong } }
...
...
LL | include!();
   | ---------- in this macro invocation
   = note: this error originates in the macro `gen_invoc` (in Nightly builds, run with -Z macro-backtrace for more info)

error[E0659]: `m` is ambiguous (macro-expanded name vs less macro-expanded name from outer scope during import/macro resolution)
   |
   |
LL |         m!(); //~ ERROR `m` is ambiguous
   |         ^ ambiguous name
...
LL | include!();
   |
   |
note: `m` could refer to the macro defined here
   |
   |
LL |         macro_rules! m { () => { Right } }
...
...
LL | include!();
   | ---------- in this macro invocation
note: `m` could also refer to the macro defined here
   |
   |
LL |             macro_rules! m { () => {} }
...
...
LL | include!();
   | ---------- in this macro invocation
   = note: this error originates in the macro `include` (in Nightly builds, run with -Z macro-backtrace for more info)

error[E0659]: `m` is ambiguous (macro-expanded name vs less macro-expanded name from outer scope during import/macro resolution)
   |
   |
LL |         macro_rules! gen_invoc { () => { m!() } } //~ ERROR `m` is ambiguous
   |                                          ^ ambiguous name
...
LL | include!();
   |
   |
note: `m` could refer to the macro defined here
   |
   |
LL |         macro_rules! m { () => { Right } }
...
...
LL | include!();
   | ---------- in this macro invocation
note: `m` could also refer to the macro defined here
   |
   |
LL |             macro_rules! m { () => {} }
...
...
LL | include!();
   | ---------- in this macro invocation
   = note: this error originates in the macro `gen_invoc` (in Nightly builds, run with -Z macro-backtrace for more info)
error: aborting due to 8 previous errors

For more information about this error, try `rustc --explain E0659`.

---
diff of stderr:

5    |                 ^ ambiguous name
6 ...
7 LL | include!();
+    | ---------- in this macro invocation
9    |
9    |
10 note: `m` could refer to the macro defined here

14    |         ^^^^^^^^^^^^^^^^^^^
15 ...
15 ...
16 LL | include!();
+    | ---------- in this macro invocation
+    | ---------- in this macro invocation
18 note: `m` could also refer to the macro defined here
20    |

22    |         ^^^^^^^^^^^^
23 ...
23 ...
24 LL | include!();
+    | ---------- in this macro invocation
+    | ---------- in this macro invocation
26    = note: this error originates in the macro `gen_gen_inner_invoc` (in Nightly builds, run with -Z macro-backtrace for more info)
27 
28 error[E0659]: `m` is ambiguous (macro-expanded name vs less macro-expanded name from outer scope during import/macro resolution)
32    |                                 ^ ambiguous name
33 ...
33 ...
34 LL | include!();
+    | ---------- in this macro invocation
36    |
36    |
37 note: `m` could refer to the macro defined here

41    |         ^^^^^^^^^^^^^^^^^^^
42 ...
42 ...
43 LL | include!();
+    | ---------- in this macro invocation
+    | ---------- in this macro invocation
45 note: `m` could also refer to the macro defined here
47    |

49    |         ^^^^^^^^^^^^
50 ...
50 ...
51 LL | include!();
+    | ---------- in this macro invocation
+    | ---------- in this macro invocation
53    = note: this error originates in the macro `gen_invoc` (in Nightly builds, run with -Z macro-backtrace for more info)
54 
55 error[E0659]: `m` is ambiguous (macro-expanded name vs less macro-expanded name from outer scope during import/macro resolution)
59    |             ^ ambiguous name
60 ...
60 ...
61 LL | include!();
+    | ---------- in this macro invocation
63    |
63    |
64 note: `m` could refer to the macro defined here

68    |         ^^^^^^^^^^^^^^^^^^^
69 ...
69 ...
70 LL | include!();
+    | ---------- in this macro invocation
+    | ---------- in this macro invocation
72 note: `m` could also refer to the macro defined here
74    |

76    |         ^^^^^^^^^^^^
77 ...
77 ...
78 LL | include!();
+    | ---------- in this macro invocation
+    | ---------- in this macro invocation
80    = note: this error originates in the macro `include` (in Nightly builds, run with -Z macro-backtrace for more info)
81 
82 error[E0659]: `m` is ambiguous (macro-expanded name vs less macro-expanded name from outer scope during import/macro resolution)
86    |             ^ ambiguous name
87 ...
87 ...
88 LL | include!();
+    | ---------- in this macro invocation
90    |
90    |
91 note: `m` could refer to the macro defined here

95    |         ^^^^^^^^^^^^^^^^^^^
96 ...
96 ...
97 LL | include!();
+    | ---------- in this macro invocation
+    | ---------- in this macro invocation
99 note: `m` could also refer to the macro defined here
101    |

103    |         ^^^^^^^^^^^^^^^^^^^
104 ...
104 ...
105 LL | include!();
+    | ---------- in this macro invocation
+    | ---------- in this macro invocation
107    = note: this error originates in the macro `include` (in Nightly builds, run with -Z macro-backtrace for more info)
108 
109 error[E0659]: `m` is ambiguous (macro-expanded name vs less macro-expanded name from outer scope during import/macro resolution)
113    |                 ^ ambiguous name
114 ...
114 ...
115 LL | include!();
+    | ---------- in this macro invocation
117    |
117    |
118 note: `m` could refer to the macro defined here

122    |         ^^^^^^^^^^^^^^^^^^^
123 ...
123 ...
124 LL | include!();
+    | ---------- in this macro invocation
+    | ---------- in this macro invocation
126 note: `m` could also refer to the macro defined here
128    |

130    |         ^^^^^^^^^^^^^^^^^^^
131 ...
131 ...
132 LL | include!();
+    | ---------- in this macro invocation
+    | ---------- in this macro invocation
134    = note: this error originates in the macro `gen_gen_inner_invoc` (in Nightly builds, run with -Z macro-backtrace for more info)
135 
136 error[E0659]: `m` is ambiguous (macro-expanded name vs less macro-expanded name from outer scope during import/macro resolution)
140    |                                 ^ ambiguous name
141 ...
141 ...
142 LL | include!();
+    | ---------- in this macro invocation
144    |
144    |
145 note: `m` could refer to the macro defined here

149    |         ^^^^^^^^^^^^^^^^^^^
150 ...
150 ...
151 LL | include!();
+    | ---------- in this macro invocation
+    | ---------- in this macro invocation
153 note: `m` could also refer to the macro defined here
155    |

157    |         ^^^^^^^^^^^^^^^^^^^
158 ...
158 ...
159 LL | include!();
+    | ---------- in this macro invocation
+    | ---------- in this macro invocation
161    = note: this error originates in the macro `gen_invoc` (in Nightly builds, run with -Z macro-backtrace for more info)
163 error: aborting due to 6 previous errors


The actual stderr differed from the expected stderr.
The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/macros/restricted-shadowing-modern/restricted-shadowing-modern.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args macros/restricted-shadowing-modern.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/macros/restricted-shadowing-modern.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/macros/restricted-shadowing-modern" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/macros/restricted-shadowing-modern/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0659]: `m` is ambiguous (macro-expanded name vs less macro-expanded name from outer scope during import/macro resolution)
   |
   |
LL |                 m!(); //~ ERROR `m` is ambiguous
   |                 ^ ambiguous name
...
LL | include!();
   |
   |
note: `m` could refer to the macro defined here
   |
   |
LL |         macro m() { Right }
...
...
LL | include!();
   | ---------- in this macro invocation
note: `m` could also refer to the macro defined here
   |
LL |         macro m() {}
   |         ^^^^^^^^^^^^
...
...
LL | include!();
   | ---------- in this macro invocation
   = note: this error originates in the macro `gen_gen_inner_invoc` (in Nightly builds, run with -Z macro-backtrace for more info)

error[E0659]: `m` is ambiguous (macro-expanded name vs less macro-expanded name from outer scope during import/macro resolution)
   |
   |
LL |             macro gen_invoc() { m!() } //~ ERROR `m` is ambiguous
   |                                 ^ ambiguous name
...
LL | include!();
   |
   |
note: `m` could refer to the macro defined here
   |
   |
LL |         macro m() { Right }
...
...
LL | include!();
   | ---------- in this macro invocation
note: `m` could also refer to the macro defined here
   |
LL |         macro m() {}
   |         ^^^^^^^^^^^^
...
...
LL | include!();
   | ---------- in this macro invocation
   = note: this error originates in the macro `gen_invoc` (in Nightly builds, run with -Z macro-backtrace for more info)

error[E0659]: `m` is ambiguous (macro-expanded name vs less macro-expanded name from outer scope during import/macro resolution)
   |
   |
LL |             m!(); //~ ERROR `m` is ambiguous
   |             ^ ambiguous name
...
LL | include!();
   |
   |
note: `m` could refer to the macro defined here
   |
   |
LL |         macro m() { Right }
...
...
LL | include!();
   | ---------- in this macro invocation
note: `m` could also refer to the macro defined here
   |
LL |         macro m() {}
   |         ^^^^^^^^^^^^
...
...
LL | include!();
   | ---------- in this macro invocation
   = note: this error originates in the macro `include` (in Nightly builds, run with -Z macro-backtrace for more info)

error[E0659]: `m` is ambiguous (macro-expanded name vs less macro-expanded name from outer scope during import/macro resolution)
   |
   |
LL |             m!(); //~ ERROR `m` is ambiguous
   |             ^ ambiguous name
...
LL | include!();
   |
   |
note: `m` could refer to the macro defined here
   |
   |
LL |         macro m() { Right }
...
...
LL | include!();
   | ---------- in this macro invocation
note: `m` could also refer to the macro defined here
   |
   |
LL |         macro m() { Wrong }
...
...
LL | include!();
   | ---------- in this macro invocation
   = note: this error originates in the macro `include` (in Nightly builds, run with -Z macro-backtrace for more info)

error[E0659]: `m` is ambiguous (macro-expanded name vs less macro-expanded name from outer scope during import/macro resolution)
   |
   |
LL |                 m!(); //~ ERROR `m` is ambiguous
   |                 ^ ambiguous name
...
LL | include!();
   |
   |
note: `m` could refer to the macro defined here
   |
   |
LL |         macro m() { Right }
...
...
LL | include!();
   | ---------- in this macro invocation
note: `m` could also refer to the macro defined here
   |
   |
LL |         macro m() { Wrong }
...
...
LL | include!();
   | ---------- in this macro invocation
   = note: this error originates in the macro `gen_gen_inner_invoc` (in Nightly builds, run with -Z macro-backtrace for more info)

error[E0659]: `m` is ambiguous (macro-expanded name vs less macro-expanded name from outer scope during import/macro resolution)
   |
   |
LL |             macro gen_invoc() { m!() } //~ ERROR `m` is ambiguous
   |                                 ^ ambiguous name
...
LL | include!();
   |
   |
note: `m` could refer to the macro defined here
   |
   |
LL |         macro m() { Right }
...
...
LL | include!();
   | ---------- in this macro invocation
note: `m` could also refer to the macro defined here
   |
   |
LL |         macro m() { Wrong }
...
...
LL | include!();
   | ---------- in this macro invocation
   = note: this error originates in the macro `gen_invoc` (in Nightly builds, run with -Z macro-backtrace for more info)
error: aborting due to 6 previous errors

For more information about this error, try `rustc --explain E0659`.

---
diff of stderr:

18   --> $DIR/same-sequence-span.rs:19:1
19    |
20 LL |   proc_macro_sequence::make_foo!();
-    |   ^--------------------------------
+    |   ^-------------------------------
22    |   |
23    |  _in this macro invocation

34   --> $DIR/same-sequence-span.rs:19:1
35    |
35    |
36 LL | proc_macro_sequence::make_foo!();
-    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ not allowed after `expr` fragments
+    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ not allowed after `expr` fragments
38    |
39    = note: allowed there are: `=>`, `,` or `;`
40    = note: this error originates in the macro `proc_macro_sequence::make_foo` (in Nightly builds, run with -Z macro-backtrace for more info)

The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/macros/same-sequence-span/same-sequence-span.stderr
To update references, rerun the tests and pass the `--bless` flag
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args macros/same-sequence-span.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/macros/same-sequence-span.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/macros/same-sequence-span" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/macros/same-sequence-span/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: `$x:expr` may be followed by `$y:tt`, which is not allowed for `expr` fragments
   |
   |
LL |     (1 $x:expr $($y:tt,)*   //~ERROR `$x:expr` may be followed by `$y:tt`
   |                  ^^^^^ not allowed after `expr` fragments
   |
   = note: allowed there are: `=>`, `,` or `;`

error: `$x:expr` may be followed by `=`, which is not allowed for `expr` fragments
   |
   |
LL |                $(= $z:tt)*  //~ERROR `$x:expr` may be followed by `=`
   |                  ^ not allowed after `expr` fragments
   |
   = note: allowed there are: `=>`, `,` or `;`

error: `$x:expr` may be followed by `$y:tt`, which is not allowed for `expr` fragments
   |
   |
LL |   proc_macro_sequence::make_foo!(); //~ERROR `$x:expr` may be followed by `$y:tt`
   |   ^-------------------------------
   |   |
   |  _in this macro invocation
   | |
LL | |                                   //~^ERROR `$x:expr` may be followed by `=`
LL | |
LL | | fn main() {}
   | |_________________________________^ not allowed after `expr` fragments
   |
   = note: allowed there are: `=>`, `,` or `;`
   = note: this error originates in the macro `proc_macro_sequence::make_foo` (in Nightly builds, run with -Z macro-backtrace for more info)

error: `$x:expr` may be followed by `=`, which is not allowed for `expr` fragments
   |
   |
LL | proc_macro_sequence::make_foo!(); //~ERROR `$x:expr` may be followed by `$y:tt`
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ not allowed after `expr` fragments
   |
   = note: allowed there are: `=>`, `,` or `;`
   = note: this error originates in the macro `proc_macro_sequence::make_foo` (in Nightly builds, run with -Z macro-backtrace for more info)
error: aborting due to 4 previous errors


------------------------------------------
------------------------------------------


---- [ui] ui/modules/issue-56411.rs stdout ----
diff of stderr:

10    |                     you can use `as` to change the binding name of the import
11 ...
12 LL | import!(("issue-56411-aux.rs", issue_56411_aux));
+    | ------------------------------------------------ in this macro invocation
14    |
14    |
15    = note: `issue_56411_aux` must be defined only once in the type namespace of this module
16    = note: this error originates in the macro `import` (in Nightly builds, run with -Z macro-backtrace for more info)

22    |                     ^^^^^^^^^^^ re-export of private `issue_56411_aux`
23 ...
24 LL | import!(("issue-56411-aux.rs", issue_56411_aux));
+    | ------------------------------------------------ in this macro invocation
26    |
26    |
27    = note: consider declaring type or module `issue_56411_aux` with `pub`
28    = note: this error originates in the macro `import` (in Nightly builds, run with -Z macro-backtrace for more info)

The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/modules/issue-56411/issue-56411.stderr
To update references, rerun the tests and pass the `--bless` flag
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args modules/issue-56411.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/modules/issue-56411.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/modules/issue-56411" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/modules/issue-56411/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0255]: the name `issue_56411_aux` is defined multiple times
   |
LL |             mod $name;
LL |             mod $name;
   |             ---------- previous definition of the module `issue_56411_aux` here
LL |             pub use self::$name;
   |                     |
   |                     |
   |                     `issue_56411_aux` reimported here
   |                     you can use `as` to change the binding name of the import
...
LL | import!(("issue-56411-aux.rs", issue_56411_aux));
   |
   |
   = note: `issue_56411_aux` must be defined only once in the type namespace of this module
   = note: this error originates in the macro `import` (in Nightly builds, run with -Z macro-backtrace for more info)

error[E0365]: `issue_56411_aux` is private, and cannot be re-exported
   |
   |
LL |             pub use self::$name;
   |                     ^^^^^^^^^^^ re-export of private `issue_56411_aux`
...
LL | import!(("issue-56411-aux.rs", issue_56411_aux));
   |
   |
   = note: consider declaring type or module `issue_56411_aux` with `pub`
   = note: this error originates in the macro `import` (in Nightly builds, run with -Z macro-backtrace for more info)
error: aborting due to 2 previous errors

Some errors have detailed explanations: E0255, E0365.
For more information about an error, try `rustc --explain E0255`.
---
diff of stderr:

13    |         ^^^^^^^^^
14 ...
15 LL |     expand_to_enum!();
+    |     ----------------- in this macro invocation
17    |
17    |
18    = help: consider moving the enum out to a nearby module scope
19    = note: this error originates in the macro `expand_to_enum` (in Nightly builds, run with -Z macro-backtrace for more info)
33    |         ^^^^^^^^^
34 ...
34 ...
35 LL |     expand_to_enum!();
+    |     ----------------- in this macro invocation
37    |
37    |
38    = help: consider moving the enum out to a nearby module scope
39    = note: this error originates in the macro `expand_to_enum` (in Nightly builds, run with -Z macro-backtrace for more info)
53    |         ^^^^^^^^^
54 ...
54 ...
55 LL |     expand_to_enum!();
+    |     ----------------- in this macro invocation
57    |
57    |
58    = help: consider moving the enum out to a nearby module scope
59    = note: this error originates in the macro `expand_to_enum` (in Nightly builds, run with -Z macro-backtrace for more info)

The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/issue-48137-macros-cannot-interpolate-impl-items-bad-variants/issue-48137-macros-cannot-interpolate-impl-items-bad-variants.stderr
To update references, rerun the tests and pass the `--bless` flag
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args parser/issue-48137-macros-cannot-interpolate-impl-items-bad-variants.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/parser/issue-48137-macros-cannot-interpolate-impl-items-bad-variants.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/issue-48137-macros-cannot-interpolate-impl-items-bad-variants" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/issue-48137-macros-cannot-interpolate-impl-items-bad-variants/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: struct is not supported in `trait`s or `impl`s
   |
   |
LL |     struct BadS; //~ ERROR struct is not supported in `trait`s or `impl`s
   |
   |
   = help: consider moving the struct out to a nearby module scope

error: enum is not supported in `trait`s or `impl`s
   |
   |
LL |         enum BadE {}
...
...
LL |     expand_to_enum!();
   |
   |
   = help: consider moving the enum out to a nearby module scope
   = note: this error originates in the macro `expand_to_enum` (in Nightly builds, run with -Z macro-backtrace for more info)

error: struct is not supported in `trait`s or `impl`s
   |
   |
LL |     struct BadS; //~ ERROR struct is not supported in `trait`s or `impl`s
   |
   |
   = help: consider moving the struct out to a nearby module scope

error: enum is not supported in `trait`s or `impl`s
   |
   |
LL |         enum BadE {}
...
...
LL |     expand_to_enum!();
   |
   |
   = help: consider moving the enum out to a nearby module scope
   = note: this error originates in the macro `expand_to_enum` (in Nightly builds, run with -Z macro-backtrace for more info)

error: struct is not supported in `extern` blocks
   |
   |
LL |     struct BadS; //~ ERROR struct is not supported in `extern` blocks
   |
   |
   = help: consider moving the struct out to a nearby module scope

error: enum is not supported in `extern` blocks
   |
   |
LL |         enum BadE {}
...
...
LL |     expand_to_enum!();
   |
   |
   = help: consider moving the enum out to a nearby module scope
   = note: this error originates in the macro `expand_to_enum` (in Nightly builds, run with -Z macro-backtrace for more info)
error: aborting due to 6 previous errors


------------------------------------------
------------------------------------------


---- [ui] ui/parser/issue-73568-lifetime-after-mut.rs stdout ----
diff of stderr:

17    |                      ^^^^^^^^ help: place the lifetime before `mut`: `&$lt mut`
18 ...
19 LL | mac!('a);
+    | -------- in this macro invocation
21    |
21    |
22    = note: this error originates in the macro `mac` (in Nightly builds, run with -Z macro-backtrace for more info)


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/issue-73568-lifetime-after-mut/issue-73568-lifetime-after-mut.stderr
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/issue-73568-lifetime-after-mut/issue-73568-lifetime-after-mut.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args parser/issue-73568-lifetime-after-mut.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/parser/issue-73568-lifetime-after-mut.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/issue-73568-lifetime-after-mut" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/issue-73568-lifetime-after-mut/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: lifetime must precede `mut`
  --> /checkout/src/test/ui/parser/issue-73568-lifetime-after-mut.rs:2:13
   |
LL | fn x<'a>(x: &mut 'a i32){} //~ ERROR lifetime must precede `mut`
   |             ^^^^^^^ help: place the lifetime before `mut`: `&'a mut`

error[E0178]: expected a path on the left-hand side of `+`, not `&mut 'a`
   |
   |
LL | fn y<'a>(y: &mut 'a + Send) {
   |             ^^^^^^^^^^^^^^ help: try adding parentheses: `&mut ('a + Send)`
error: lifetime must precede `mut`
  --> /checkout/src/test/ui/parser/issue-73568-lifetime-after-mut.rs:6:22
   |
   |
LL |         fn w<$lt>(w: &mut $lt i32) {}
   |                      ^^^^^^^^ help: place the lifetime before `mut`: `&$lt mut`
...
LL | mac!('a);
   |
   |
   = note: this error originates in the macro `mac` (in Nightly builds, run with -Z macro-backtrace for more info)
error[E0423]: expected value, found trait `Send`
  --> /checkout/src/test/ui/parser/issue-73568-lifetime-after-mut.rs:19:28
   |
   |
LL |     let z = y as &mut 'a + Send;

warning: trait objects without an explicit `dyn` are deprecated
  --> /checkout/src/test/ui/parser/issue-73568-lifetime-after-mut.rs:14:18
   |
   |
LL | fn y<'a>(y: &mut 'a + Send) {
   |                  ^^ help: use `dyn`: `dyn 'a`
   = note: `#[warn(bare_trait_objects)]` on by default
   = warning: this is accepted in the current edition (Rust 2015) but is a hard error in Rust 2021!
   = note: for more information, see <https://doc.rust-lang.org/nightly/edition-guide/rust-2021/warnings-promoted-to-error.html>


warning: trait objects without an explicit `dyn` are deprecated
  --> /checkout/src/test/ui/parser/issue-73568-lifetime-after-mut.rs:19:23
   |
LL |     let z = y as &mut 'a + Send;
   |                       ^^ help: use `dyn`: `dyn 'a`
   = warning: this is accepted in the current edition (Rust 2015) but is a hard error in Rust 2021!
   = note: for more information, see <https://doc.rust-lang.org/nightly/edition-guide/rust-2021/warnings-promoted-to-error.html>

error[E0224]: at least one trait is required for an object type
error[E0224]: at least one trait is required for an object type
  --> /checkout/src/test/ui/parser/issue-73568-lifetime-after-mut.rs:14:18
   |
LL | fn y<'a>(y: &mut 'a + Send) {

error: aborting due to 5 previous errors; 2 warnings emitted

Some errors have detailed explanations: E0178, E0224, E0423.
---
diff of stderr:

5    |     ^^^ help: remove the visibility
6 ...
7 LL |     pub_x!();
+    |     -------- in this macro invocation
9    |
9    |
10    = help: try adjusting the macro to put `pub` inside the invocation
11    = note: this error originates in the macro `pub_x` (in Nightly builds, run with -Z macro-backtrace for more info)
23    |         ^^^^^^^^^^^^^^^^^^
24 ...
24 ...
25 LL |     pub_x!();
+    |     -------- in this macro invocation
+    |     -------- in this macro invocation
27    = note: this error originates in the macro `priv_x` (in Nightly builds, run with -Z macro-backtrace for more info)
29 error: aborting due to 2 previous errors


The actual stderr differed from the expected stderr.
The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/macro/pub-item-macro/pub-item-macro.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args parser/macro/pub-item-macro.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/parser/macro/pub-item-macro.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/macro/pub-item-macro" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/macro/pub-item-macro/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: can't qualify macro invocation with `pub`
   |
   |
LL |     pub priv_x!(); //~ ERROR can't qualify macro invocation with `pub`
...
...
LL |     pub_x!();
   |
   |
   = help: try adjusting the macro to put `pub` inside the invocation
   = note: this error originates in the macro `pub_x` (in Nightly builds, run with -Z macro-backtrace for more info)

error[E0603]: static `x` is private
   |
   |
LL |     let y: u32 = foo::x; //~ ERROR static `x` is private
   |                       ^ private static
   |
note: the static `x` is defined here
   |
LL |         static x: u32 = 0;
   |         ^^^^^^^^^^^^^^^^^^
...
...
LL |     pub_x!();
   |     -------- in this macro invocation
   = note: this error originates in the macro `priv_x` (in Nightly builds, run with -Z macro-backtrace for more info)
error: aborting due to 2 previous errors

For more information about this error, try `rustc --explain E0603`.

---
diff of stderr:

5    |         ^^
6 ...
7 LL |     bah!(2);
-    |     -------- caused by the macro expansion here
+    |     ------- caused by the macro expansion here
9    |
10    = note: the usage of `bah!` is likely invalid in trait item context


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/macro/trait-non-item-macros/trait-non-item-macros.stderr
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/macro/trait-non-item-macros/trait-non-item-macros.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args parser/macro/trait-non-item-macros.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/parser/macro/trait-non-item-macros.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/macro/trait-non-item-macros" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/macro/trait-non-item-macros/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: macro expansion ignores token `2` and any following
   |
LL |         $a
   |         ^^
...
...
LL |     bah!(2);
   |     ------- caused by the macro expansion here
   |
   = note: the usage of `bah!` is likely invalid in trait item context
error[E0308]: mismatched types
  --> /checkout/src/test/ui/parser/macro/trait-non-item-macros.rs:12:33
   |
   |
LL |     let _recovery_witness: () = 0; //~ ERROR mismatched types
   |                            --   ^ expected `()`, found integer
   |                            expected due to this

error: aborting due to 2 previous errors

---
diff of stderr:

27    |           ^^^^^^^^^^^^
28 ...
29 LL |             gen_helper_use!();
+    |             ----------------- in this macro invocation
31    |
32    = note: consider importing this attribute macro:
33            crate::empty_helper
---
To only update this specific test, also pass `--test-args proc-macro/derive-helper-shadowing.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/proc-macro/derive-helper-shadowing.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/proc-macro/derive-helper-shadowing" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--edition=2018" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/proc-macro/derive-helper-shadowing/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: cannot use a derive helper attribute through an import
  --> /checkout/src/test/ui/proc-macro/derive-helper-shadowing.rs:42:15
   |
LL |             #[renamed] //~ ERROR cannot use a derive helper attribute through an import
   |
note: the derive helper attribute imported here
  --> /checkout/src/test/ui/proc-macro/derive-helper-shadowing.rs:41:17
   |
   |
LL |             use empty_helper as renamed;
   |                 ^^^^^^^^^^^^^^^^^^^^^^^

error: cannot find attribute `empty_helper` in this scope
   |
   |
LL |             #[derive(GenHelperUse)] //~ ERROR cannot find attribute `empty_helper` in this scope
   |
   = note: consider importing this attribute macro:
           empty_helper
           empty_helper
   = note: this error originates in the derive macro `GenHelperUse` (in Nightly builds, run with -Z macro-backtrace for more info)

error: cannot find attribute `empty_helper` in this scope
   |
   |
LL |         #[empty_helper] //~ ERROR cannot find attribute `empty_helper` in this scope
...
...
LL |             gen_helper_use!();
   |
   = note: consider importing this attribute macro:
           crate::empty_helper
           crate::empty_helper
   = note: this error originates in the macro `gen_helper_use` (in Nightly builds, run with -Z macro-backtrace for more info)

error[E0659]: `empty_helper` is ambiguous (name vs any other name during import resolution)
   |
   |
LL |         use empty_helper; //~ ERROR `empty_helper` is ambiguous
   |             ^^^^^^^^^^^^ ambiguous name
   |
note: `empty_helper` could refer to the derive helper attribute defined here
   |
   |
LL | #[derive(Empty)]
   |          ^^^^^
note: `empty_helper` could also refer to the attribute macro imported here
   |
   |
LL | use test_macros::empty_attr as empty_helper;
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
   = help: use `crate::empty_helper` to refer to this attribute macro unambiguously

error[E0659]: `empty_helper` is ambiguous (derive helper attribute vs any other name)
   |
   |
LL | #[empty_helper] //~ ERROR `empty_helper` is ambiguous
   |   ^^^^^^^^^^^^ ambiguous name
   |
note: `empty_helper` could refer to the derive helper attribute defined here
   |
   |
LL | #[derive(Empty)]
   |          ^^^^^
note: `empty_helper` could also refer to the attribute macro imported here
   |
   |
LL | use test_macros::empty_attr as empty_helper;
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
   = help: use `crate::empty_helper` to refer to this attribute macro unambiguously
warning: derive helper attribute is used before it is introduced
  --> /checkout/src/test/ui/proc-macro/derive-helper-shadowing.rs:19:3
   |
   |
LL | #[empty_helper] //~ ERROR `empty_helper` is ambiguous
...
...
LL | #[derive(Empty)]
   |
   = note: `#[warn(legacy_derive_helpers)]` on by default
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
   = note: for more information, see issue #79202 <https://github.com/rust-lang/rust/issues/79202>
---
diff of stderr:

7   ::: $DIR/group-compat-hack.rs:27:5
8    |
9 LL |     impl_macros!(Foo);
+    |     ----------------- in this macro invocation
11    |
12    = note: `#[warn(proc_macro_back_compat)]` on by default
13    = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
13    = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!

24   ::: $DIR/group-compat-hack.rs:44:5
25    |
26 LL |     impl_macros!(Foo);
+    |     ----------------- in this macro invocation
28    |
29    = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
30    = note: for more information, see issue #83125 <https://github.com/rust-lang/rust/issues/83125>
30    = note: for more information, see issue #83125 <https://github.com/rust-lang/rust/issues/83125>

40   ::: $DIR/group-compat-hack.rs:46:5
41    |
42 LL |     arrays!(Foo);
+    |     ------------ in this macro invocation
44    |
45    = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
46    = note: for more information, see issue #83125 <https://github.com/rust-lang/rust/issues/83125>
46    = note: for more information, see issue #83125 <https://github.com/rust-lang/rust/issues/83125>

56   ::: $DIR/group-compat-hack.rs:55:5
57    |
58 LL |     tuple_from_req!(Foo);
+    |     -------------------- in this macro invocation
60    |
61    = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
62    = note: for more information, see issue #83125 <https://github.com/rust-lang/rust/issues/83125>
62    = note: for more information, see issue #83125 <https://github.com/rust-lang/rust/issues/83125>

72   ::: $DIR/group-compat-hack.rs:63:5
73    |
74 LL |     tuple_from_req!(Foo);
+    |     -------------------- in this macro invocation
76    |
77    = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
78    = note: for more information, see issue #83125 <https://github.com/rust-lang/rust/issues/83125>
78    = note: for more information, see issue #83125 <https://github.com/rust-lang/rust/issues/83125>

91   ::: $DIR/group-compat-hack.rs:27:5
92    |
93 LL |     impl_macros!(Foo);
+    |     ----------------- in this macro invocation
95    |
96    = note: `#[warn(proc_macro_back_compat)]` on by default
97    = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
97    = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!

109   ::: $DIR/group-compat-hack.rs:44:5
110    |
111 LL |     impl_macros!(Foo);
+    |     ----------------- in this macro invocation
113    |
114    = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
115    = note: for more information, see issue #83125 <https://github.com/rust-lang/rust/issues/83125>
115    = note: for more information, see issue #83125 <https://github.com/rust-lang/rust/issues/83125>

126   ::: $DIR/group-compat-hack.rs:46:5
127    |
128 LL |     arrays!(Foo);
+    |     ------------ in this macro invocation
130    |
131    = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
132    = note: for more information, see issue #83125 <https://github.com/rust-lang/rust/issues/83125>
132    = note: for more information, see issue #83125 <https://github.com/rust-lang/rust/issues/83125>

143   ::: $DIR/group-compat-hack.rs:55:5
144    |
145 LL |     tuple_from_req!(Foo);
+    |     -------------------- in this macro invocation
147    |
148    = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
149    = note: for more information, see issue #83125 <https://github.com/rust-lang/rust/issues/83125>
149    = note: for more information, see issue #83125 <https://github.com/rust-lang/rust/issues/83125>

160   ::: $DIR/group-compat-hack.rs:63:5
161    |
162 LL |     tuple_from_req!(Foo);
+    |     -------------------- in this macro invocation
164    |
165    = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
166    = note: for more information, see issue #83125 <https://github.com/rust-lang/rust/issues/83125>
---
To only update this specific test, also pass `--test-args proc-macro/group-compat-hack/group-compat-hack.rs`

error: 1 errors occurred comparing output.
status: exit status: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/proc-macro/group-compat-hack/group-compat-hack.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/proc-macro/group-compat-hack/group-compat-hack" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "span-debug" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/proc-macro/group-compat-hack/group-compat-hack/auxiliary"
------------------------------------------
------------------------------------------
Called proc_macro_hack with TokenStream [Ident { ident: "struct", span: /checkout/src/test/ui/proc-macro/group-compat-hack/time-macros-impl/src/lib.rs:5:21: 5:27 (#6) }, Ident { ident: "One", span: /checkout/src/test/ui/proc-macro/group-compat-hack/time-macros-impl/src/lib.rs:5:28: 5:31 (#6) }, Group { delimiter: Parenthesis, stream: TokenStream [Ident { ident: "Foo", span: /checkout/src/test/ui/proc-macro/group-compat-hack/group-compat-hack.rs:27:18: 27:21 (#0) }], span: /checkout/src/test/ui/proc-macro/group-compat-hack/time-macros-impl/src/lib.rs:5:31: 5:38 (#6) }, Punct { ch: ';', spacing: Alone, span: /checkout/src/test/ui/proc-macro/group-compat-hack/time-macros-impl/src/lib.rs:5:38: 5:39 (#6) }]
Called proc_macro_hack with TokenStream [Ident { ident: "struct", span: /checkout/src/test/ui/proc-macro/group-compat-hack/js-sys/src/lib.rs:5:21: 5:27 (#10) }, Ident { ident: "Two", span: /checkout/src/test/ui/proc-macro/group-compat-hack/js-sys/src/lib.rs:5:28: 5:31 (#10) }, Group { delimiter: Parenthesis, stream: TokenStream [Group { delimiter: None, stream: TokenStream [Ident { ident: "Foo", span: /checkout/src/test/ui/proc-macro/group-compat-hack/group-compat-hack.rs:29:13: 29:16 (#0) }], span: /checkout/src/test/ui/proc-macro/group-compat-hack/js-sys/src/lib.rs:5:32: 5:37 (#10) }], span: /checkout/src/test/ui/proc-macro/group-compat-hack/js-sys/src/lib.rs:5:31: 5:38 (#10) }, Punct { ch: ';', spacing: Alone, span: /checkout/src/test/ui/proc-macro/group-compat-hack/js-sys/src/lib.rs:5:38: 5:39 (#10) }]
Called proc_macro_hack with TokenStream [Ident { ident: "struct", span: /checkout/src/test/ui/proc-macro/group-compat-hack/group-compat-hack.rs:22:25: 22:31 (#14) }, Ident { ident: "Three", span: /checkout/src/test/ui/proc-macro/group-compat-hack/group-compat-hack.rs:22:32: 22:37 (#14) }, Group { delimiter: Parenthesis, stream: TokenStream [Group { delimiter: None, stream: TokenStream [Ident { ident: "Foo", span: /checkout/src/test/ui/proc-macro/group-compat-hack/group-compat-hack.rs:30:12: 30:15 (#0) }], span: /checkout/src/test/ui/proc-macro/group-compat-hack/group-compat-hack.rs:22:38: 22:43 (#14) }], span: /checkout/src/test/ui/proc-macro/group-compat-hack/group-compat-hack.rs:22:37: 22:44 (#14) }, Punct { ch: ';', spacing: Alone, span: /checkout/src/test/ui/proc-macro/group-compat-hack/group-compat-hack.rs:22:44: 22:45 (#14) }]
Called proc_macro_hack with TokenStream [Ident { ident: "struct", span: /checkout/src/test/ui/proc-macro/group-compat-hack/time-macros-impl-0.1.0/src/lib.rs:5:21: 5:27 (#20) }, Ident { ident: "One", span: /checkout/src/test/ui/proc-macro/group-compat-hack/time-macros-impl-0.1.0/src/lib.rs:5:28: 5:31 (#20) }, Group { delimiter: Parenthesis, stream: TokenStream [Ident { ident: "Foo", span: /checkout/src/test/ui/proc-macro/group-compat-hack/group-compat-hack.rs:44:18: 44:21 (#0) }], span: /checkout/src/test/ui/proc-macro/group-compat-hack/time-macros-impl-0.1.0/src/lib.rs:5:31: 5:38 (#20) }, Punct { ch: ';', spacing: Alone, span: /checkout/src/test/ui/proc-macro/group-compat-hack/time-macros-impl-0.1.0/src/lib.rs:5:38: 5:39 (#20) }]
Called proc_macro_hack with TokenStream [Ident { ident: "struct", span: /checkout/src/test/ui/proc-macro/group-compat-hack/js-sys-0.3.17/src/lib.rs:5:21: 5:27 (#24) }, Ident { ident: "Two", span: /checkout/src/test/ui/proc-macro/group-compat-hack/js-sys-0.3.17/src/lib.rs:5:28: 5:31 (#24) }, Group { delimiter: Parenthesis, stream: TokenStream [Ident { ident: "Foo", span: /checkout/src/test/ui/proc-macro/group-compat-hack/group-compat-hack.rs:46:13: 46:16 (#0) }], span: /checkout/src/test/ui/proc-macro/group-compat-hack/js-sys-0.3.17/src/lib.rs:5:31: 5:38 (#24) }, Punct { ch: ';', spacing: Alone, span: /checkout/src/test/ui/proc-macro/group-compat-hack/js-sys-0.3.17/src/lib.rs:5:38: 5:39 (#24) }]
Called proc_macro_hack with TokenStream [Ident { ident: "struct", span: /checkout/src/test/ui/proc-macro/group-compat-hack/group-compat-hack.rs:39:25: 39:31 (#28) }, Ident { ident: "Three", span: /checkout/src/test/ui/proc-macro/group-compat-hack/group-compat-hack.rs:39:32: 39:37 (#28) }, Group { delimiter: Parenthesis, stream: TokenStream [Group { delimiter: None, stream: TokenStream [Ident { ident: "Foo", span: /checkout/src/test/ui/proc-macro/group-compat-hack/group-compat-hack.rs:48:12: 48:15 (#0) }], span: /checkout/src/test/ui/proc-macro/group-compat-hack/group-compat-hack.rs:39:38: 39:43 (#28) }], span: /checkout/src/test/ui/proc-macro/group-compat-hack/group-compat-hack.rs:39:37: 39:44 (#28) }, Punct { ch: ';', spacing: Alone, span: /checkout/src/test/ui/proc-macro/group-compat-hack/group-compat-hack.rs:39:44: 39:45 (#28) }]
Called proc_macro_hack with TokenStream [Ident { ident: "struct", span: /checkout/src/test/ui/proc-macro/group-compat-hack/actix-web/src/extract.rs:5:21: 5:27 (#33) }, Ident { ident: "Three", span: /checkout/src/test/ui/proc-macro/group-compat-hack/actix-web/src/extract.rs:5:28: 5:33 (#33) }, Group { delimiter: Parenthesis, stream: TokenStream [Ident { ident: "Foo", span: /checkout/src/test/ui/proc-macro/group-compat-hack/group-compat-hack.rs:55:21: 55:24 (#0) }], span: /checkout/src/test/ui/proc-macro/group-compat-hack/actix-web/src/extract.rs:5:33: 5:37 (#33) }, Punct { ch: ';', spacing: Alone, span: /checkout/src/test/ui/proc-macro/group-compat-hack/actix-web/src/extract.rs:5:37: 5:38 (#33) }]
Called proc_macro_hack with TokenStream [Ident { ident: "struct", span: /checkout/src/test/ui/proc-macro/group-compat-hack/actix-web-2.0.0/src/extract.rs:5:21: 5:27 (#38) }, Ident { ident: "Three", span: /checkout/src/test/ui/proc-macro/group-compat-hack/actix-web-2.0.0/src/extract.rs:5:28: 5:33 (#38) }, Group { delimiter: Parenthesis, stream: TokenStream [Ident { ident: "Foo", span: /checkout/src/test/ui/proc-macro/group-compat-hack/group-compat-hack.rs:63:21: 63:24 (#0) }], span: /checkout/src/test/ui/proc-macro/group-compat-hack/actix-web-2.0.0/src/extract.rs:5:33: 5:37 (#38) }, Punct { ch: ';', spacing: Alone, span: /checkout/src/test/ui/proc-macro/group-compat-hack/actix-web-2.0.0/src/extract.rs:5:37: 5:38 (#38) }]
Called proc_macro_hack with TokenStream [Ident { ident: "struct", span: /checkout/src/test/ui/proc-macro/group-compat-hack/actori-web/src/extract.rs:5:21: 5:27 (#43) }, Ident { ident: "Four", span: /checkout/src/test/ui/proc-macro/group-compat-hack/actori-web/src/extract.rs:5:28: 5:32 (#43) }, Group { delimiter: Parenthesis, stream: TokenStream [Group { delimiter: None, stream: TokenStream [Ident { ident: "Foo", span: /checkout/src/test/ui/proc-macro/group-compat-hack/group-compat-hack.rs:71:21: 71:24 (#0) }], span: /checkout/src/test/ui/proc-macro/group-compat-hack/actori-web/src/extract.rs:5:33: 5:35 (#43) }], span: /checkout/src/test/ui/proc-macro/group-compat-hack/actori-web/src/extract.rs:5:32: 5:36 (#43) }, Punct { ch: ';', spacing: Alone, span: /checkout/src/test/ui/proc-macro/group-compat-hack/actori-web/src/extract.rs:5:36: 5:37 (#43) }]
Called proc_macro_hack with TokenStream [Ident { ident: "struct", span: /checkout/src/test/ui/proc-macro/group-compat-hack/actori-web-2.0.0/src/extract.rs:5:21: 5:27 (#48) }, Ident { ident: "Four", span: /checkout/src/test/ui/proc-macro/group-compat-hack/actori-web-2.0.0/src/extract.rs:5:28: 5:32 (#48) }, Group { delimiter: Parenthesis, stream: TokenStream [Group { delimiter: None, stream: TokenStream [Ident { ident: "Foo", span: /checkout/src/test/ui/proc-macro/group-compat-hack/group-compat-hack.rs:78:21: 78:24 (#0) }], span: /checkout/src/test/ui/proc-macro/group-compat-hack/actori-web-2.0.0/src/extract.rs:5:33: 5:35 (#48) }], span: /checkout/src/test/ui/proc-macro/group-compat-hack/actori-web-2.0.0/src/extract.rs:5:32: 5:36 (#48) }, Punct { ch: ';', spacing: Alone, span: /checkout/src/test/ui/proc-macro/group-compat-hack/actori-web-2.0.0/src/extract.rs:5:36: 5:37 (#48) }]
Called proc_macro_hack with TokenStream [Ident { ident: "struct", span: /checkout/src/test/ui/proc-macro/group-compat-hack/js-sys-0.3.40/src/lib.rs:5:21: 5:27 (#53) }, Ident { ident: "Two", span: /checkout/src/test/ui/proc-macro/group-compat-hack/js-sys-0.3.40/src/lib.rs:5:28: 5:31 (#53) }, Group { delimiter: Parenthesis, stream: TokenStream [Group { delimiter: None, stream: TokenStream [Ident { ident: "Foo", span: /checkout/src/test/ui/proc-macro/group-compat-hack/group-compat-hack.rs:84:13: 84:16 (#0) }], span: /checkout/src/test/ui/proc-macro/group-compat-hack/js-sys-0.3.40/src/lib.rs:5:32: 5:37 (#53) }], span: /checkout/src/test/ui/proc-macro/group-compat-hack/js-sys-0.3.40/src/lib.rs:5:31: 5:38 (#53) }, Punct { ch: ';', spacing: Alone, span: /checkout/src/test/ui/proc-macro/group-compat-hack/js-sys-0.3.40/src/lib.rs:5:38: 5:39 (#53) }]
------------------------------------------
stderr:
------------------------------------------
warning: using an old version of `time-macros-impl`
warning: using an old version of `time-macros-impl`
  --> /checkout/src/test/ui/proc-macro/group-compat-hack/time-macros-impl/src/lib.rs:5:32
   |
LL |         #[my_macro] struct One($name);
   |
  ::: /checkout/src/test/ui/proc-macro/group-compat-hack/group-compat-hack.rs:27:5
   |
   |
LL |     impl_macros!(Foo); //~ WARN  using an old version
   |
   = note: `#[warn(proc_macro_back_compat)]` on by default
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
   = note: for more information, see issue #83125 <https://github.com/rust-lang/rust/issues/83125>
   = note: for more information, see issue #83125 <https://github.com/rust-lang/rust/issues/83125>
   = note: the `time-macros-impl` crate will stop compiling in futures version of Rust. Please update to the latest version of the `time` crate to avoid breakage
   = note: this warning originates in the macro `impl_macros` (in Nightly builds, run with -Z macro-backtrace for more info)
warning: using an old version of `time-macros-impl`
  --> /checkout/src/test/ui/proc-macro/group-compat-hack/time-macros-impl-0.1.0/src/lib.rs:5:32
   |
   |
LL |         #[my_macro] struct One($name);
   |
  ::: /checkout/src/test/ui/proc-macro/group-compat-hack/group-compat-hack.rs:44:5
   |
   |
LL |     impl_macros!(Foo); //~  WARN using an old version
   |
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
   = note: for more information, see issue #83125 <https://github.com/rust-lang/rust/issues/83125>
   = note: for more information, see issue #83125 <https://github.com/rust-lang/rust/issues/83125>
   = note: the `time-macros-impl` crate will stop compiling in futures version of Rust. Please update to the latest version of the `time` crate to avoid breakage
   = note: this warning originates in the macro `impl_macros` (in Nightly builds, run with -Z macro-backtrace for more info)

warning: using an old version of `js-sys`
   |
   |
LL |         #[my_macro] struct Two($name);
   |
  ::: /checkout/src/test/ui/proc-macro/group-compat-hack/group-compat-hack.rs:46:5
   |
   |
LL |     arrays!(Foo); //~  WARN using an old version
   |
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
   = note: for more information, see issue #83125 <https://github.com/rust-lang/rust/issues/83125>
   = note: for more information, see issue #83125 <https://github.com/rust-lang/rust/issues/83125>
   = note: older versions of the `js-sys` crate will stop compiling in future versions of Rust; please update to `js-sys` v0.3.40 or above
   = note: this warning originates in the macro `arrays` (in Nightly builds, run with -Z macro-backtrace for more info)
warning: using an old version of `actix-web`
  --> /checkout/src/test/ui/proc-macro/group-compat-hack/actix-web/src/extract.rs:5:34
   |
   |
LL |         #[my_macro] struct Three($T);
   |
  ::: /checkout/src/test/ui/proc-macro/group-compat-hack/group-compat-hack.rs:55:5
   |
   |
LL |     tuple_from_req!(Foo); //~ WARN using an old version
   |
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
   = note: for more information, see issue #83125 <https://github.com/rust-lang/rust/issues/83125>
   = note: for more information, see issue #83125 <https://github.com/rust-lang/rust/issues/83125>
   = note: the version of `actix-web` you are using might stop compiling in future versions of Rust; please update to the latest version of the `actix-web` crate to avoid breakage
   = note: this warning originates in the macro `tuple_from_req` (in Nightly builds, run with -Z macro-backtrace for more info)
warning: using an old version of `actix-web`
  --> /checkout/src/test/ui/proc-macro/group-compat-hack/actix-web-2.0.0/src/extract.rs:5:34
   |
   |
LL |         #[my_macro] struct Three($T);
   |
  ::: /checkout/src/test/ui/proc-macro/group-compat-hack/group-compat-hack.rs:63:5
   |
   |
LL |     tuple_from_req!(Foo); //~ WARN using an old version
   |
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
   = note: for more information, see issue #83125 <https://github.com/rust-lang/rust/issues/83125>
   = note: for more information, see issue #83125 <https://github.com/rust-lang/rust/issues/83125>
   = note: the version of `actix-web` you are using might stop compiling in future versions of Rust; please update to the latest version of the `actix-web` crate to avoid breakage
   = note: this warning originates in the macro `tuple_from_req` (in Nightly builds, run with -Z macro-backtrace for more info)
warning: 5 warnings emitted


Future incompatibility report: Future breakage diagnostic:
warning: using an old version of `time-macros-impl`
   |
   |
LL |         #[my_macro] struct One($name);
   |
  ::: /checkout/src/test/ui/proc-macro/group-compat-hack/group-compat-hack.rs:27:5
   |
   |
LL |     impl_macros!(Foo); //~ WARN  using an old version
   |
   = note: `#[warn(proc_macro_back_compat)]` on by default
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
   = note: for more information, see issue #83125 <https://github.com/rust-lang/rust/issues/83125>
   = note: for more information, see issue #83125 <https://github.com/rust-lang/rust/issues/83125>
   = note: the `time-macros-impl` crate will stop compiling in futures version of Rust. Please update to the latest version of the `time` crate to avoid breakage
   = note: this warning originates in the macro `impl_macros` (in Nightly builds, run with -Z macro-backtrace for more info)
Future breakage diagnostic:
warning: using an old version of `time-macros-impl`
  --> /checkout/src/test/ui/proc-macro/group-compat-hack/time-macros-impl-0.1.0/src/lib.rs:5:32
   |
   |
LL |         #[my_macro] struct One($name);
   |
  ::: /checkout/src/test/ui/proc-macro/group-compat-hack/group-compat-hack.rs:44:5
   |
   |
LL |     impl_macros!(Foo); //~  WARN using an old version
   |
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
   = note: for more information, see issue #83125 <https://github.com/rust-lang/rust/issues/83125>
   = note: for more information, see issue #83125 <https://github.com/rust-lang/rust/issues/83125>
   = note: the `time-macros-impl` crate will stop compiling in futures version of Rust. Please update to the latest version of the `time` crate to avoid breakage
   = note: this warning originates in the macro `impl_macros` (in Nightly builds, run with -Z macro-backtrace for more info)
Future breakage diagnostic:
Future breakage diagnostic:
warning: using an old version of `js-sys`
   |
   |
LL |         #[my_macro] struct Two($name);
   |
  ::: /checkout/src/test/ui/proc-macro/group-compat-hack/group-compat-hack.rs:46:5
   |
   |
LL |     arrays!(Foo); //~  WARN using an old version
   |
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
   = note: for more information, see issue #83125 <https://github.com/rust-lang/rust/issues/83125>
   = note: for more information, see issue #83125 <https://github.com/rust-lang/rust/issues/83125>
   = note: older versions of the `js-sys` crate will stop compiling in future versions of Rust; please update to `js-sys` v0.3.40 or above
   = note: this warning originates in the macro `arrays` (in Nightly builds, run with -Z macro-backtrace for more info)
Future breakage diagnostic:
warning: using an old version of `actix-web`
  --> /checkout/src/test/ui/proc-macro/group-compat-hack/actix-web/src/extract.rs:5:34
   |
   |
LL |         #[my_macro] struct Three($T);
   |
  ::: /checkout/src/test/ui/proc-macro/group-compat-hack/group-compat-hack.rs:55:5
   |
   |
LL |     tuple_from_req!(Foo); //~ WARN using an old version
   |
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
   = note: for more information, see issue #83125 <https://github.com/rust-lang/rust/issues/83125>
   = note: for more information, see issue #83125 <https://github.com/rust-lang/rust/issues/83125>
   = note: the version of `actix-web` you are using might stop compiling in future versions of Rust; please update to the latest version of the `actix-web` crate to avoid breakage
   = note: this warning originates in the macro `tuple_from_req` (in Nightly builds, run with -Z macro-backtrace for more info)
Future breakage diagnostic:
warning: using an old version of `actix-web`
  --> /checkout/src/test/ui/proc-macro/group-compat-hack/actix-web-2.0.0/src/extract.rs:5:34
   |
   |
LL |         #[my_macro] struct Three($T);
   |
  ::: /checkout/src/test/ui/proc-macro/group-compat-hack/group-compat-hack.rs:63:5
   |
   |
LL |     tuple_from_req!(Foo); //~ WARN using an old version
   |
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
   = note: for more information, see issue #83125 <https://github.com/rust-lang/rust/issues/83125>
   = note: for more information, see issue #83125 <https://github.com/rust-lang/rust/issues/83125>
   = note: the version of `actix-web` you are using might stop compiling in future versions of Rust; please update to the latest version of the `actix-web` crate to avoid breakage
   = note: this warning originates in the macro `tuple_from_req` (in Nightly builds, run with -Z macro-backtrace for more info)

------------------------------------------



---- [ui] ui/proc-macro/invalid-punct-ident-2.rs stdout ----
diff of stderr:

2   --> $DIR/invalid-punct-ident-2.rs:19:1
3    |
4 LL | invalid_ident!();
+    | ^^^^^^^^^^^^^^^^
6    |
6    |
7    = help: message: `"*"` is not a valid identifier


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/proc-macro/invalid-punct-ident-2/invalid-punct-ident-2.stderr
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/proc-macro/invalid-punct-ident-2/invalid-punct-ident-2.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args proc-macro/invalid-punct-ident-2.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/proc-macro/invalid-punct-ident-2.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/proc-macro/invalid-punct-ident-2" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/proc-macro/invalid-punct-ident-2/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: proc macro panicked
  --> /checkout/src/test/ui/proc-macro/invalid-punct-ident-2.rs:19:1
   |
LL | invalid_ident!(); //~ ERROR proc macro panicked
   |
   |
   = help: message: `"*"` is not a valid identifier
error: aborting due to previous error


------------------------------------------
------------------------------------------


---- [ui] ui/proc-macro/invalid-punct-ident-1.rs stdout ----
diff of stderr:

2   --> $DIR/invalid-punct-ident-1.rs:19:1
3    |
4 LL | invalid_punct!();
+    | ^^^^^^^^^^^^^^^^
6    |
6    |
7    = help: message: unsupported character `'`'`


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/proc-macro/invalid-punct-ident-1/invalid-punct-ident-1.stderr
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/proc-macro/invalid-punct-ident-1/invalid-punct-ident-1.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args proc-macro/invalid-punct-ident-1.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/proc-macro/invalid-punct-ident-1.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/proc-macro/invalid-punct-ident-1" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/proc-macro/invalid-punct-ident-1/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: proc macro panicked
  --> /checkout/src/test/ui/proc-macro/invalid-punct-ident-1.rs:19:1
   |
LL | invalid_punct!(); //~ ERROR proc macro panicked
   |
   |
   = help: message: unsupported character `'`'`
error: aborting due to previous error


------------------------------------------
------------------------------------------


---- [ui] ui/proc-macro/invalid-punct-ident-3.rs stdout ----
diff of stderr:

2   --> $DIR/invalid-punct-ident-3.rs:19:1
3    |
4 LL | invalid_raw_ident!();
+    | ^^^^^^^^^^^^^^^^^^^^
6    |
6    |
7    = help: message: `self` cannot be a raw identifier


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/proc-macro/invalid-punct-ident-3/invalid-punct-ident-3.stderr
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/proc-macro/invalid-punct-ident-3/invalid-punct-ident-3.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args proc-macro/invalid-punct-ident-3.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/proc-macro/invalid-punct-ident-3.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/proc-macro/invalid-punct-ident-3" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/proc-macro/invalid-punct-ident-3/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: proc macro panicked
  --> /checkout/src/test/ui/proc-macro/invalid-punct-ident-3.rs:19:1
   |
LL | invalid_raw_ident!(); //~ ERROR proc macro panicked
   |
   |
   = help: message: `self` cannot be a raw identifier
error: aborting due to previous error


------------------------------------------
------------------------------------------


---- [ui] ui/proc-macro/invalid-punct-ident-4.rs stdout ----
diff of stderr:

2   --> $DIR/invalid-punct-ident-4.rs:6:1
3    |
4 LL | lexer_failure!();
+    | ^^^^^^^^^^^^^^^^ unexpected closing delimiter
6    |
6    |
7    = note: this error originates in the macro `lexer_failure` (in Nightly builds, run with -Z macro-backtrace for more info)

10   --> $DIR/invalid-punct-ident-4.rs:6:1
11    |
11    |
12 LL | lexer_failure!();
+    | ^^^^^^^^^^^^^^^^
14 
15 error[E0308]: mismatched types
16   --> $DIR/invalid-punct-ident-4.rs:11:33
---
To only update this specific test, also pass `--test-args proc-macro/invalid-punct-ident-4.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/proc-macro/invalid-punct-ident-4.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/proc-macro/invalid-punct-ident-4" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/proc-macro/invalid-punct-ident-4/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: unexpected closing delimiter: `)`
  --> /checkout/src/test/ui/proc-macro/invalid-punct-ident-4.rs:6:1
   |
LL | lexer_failure!();
   |
   |
   = note: this error originates in the macro `lexer_failure` (in Nightly builds, run with -Z macro-backtrace for more info)
error: proc macro panicked
  --> /checkout/src/test/ui/proc-macro/invalid-punct-ident-4.rs:6:1
   |
   |
LL | lexer_failure!();

error[E0308]: mismatched types
  --> /checkout/src/test/ui/proc-macro/invalid-punct-ident-4.rs:11:33
   |
   |
LL |     let _recovery_witness: () = 0; //~ ERROR mismatched types
   |                            --   ^ expected `()`, found integer
   |                            expected due to this

error: aborting due to 3 previous errors

---
diff of stderr:

2   --> $DIR/issue-83510.rs:5:1
3    |
4 LL | issue_83510::dance_like_you_want_to_ice!();
+    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ not found in this scope
6    |
6    |
7    = note: this error originates in the macro `issue_83510::dance_like_you_want_to_ice` (in Nightly builds, run with -Z macro-backtrace for more info)

10   --> $DIR/issue-83510.rs:5:1
11    |
11    |
12 LL | issue_83510::dance_like_you_want_to_ice!();
+    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ not a trait
14    |
14    |
15    = note: this error originates in the macro `issue_83510::dance_like_you_want_to_ice` (in Nightly builds, run with -Z macro-backtrace for more info)

18   --> $DIR/issue-83510.rs:5:1
19    |
19    |
20 LL | issue_83510::dance_like_you_want_to_ice!();
+    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ not found in this scope
22    |
22    |
23    = note: this error originates in the macro `issue_83510::dance_like_you_want_to_ice` (in Nightly builds, run with -Z macro-backtrace for more info)

26   --> $DIR/issue-83510.rs:5:1
27    |
27    |
28 LL | issue_83510::dance_like_you_want_to_ice!();
+    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
30    |
31    = note: see issue #8995 <https://github.com/rust-lang/rust/issues/8995> for more information
32    = help: add `#![feature(inherent_associated_types)]` to the crate attributes to enable
---
To only update this specific test, also pass `--test-args proc-macro/issue-83510.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/proc-macro/issue-83510.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/proc-macro/issue-83510" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/proc-macro/issue-83510/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0412]: cannot find type `Foo` in this scope
  --> /checkout/src/test/ui/proc-macro/issue-83510.rs:5:1
   |
LL | issue_83510::dance_like_you_want_to_ice!();
   |
   |
   = note: this error originates in the macro `issue_83510::dance_like_you_want_to_ice` (in Nightly builds, run with -Z macro-backtrace for more info)
error[E0404]: expected trait, found struct `Box`
  --> /checkout/src/test/ui/proc-macro/issue-83510.rs:5:1
   |
   |
LL | issue_83510::dance_like_you_want_to_ice!();
   |
   |
   = note: this error originates in the macro `issue_83510::dance_like_you_want_to_ice` (in Nightly builds, run with -Z macro-backtrace for more info)

error[E0405]: cannot find trait `Baz` in this scope
   |
   |
LL | issue_83510::dance_like_you_want_to_ice!();
   |
   |
   = note: this error originates in the macro `issue_83510::dance_like_you_want_to_ice` (in Nightly builds, run with -Z macro-backtrace for more info)
error[E0658]: inherent associated types are unstable
  --> /checkout/src/test/ui/proc-macro/issue-83510.rs:5:1
   |
   |
LL | issue_83510::dance_like_you_want_to_ice!();
   |
   = note: see issue #8995 <https://github.com/rust-lang/rust/issues/8995> for more information
   = help: add `#![feature(inherent_associated_types)]` to the crate attributes to enable
   = help: add `#![feature(inherent_associated_types)]` to the crate attributes to enable
   = note: this error originates in the macro `issue_83510::dance_like_you_want_to_ice` (in Nightly builds, run with -Z macro-backtrace for more info)
error: aborting due to 4 previous errors

Some errors have detailed explanations: E0404, E0405, E0412, E0658.
For more information about an error, try `rustc --explain E0404`.
---
diff of stderr:

5    |                    ^^^^^^^^^^^ not found in this scope
6 ...
7 LL | produce_it!(MyName);
+    | ------------------- in this macro invocation
9    |
9    |
10    = note: this error originates in the macro `produce_it` (in Nightly builds, run with -Z macro-backtrace for more info)


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/proc-macro/macro-rules-derive/macro-rules-derive.stderr
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/proc-macro/macro-rules-derive/macro-rules-derive.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args proc-macro/macro-rules-derive.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/proc-macro/macro-rules-derive.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/proc-macro/macro-rules-derive" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/proc-macro/macro-rules-derive/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0412]: cannot find type `MissingType` in this scope
  --> /checkout/src/test/ui/proc-macro/macro-rules-derive.rs:10:20
   |
LL |             field: MissingType //~ ERROR cannot find type
...
...
LL | produce_it!(MyName);
   |
   |
   = note: this error originates in the macro `produce_it` (in Nightly builds, run with -Z macro-backtrace for more info)
error: aborting due to previous error

For more information about this error, try `rustc --explain E0412`.

---
diff of stderr:

2   --> $DIR/subspan.rs:11:1
3    |
4 LL | subspan!("hi");
+    | ^^^^^^^^^^^^^^
6    |
7 note: here
8   --> $DIR/subspan.rs:11:11
8   --> $DIR/subspan.rs:11:11

15   --> $DIR/subspan.rs:14:1
16    |
17 LL | subspan!("hihi");
+    | ^^^^^^^^^^^^^^^^
19    |
20 note: here
21   --> $DIR/subspan.rs:14:11
21   --> $DIR/subspan.rs:14:11

28   --> $DIR/subspan.rs:17:1
29    |
30 LL | subspan!("hihihi");
+    | ^^^^^^^^^^^^^^^^^^
32    |
33 note: here
34   --> $DIR/subspan.rs:17:11
34   --> $DIR/subspan.rs:17:11

41   --> $DIR/subspan.rs:20:1
42    |
43 LL | subspan!("why I hide? hi!");
+    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^
45    |
46 note: here
47   --> $DIR/subspan.rs:20:17
47   --> $DIR/subspan.rs:20:17

54   --> $DIR/subspan.rs:21:1
55    |
56 LL | subspan!("hey, hi, hidy, hidy, hi hi");
+    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
58    |
59 note: here
60   --> $DIR/subspan.rs:21:16
60   --> $DIR/subspan.rs:21:16

67   --> $DIR/subspan.rs:22:1
68    |
69 LL | subspan!("this is a hi, and this is another hi");
+    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
71    |
72 note: here
73   --> $DIR/subspan.rs:22:12
73   --> $DIR/subspan.rs:22:12

80   --> $DIR/subspan.rs:23:1
81    |
82 LL | subspan!("how are you this evening");
+    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
84    |
85 note: here
86   --> $DIR/subspan.rs:23:24
86   --> $DIR/subspan.rs:23:24

93   --> $DIR/subspan.rs:24:1
94    |
95 LL | subspan!("this is highly eradic");
+    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
97    |
98 note: here
99   --> $DIR/subspan.rs:24:12
---
To only update this specific test, also pass `--test-args proc-macro/subspan.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/proc-macro/subspan.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/proc-macro/subspan" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/proc-macro/subspan/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: found 'hi's
   |
   |
LL | subspan!("hi"); //~ ERROR found 'hi's
   |
note: here
  --> /checkout/src/test/ui/proc-macro/subspan.rs:11:11
   |
   |
LL | subspan!("hi"); //~ ERROR found 'hi's
   |           ^^
   = note: this error originates in the macro `subspan` (in Nightly builds, run with -Z macro-backtrace for more info)

error: found 'hi's
   |
   |
LL | subspan!("hihi"); //~ ERROR found 'hi's
   |
note: here
  --> /checkout/src/test/ui/proc-macro/subspan.rs:14:11
   |
   |
LL | subspan!("hihi"); //~ ERROR found 'hi's
   |           ^^^^
   = note: this error originates in the macro `subspan` (in Nightly builds, run with -Z macro-backtrace for more info)

error: found 'hi's
   |
   |
LL | subspan!("hihihi"); //~ ERROR found 'hi's
   |
note: here
  --> /checkout/src/test/ui/proc-macro/subspan.rs:17:11
   |
   |
LL | subspan!("hihihi"); //~ ERROR found 'hi's
   |           ^^^^^^
   = note: this error originates in the macro `subspan` (in Nightly builds, run with -Z macro-backtrace for more info)

error: found 'hi's
   |
   |
LL | subspan!("why I hide? hi!"); //~ ERROR found 'hi's
   |
note: here
  --> /checkout/src/test/ui/proc-macro/subspan.rs:20:17
   |
   |
LL | subspan!("why I hide? hi!"); //~ ERROR found 'hi's
   |                 ^^    ^^
   = note: this error originates in the macro `subspan` (in Nightly builds, run with -Z macro-backtrace for more info)

error: found 'hi's
   |
   |
LL | subspan!("hey, hi, hidy, hidy, hi hi"); //~ ERROR found 'hi's
   |
note: here
  --> /checkout/src/test/ui/proc-macro/subspan.rs:21:16
   |
   |
LL | subspan!("hey, hi, hidy, hidy, hi hi"); //~ ERROR found 'hi's
   |                ^^  ^^    ^^    ^^ ^^
   = note: this error originates in the macro `subspan` (in Nightly builds, run with -Z macro-backtrace for more info)

error: found 'hi's
   |
   |
LL | subspan!("this is a hi, and this is another hi"); //~ ERROR found 'hi's
   |
note: here
  --> /checkout/src/test/ui/proc-macro/subspan.rs:22:12
   |
   |
LL | subspan!("this is a hi, and this is another hi"); //~ ERROR found 'hi's
   |            ^^       ^^       ^^             ^^
   = note: this error originates in the macro `subspan` (in Nightly builds, run with -Z macro-backtrace for more info)

error: found 'hi's
   |
   |
LL | subspan!("how are you this evening"); //~ ERROR found 'hi's
   |
note: here
  --> /checkout/src/test/ui/proc-macro/subspan.rs:23:24
   |
   |
LL | subspan!("how are you this evening"); //~ ERROR found 'hi's
   |                        ^^
   = note: this error originates in the macro `subspan` (in Nightly builds, run with -Z macro-backtrace for more info)

error: found 'hi's
   |
   |
LL | subspan!("this is highly eradic"); //~ ERROR found 'hi's
   |
note: here
  --> /checkout/src/test/ui/proc-macro/subspan.rs:24:12
   |
   |
LL | subspan!("this is highly eradic"); //~ ERROR found 'hi's
   |            ^^     ^^
   = note: this error originates in the macro `subspan` (in Nightly builds, run with -Z macro-backtrace for more info)
error: aborting due to 8 previous errors


------------------------------------------
------------------------------------------


---- [ui] ui/recursion_limit/zero.rs stdout ----
diff of stderr:

2   --> $DIR/zero.rs:10:1
3    |
4 LL | test!(test);
+    | ^^^^^^^^^^^
6    |
6    |
7    = help: consider increasing the recursion limit by adding a `#![recursion_limit = "2"]` attribute to your crate (`zero`)


The actual stderr differed from the expected stderr.
The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/recursion_limit/zero/zero.stderr
To only update this specific test, also pass `--test-args recursion_limit/zero.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/recursion_limit/zero.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/recursion_limit/zero" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/recursion_limit/zero/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: recursion limit reached while expanding `test!`
  --> /checkout/src/test/ui/recursion_limit/zero.rs:10:1
   |
LL | test!(test); //~ ERROR 10:1: 10:13: recursion limit reached while expanding `test!`
   |
   |
   = help: consider increasing the recursion limit by adding a `#![recursion_limit = "2"]` attribute to your crate (`zero`)
error: aborting due to previous error


------------------------------------------
---
16 ...
17 LL |       m!();
-    |       ----- in this macro invocation
+    |       ---- in this macro invocation
19    = help: use `self::std` to refer to this module unambiguously
20    = note: this error originates in the macro `m` (in Nightly builds, run with -Z macro-backtrace for more info)


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rust-2018/uniform-paths/ambiguity-macros-nested/ambiguity-macros-nested.stderr
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rust-2018/uniform-paths/ambiguity-macros-nested/ambiguity-macros-nested.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args rust-2018/uniform-paths/ambiguity-macros-nested.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/rust-2018/uniform-paths/ambiguity-macros-nested.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rust-2018/uniform-paths/ambiguity-macros-nested" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--edition=2018" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rust-2018/uniform-paths/ambiguity-macros-nested/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0659]: `std` is ambiguous (name vs any other name during import resolution)
   |
LL |     pub use std::io;
   |             ^^^ ambiguous name
   |
   |
   = note: `std` could refer to a built-in crate
   = help: use `::std` to refer to this crate unambiguously
note: `std` could also refer to the module defined here
   |
LL | /             mod std {
LL | |                 pub struct io;
LL | |             }
LL | |             }
   | |_____________^
...
LL |       m!();
   |       ---- in this macro invocation
   = help: use `self::std` to refer to this module unambiguously
   = note: this error originates in the macro `m` (in Nightly builds, run with -Z macro-backtrace for more info)
error: aborting due to previous error

For more information about this error, try `rustc --explain E0659`.

---
16 ...
17 LL |   m!();
-    |   ----- in this macro invocation
+    |   ---- in this macro invocation
19    = help: use `crate::std` to refer to this module unambiguously
20    = note: this error originates in the macro `m` (in Nightly builds, run with -Z macro-backtrace for more info)


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rust-2018/uniform-paths/ambiguity-macros/ambiguity-macros.stderr
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rust-2018/uniform-paths/ambiguity-macros/ambiguity-macros.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args rust-2018/uniform-paths/ambiguity-macros.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/rust-2018/uniform-paths/ambiguity-macros.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rust-2018/uniform-paths/ambiguity-macros" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--edition=2018" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rust-2018/uniform-paths/ambiguity-macros/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0659]: `std` is ambiguous (name vs any other name during import resolution)
   |
LL | use std::io;
   |     ^^^ ambiguous name
   |
   |
   = note: `std` could refer to a built-in crate
   = help: use `::std` to refer to this crate unambiguously
note: `std` could also refer to the module defined here
   |
LL | /         mod std {
LL | |             pub struct io;
LL | |         }
LL | |         }
   | |_________^
...
LL |   m!();
   |   ---- in this macro invocation
   = help: use `crate::std` to refer to this module unambiguously
   = note: this error originates in the macro `m` (in Nightly builds, run with -Z macro-backtrace for more info)
error: aborting due to previous error

For more information about this error, try `rustc --explain E0659`.

---
diff of stderr:

10   ::: $DIR/transitive-dep-span.rs:13:1
11    |
12 LL |   transitive_dep_two::parse_error!();
+    |   ----------------------------------
14    |   |
15    |   in this macro invocation
16    |   in this macro invocation
---
To only update this specific test, also pass `--test-args span/transitive-dep-span.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/span/transitive-dep-span.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/span/transitive-dep-span" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "macro-backtrace" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/span/transitive-dep-span/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: expected one of `!` or `::`, found `error`
  --> /checkout/src/test/ui/span/auxiliary/transitive_dep_three.rs:6:27
LL | /         macro_rules! parse_error {
LL | /         macro_rules! parse_error {
LL | |             () => { parse error }
   | |                           ^^^^^ expected one of `!` or `::`
LL | |         }
   | |_________- in this expansion of `transitive_dep_two::parse_error!`
  ::: /checkout/src/test/ui/span/transitive-dep-span.rs:13:1
   |
   |
LL |   transitive_dep_two::parse_error!(); //~ ERROR expected one of
   |   |
   |   in this macro invocation
   |   in this macro invocation

---
diff of stderr:

131    |     ^^^^^^^^^^^^^^^^^^^^
132 ...
133 LL | duplicate_field_name_test!(x);
+    | ----------------------------- in this macro invocation
135    |
136    = note: this error originates in the macro `duplicate_field_name_test` (in Nightly builds, run with -Z macro-backtrace for more info)
137 
137 

142    |     ^^^^^^^^^^^^^^^^^^^^
143 ...
144 LL | duplicate_field_name_test!(x);
+    | ----------------------------- in this macro invocation
146    |
147    = note: this error originates in the macro `duplicate_field_name_test` (in Nightly builds, run with -Z macro-backtrace for more info)
148 
148 

153    |     ^^^^^^^^^^^^^^^^^^^^
154 ...
155 LL | duplicate_field_name_test!(x);
+    | ----------------------------- in this macro invocation
157    |
158    = note: this error originates in the macro `duplicate_field_name_test` (in Nightly builds, run with -Z macro-backtrace for more info)
159 
---
To only update this specific test, also pass `--test-args symbol-names/const-generics-structural-demangling.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/symbol-names/const-generics-structural-demangling.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/symbol-names/const-generics-structural-demangling" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "symbol-mangling-version=v0" "--crate-name=c" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/symbol-names/const-generics-structural-demangling/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: symbol-name(_RMCsno73SFvQKx_1cINtB0_7RefByteKRh7b_E)
   |
LL | #[rustc_symbol_name]
   | ^^^^^^^^^^^^^^^^^^^^


error: demangling(<c[464da6a86cb672f]::RefByte<{&123u8}>>)
   |
LL | #[rustc_symbol_name]
   | ^^^^^^^^^^^^^^^^^^^^


error: demangling-alt(<c::RefByte<{&123}>>)
   |
LL | #[rustc_symbol_name]
   | ^^^^^^^^^^^^^^^^^^^^


error: symbol-name(_RMs_Csno73SFvQKx_1cINtB2_6RefZstKRAEE)
   |
LL | #[rustc_symbol_name]
   | ^^^^^^^^^^^^^^^^^^^^


error: demangling(<c[464da6a86cb672f]::RefZst<{&[]}>>)
   |
LL | #[rustc_symbol_name]
   | ^^^^^^^^^^^^^^^^^^^^


error: demangling-alt(<c::RefZst<{&[]}>>)
   |
LL | #[rustc_symbol_name]
   | ^^^^^^^^^^^^^^^^^^^^


error: symbol-name(_RMs0_Csno73SFvQKx_1cINtB3_11Array3BytesKAh1_h2_h3_EE)
   |
LL | #[rustc_symbol_name]
   | ^^^^^^^^^^^^^^^^^^^^


error: demangling(<c[464da6a86cb672f]::Array3Bytes<{[1u8, 2u8, 3u8]}>>)
   |
LL | #[rustc_symbol_name]
   | ^^^^^^^^^^^^^^^^^^^^


error: demangling-alt(<c::Array3Bytes<{[1, 2, 3]}>>)
   |
LL | #[rustc_symbol_name]
   | ^^^^^^^^^^^^^^^^^^^^


error: symbol-name(_RMs1_Csno73SFvQKx_1cINtB3_13TupleByteBoolKTh1_b0_EE)
   |
LL | #[rustc_symbol_name]
   | ^^^^^^^^^^^^^^^^^^^^


error: demangling(<c[464da6a86cb672f]::TupleByteBool<{(1u8, false)}>>)
   |
LL | #[rustc_symbol_name]
   | ^^^^^^^^^^^^^^^^^^^^


error: demangling-alt(<c::TupleByteBool<{(1, false)}>>)
   |
LL | #[rustc_symbol_name]
   | ^^^^^^^^^^^^^^^^^^^^


error: symbol-name(_RMs2_Csno73SFvQKx_1cINtB3_11OptionUsizeKVNtINtNtCs7gJWy4nsU48_4core6option6OptionjE4NoneUE)
   |
LL | #[rustc_symbol_name]
   | ^^^^^^^^^^^^^^^^^^^^


error: demangling(<c[464da6a86cb672f]::OptionUsize<{core[54adaa694f584a72]::option::Option::<usize>::None}>>)
   |
LL | #[rustc_symbol_name]
   | ^^^^^^^^^^^^^^^^^^^^


error: demangling-alt(<c::OptionUsize<{core::option::Option::<usize>::None}>>)
   |
LL | #[rustc_symbol_name]
   | ^^^^^^^^^^^^^^^^^^^^


error: symbol-name(_RMs3_Csno73SFvQKx_1cINtB3_11OptionUsizeKVNtINtNtCs7gJWy4nsU48_4core6option6OptionjE4SomeTj0_EE)
   |
LL | #[rustc_symbol_name]
   | ^^^^^^^^^^^^^^^^^^^^


error: demangling(<c[464da6a86cb672f]::OptionUsize<{core[54adaa694f584a72]::option::Option::<usize>::Some(0usize)}>>)
   |
LL | #[rustc_symbol_name]
   | ^^^^^^^^^^^^^^^^^^^^


error: demangling-alt(<c::OptionUsize<{core::option::Option::<usize>::Some(0)}>>)
   |
LL | #[rustc_symbol_name]
   | ^^^^^^^^^^^^^^^^^^^^


error: symbol-name(_RMs4_Csno73SFvQKx_1cINtB3_4Foo_KVNtB3_3FooS1sRe616263_2chc78_5sliceRAh1_h2_h3_EEE)
   |
LL | #[rustc_symbol_name]
   | ^^^^^^^^^^^^^^^^^^^^


error: demangling(<c[464da6a86cb672f]::Foo_<{c[464da6a86cb672f]::Foo { s: "abc", ch: 'x', slice: &[1u8, 2u8, 3u8] }}>>)
   |
LL | #[rustc_symbol_name]
   | ^^^^^^^^^^^^^^^^^^^^


error: demangling-alt(<c::Foo_<{c::Foo { s: "abc", ch: 'x', slice: &[1, 2, 3] }}>>)
   |
LL | #[rustc_symbol_name]
   | ^^^^^^^^^^^^^^^^^^^^


error: symbol-name(_RMs9_Csno73SFvQKx_1cINtB3_4Bar_KVNtB3_3BarS1xh7b_s_1xt1000_EE)
   |
LL |     #[rustc_symbol_name]
   |     ^^^^^^^^^^^^^^^^^^^^
...
...
LL | duplicate_field_name_test!(x);
   |
   = note: this error originates in the macro `duplicate_field_name_test` (in Nightly builds, run with -Z macro-backtrace for more info)


error: demangling(<c[464da6a86cb672f]::Bar_<{c[464da6a86cb672f]::Bar { x: 123u8, x: 4096u16 }}>>)
   |
LL |     #[rustc_symbol_name]
   |     ^^^^^^^^^^^^^^^^^^^^
...
...
LL | duplicate_field_name_test!(x);
   |
   = note: this error originates in the macro `duplicate_field_name_test` (in Nightly builds, run with -Z macro-backtrace for more info)


error: demangling-alt(<c::Bar_<{c::Bar { x: 123, x: 4096 }}>>)
   |
LL |     #[rustc_symbol_name]
   |     ^^^^^^^^^^^^^^^^^^^^
...
...
LL | duplicate_field_name_test!(x);
   |
   = note: this error originates in the macro `duplicate_field_name_test` (in Nightly builds, run with -Z macro-backtrace for more info)

error: aborting due to 24 previous errors
---
test result: FAILED. 12111 passed; 56 failed; 117 ignored; 0 measured; 0 filtered out; finished in 129.91s



command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--suite" "ui" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-10/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python3" "--lldb-python" "/usr/bin/python3" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "10.0.0" "--llvm-components" "aarch64 aarch64asmparser aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets amdgpu amdgpuasmparser amdgpucodegen amdgpudesc amdgpudisassembler amdgpuinfo amdgpuutils analysis arm armasmparser armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter avr avrasmparser avrcodegen avrdesc avrdisassembler avrinfo binaryformat bitreader bitstreamreader bitwriter bpf bpfasmparser bpfcodegen bpfdesc bpfdisassembler bpfinfo cfguard codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfogsym debuginfomsf debuginfopdb demangle dlltooldriver dwarflinker engine executionengine frontendopenmp fuzzmutate globalisel hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interpreter ipo irreader jitlink lanai lanaiasmparser lanaicodegen lanaidesc lanaidisassembler lanaiinfo libdriver lineeditor linker lto mc mca mcdisassembler mcjit mcparser mips mipsasmparser mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmparser msp430codegen msp430desc msp430disassembler msp430info native nativecodegen nvptx nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcerror orcjit passes perfjitevents powerpc powerpcasmparser powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata remarks riscv riscvasmparser riscvcodegen riscvdesc riscvdisassembler riscvinfo riscvutils runtimedyld scalaropts selectiondag sparc sparcasmparser sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target textapi transformutils vectorize webassembly webassemblyasmparser webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo windowsmanifest x86 x86asmparser x86codegen x86desc x86disassembler x86info x86utils xcore xcorecodegen xcoredesc xcoredisassembler xcoreinfo xray" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--channel" "nightly" "--color" "always"


Build completed unsuccessfully in 0:14:17
