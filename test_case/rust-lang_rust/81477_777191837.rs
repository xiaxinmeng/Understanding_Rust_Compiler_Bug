plain

#######################################                                   54.3%
######################################################################## 100.0%
extracting /checkout/obj/build/cache/2021-01-28/rustfmt-nightly-x86_64-unknown-linux-gnu.tar.xz
    Updating git repository `https://github.com/webdesus/fs_extra`
---
.................................................................................................... 3600/11439
.......................i............................................................................ 3700/11439
.................................................................................................... 3800/11439
.................................................................................................... 3900/11439
.................................................................................................F.. 4000/11439
....F...............F............................................................................... 4100/11439
.................................................................................................... 4300/11439
...................ii............................................................................... 4400/11439
...i................................................................................................ 4500/11439
.................................................................................................... 4600/11439
---
.................................................................................................ii. 6200/11439
ii.......i...i...................................................................................... 6300/11439
...............................i....i.................................i........................i.... 6400/11439
.................................................................................................... 6500/11439
..............i..........................F...F...................................................... 6600/11439
..................................................................................................ii 6700/11439
..................................F.........i.............F......................................... 6800/11439
................................................F................................................... 6900/11439
.......................i............................................................................ 7100/11439
......................ii................i.i..ii..................................................... 7200/11439
.................................................................................................... 7300/11439
.................................................................................................... 7400/11439
---

+ error: Trailing semicolon in macro
+   --> $DIR/hygienic-label-1.rs:2:21
+    |
+ LL |     () => { break 'x; }
+ ...
+ ...
+ LL |     'x: loop { foo!() }
+    |
+    = note: this error originates in a macro (in Nightly builds, run with -Z macro-backtrace for more info)
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
+ 
---
15 


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/hygiene/hygienic-label-1/hygienic-label-1.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args hygiene/hygienic-label-1.rs`
error: 1 errors occurred comparing output.
status: exit code: 1
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/hygiene/hygienic-label-1.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/hygiene/hygienic-label-1" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/hygiene/hygienic-label-1/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: Trailing semicolon in macro
  --> /checkout/src/test/ui/hygiene/hygienic-label-1.rs:2:21
   |
LL |     () => { break 'x; } //~ ERROR use of undeclared label `'x`
...
...
LL |     'x: loop { foo!() }
   |
   = note: this error originates in a macro (in Nightly builds, run with -Z macro-backtrace for more info)

error[E0426]: use of undeclared label `'x`
error[E0426]: use of undeclared label `'x`
  --> /checkout/src/test/ui/hygiene/hygienic-label-1.rs:2:19
   |
LL |     () => { break 'x; } //~ ERROR use of undeclared label `'x`
   |                   ^^ undeclared label `'x`
...
LL |     'x: loop { foo!() }
   |
   = note: this error originates in a macro (in Nightly builds, run with -Z macro-backtrace for more info)

error: aborting due to 2 previous errors
---

+ error: Trailing semicolon in macro
+   --> $DIR/hygienic-label-3.rs:2:21
+    |
+ LL |     () => { break 'x; }
+ ...
+ ...
+ LL |         foo!()
+    |
+    = note: this error originates in a macro (in Nightly builds, run with -Z macro-backtrace for more info)
+ 
1 error[E0426]: use of undeclared label `'x`
---
15 


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/hygiene/hygienic-label-3/hygienic-label-3.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args hygiene/hygienic-label-3.rs`
error: 1 errors occurred comparing output.
status: exit code: 1
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/hygiene/hygienic-label-3.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/hygiene/hygienic-label-3" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/hygiene/hygienic-label-3/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: Trailing semicolon in macro
  --> /checkout/src/test/ui/hygiene/hygienic-label-3.rs:2:21
   |
LL |     () => { break 'x; } //~ ERROR use of undeclared label `'x`
...
LL |         foo!()
   |         ------ in this macro invocation
   |
   |
   = note: this error originates in a macro (in Nightly builds, run with -Z macro-backtrace for more info)

error[E0426]: use of undeclared label `'x`
  --> /checkout/src/test/ui/hygiene/hygienic-label-3.rs:2:19
   |
LL |     () => { break 'x; } //~ ERROR use of undeclared label `'x`
   |                   ^^ undeclared label `'x`
LL |         foo!()
   |         ------ in this macro invocation
   |
   = note: this error originates in a macro (in Nightly builds, run with -Z macro-backtrace for more info)
---

---- [ui] ui/hygiene/intercrate.rs stdout ----
diff of stderr:

- error: type `fn() -> u32 {foo::bar::f}` is private
+ error: Trailing semicolon in macro
3    |
3    |
4 LL |     assert_eq!(intercrate::foo::m!(), 1);
-    |                ^^^^^^^^^^^^^^^^^^^^^ private type
+    |                ^^^^^^^^^^^^^^^^^^^^^
6    |
7    = note: this error originates in a macro (in Nightly builds, run with -Z macro-backtrace for more info)
7    = note: this error originates in a macro (in Nightly builds, run with -Z macro-backtrace for more info)
8 


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/hygiene/intercrate/intercrate.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args hygiene/intercrate.rs`
error: 1 errors occurred comparing output.
status: exit code: 1
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/hygiene/intercrate.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/hygiene/intercrate" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/hygiene/intercrate/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: Trailing semicolon in macro
  --> /checkout/src/test/ui/hygiene/intercrate.rs:10:16
   |
LL |     assert_eq!(intercrate::foo::m!(), 1);
   |
   = note: this error originates in a macro (in Nightly builds, run with -Z macro-backtrace for more info)

error: aborting due to previous error
error: aborting due to previous error


------------------------------------------


---- [ui] ui/lint/semicolon-in-expressions-from-macros/allow-semicolon-in-expressions-from-macros.rs stdout ----

error: test compilation failed although it shouldn't!
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/lint/semicolon-in-expressions-from-macros/allow-semicolon-in-expressions-from-macros.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/semicolon-in-expressions-from-macros/allow-semicolon-in-expressions-from-macros" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/semicolon-in-expressions-from-macros/allow-semicolon-in-expressions-from-macros/auxiliary"
------------------------------------------

------------------------------------------
stderr:
---


---- [ui] ui/lint/semicolon-in-expressions-from-macros/semicolon-in-expressions-from-macros.rs stdout ----

error: test compilation failed although it shouldn't!
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/lint/semicolon-in-expressions-from-macros/semicolon-in-expressions-from-macros.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/semicolon-in-expressions-from-macros/semicolon-in-expressions-from-macros" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--edition=2018" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/semicolon-in-expressions-from-macros/semicolon-in-expressions-from-macros/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: Trailing semicolon in macro
  --> /checkout/src/test/ui/lint/semicolon-in-expressions-from-macros/semicolon-in-expressions-from-macros.rs:8:13
   |
LL |         true; //~ WARN trailing
...
...
LL |         foo!(first)
   |
   = note: this error originates in a macro (in Nightly builds, run with -Z macro-backtrace for more info)

error: Trailing semicolon in macro
error: Trailing semicolon in macro
  --> /checkout/src/test/ui/lint/semicolon-in-expressions-from-macros/semicolon-in-expressions-from-macros.rs:8:13
   |
LL |         true; //~ WARN trailing
...
...
LL |     let _ = foo!(second);
   |
   = note: this error originates in a macro (in Nightly builds, run with -Z macro-backtrace for more info)

error: Trailing semicolon in macro
error: Trailing semicolon in macro
  --> /checkout/src/test/ui/lint/semicolon-in-expressions-from-macros/semicolon-in-expressions-from-macros.rs:8:13
   |
LL |         true; //~ WARN trailing
...
...
LL |         let _ = foo!(third);
   |
   = note: this error originates in a macro (in Nightly builds, run with -Z macro-backtrace for more info)

error: Trailing semicolon in macro
error: Trailing semicolon in macro
  --> /checkout/src/test/ui/lint/semicolon-in-expressions-from-macros/semicolon-in-expressions-from-macros.rs:8:13
   |
LL |         true; //~ WARN trailing
...
...
LL |         let _ = foo!(fourth);
   |
   = note: this error originates in a macro (in Nightly builds, run with -Z macro-backtrace for more info)

warning: trailing semicolon in macro used in expression position
warning: trailing semicolon in macro used in expression position
  --> /checkout/src/test/ui/lint/semicolon-in-expressions-from-macros/semicolon-in-expressions-from-macros.rs:8:13
   |
LL |         true; //~ WARN trailing
...
...
LL |         foo!(first)
   |
note: the lint level is defined here
  --> /checkout/src/test/ui/lint/semicolon-in-expressions-from-macros/semicolon-in-expressions-from-macros.rs:3:9
   |
---

warning: trailing semicolon in macro used in expression position
  --> /checkout/src/test/ui/lint/semicolon-in-expressions-from-macros/semicolon-in-expressions-from-macros.rs:8:13
   |
LL |         true; //~ WARN trailing
...
...
LL |     let _ = foo!(second);
   |
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
   = note: for more information, see issue #79813 <https://github.com/rust-lang/rust/issues/79813>
   = note: this warning originates in a macro (in Nightly builds, run with -Z macro-backtrace for more info)
---
---- [ui] ui/macros/macro-context.rs stdout ----
diff of stderr:

9    |
10    = note: the usage of `m!` is likely invalid in type context
+ error: Trailing semicolon in macro
+   --> $DIR/macro-context.rs:3:15
+    |
+    |
+ LL |     () => ( i ; typeof );
+ ...
+ ...
+ LL |     let i = m!();
+    |
+    = note: this error originates in a macro (in Nightly builds, run with -Z macro-backtrace for more info)
+ 
+ 
12 error: macro expansion ignores token `typeof` and any following
14    |

64    |
65    = note: this error originates in a macro (in Nightly builds, run with -Z macro-backtrace for more info)
---
70 For more information about an error, try `rustc --explain E0412`.


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/macros/macro-context/macro-context.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args macros/macro-context.rs`
error: 1 errors occurred comparing output.
status: exit code: 1
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/macros/macro-context.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/macros/macro-context" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/macros/macro-context/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: macro expansion ignores token `;` and any following
   |
   |
LL |     () => ( i ; typeof );   //~ ERROR expected expression, found reserved keyword `typeof`
...
...
LL |     let a: m!();
   |            ---- caused by the macro expansion here
   |
   = note: the usage of `m!` is likely invalid in type context
error: Trailing semicolon in macro
  --> /checkout/src/test/ui/macros/macro-context.rs:3:15
   |
   |
LL |     () => ( i ; typeof );   //~ ERROR expected expression, found reserved keyword `typeof`
...
...
LL |     let i = m!();
   |
   = note: this error originates in a macro (in Nightly builds, run with -Z macro-backtrace for more info)


error: macro expansion ignores token `typeof` and any following
   |
   |
LL |     () => ( i ; typeof );   //~ ERROR expected expression, found reserved keyword `typeof`
...
...
LL |     let i = m!();
   |             ---- caused by the macro expansion here
   |
   = note: the usage of `m!` is likely invalid in expression context

error: macro expansion ignores token `;` and any following
   |
   |
LL |     () => ( i ; typeof );   //~ ERROR expected expression, found reserved keyword `typeof`
...
...
LL |         m!() => {}
   |         ---- caused by the macro expansion here
   |
   = note: the usage of `m!` is likely invalid in pattern context

error: expected expression, found reserved keyword `typeof`
   |
   |
LL |     () => ( i ; typeof );   //~ ERROR expected expression, found reserved keyword `typeof`
...
LL |     m!();
   |     ----- in this macro invocation
   |
   |
   = note: this error originates in a macro (in Nightly builds, run with -Z macro-backtrace for more info)

error[E0412]: cannot find type `i` in this scope
  --> /checkout/src/test/ui/macros/macro-context.rs:3:13
   |
LL |     () => ( i ; typeof );   //~ ERROR expected expression, found reserved keyword `typeof`
   |             ^ help: a builtin type with a similar name exists: `i8`
...
LL |     let a: m!();
   |
   = note: this error originates in a macro (in Nightly builds, run with -Z macro-backtrace for more info)

error[E0425]: cannot find value `i` in this scope
error[E0425]: cannot find value `i` in this scope
  --> /checkout/src/test/ui/macros/macro-context.rs:3:13
   |
LL |     () => ( i ; typeof );   //~ ERROR expected expression, found reserved keyword `typeof`
   |             ^ help: a local variable with a similar name exists: `a`
...
LL |     let i = m!();
   |
   = note: this error originates in a macro (in Nightly builds, run with -Z macro-backtrace for more info)

error: aborting due to 7 previous errors
---

+ error: Trailing semicolon in macro
+   --> $DIR/macro-in-expression-context.rs:5:29
+    |
+ LL |         assert_eq!("A", "A");
+ ...
+ ...
+ LL |     foo!()
+    |
+    = note: this error originates in a macro (in Nightly builds, run with -Z macro-backtrace for more info)
+ 
+ 
1 error: macro expansion ignores token `assert_eq` and any following
3    |

11    |
11    |
12    = note: the usage of `foo!` is likely invalid in expression context
- error: aborting due to previous error
+ error: aborting due to 2 previous errors
15 
16 
16 


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/macros/macro-in-expression-context/macro-in-expression-context.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args macros/macro-in-expression-context.rs`
error: 1 errors occurred comparing output.
status: exit code: 1
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/macros/macro-in-expression-context.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/macros/macro-in-expression-context" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/macros/macro-in-expression-context/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: Trailing semicolon in macro
  --> /checkout/src/test/ui/macros/macro-in-expression-context.rs:5:29
   |
LL |         assert_eq!("A", "A");
...
LL |     foo!()
   |     ------ in this macro invocation
   |
   |
   = note: this error originates in a macro (in Nightly builds, run with -Z macro-backtrace for more info)

error: macro expansion ignores token `assert_eq` and any following
   |
   |
LL |         assert_eq!("B", "B");
...
LL |     foo!()
LL |     foo!()
   |     ------- help: you might be missing a semicolon here: `;`
   |     caused by the macro expansion here
   |
   |
   = note: the usage of `foo!` is likely invalid in expression context
error: aborting due to 2 previous errors


------------------------------------------
------------------------------------------


---- [ui] ui/macros/trace_faulty_macros.rs stdout ----
diff of stderr:

49    = note: expanding `my_recursive_macro! {  }`
50    = note: to `my_recursive_macro ! () ;`
+ error: Trailing semicolon in macro
+   --> $DIR/trace_faulty_macros.rs:13:41
+    |
+    |
+ LL |         pat_macro!(A{a:a, b:0, c:_, ..});
+ ...
+ ...
+ LL |     let a = pat_macro!();
+    |
+    = note: this error originates in a macro (in Nightly builds, run with -Z macro-backtrace for more info)
+ 
+ 
52 error: expected expression, found `A { a: a, b: 0, c: _, .. }`
54    |


77    = note: expanding `pat_macro! { A { a : a, b : 0, c : _, .. } }`
78    = note: to `A { a: a, b: 0, c: _, .. }`
- error: aborting due to 4 previous errors
+ error: aborting due to 5 previous errors
81 
82 For more information about this error, try `rustc --explain E0774`.
82 For more information about this error, try `rustc --explain E0774`.
83 


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/macros/trace_faulty_macros/trace_faulty_macros.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args macros/trace_faulty_macros.rs`
error: 1 errors occurred comparing output.
status: exit code: 1
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/macros/trace_faulty_macros.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/macros/trace_faulty_macros" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "trace-macros" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/macros/trace_faulty_macros/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: no rules expected the token `bcd`
   |
   |
LL | macro_rules! my_faulty_macro {
   | ---------------------------- when calling this macro
LL |     () => {
LL |         my_faulty_macro!(bcd); //~ ERROR no rules
   |                          ^^^ no rules expected this token in macro call
...
LL |     my_faulty_macro!();
   |
   = note: this error originates in a macro (in Nightly builds, run with -Z macro-backtrace for more info)

note: trace_macro
note: trace_macro
  --> /checkout/src/test/ui/macros/trace_faulty_macros.rs:31:5
   |
LL |     my_faulty_macro!();
   |
   |
   = note: expanding `my_faulty_macro! {  }`
   = note: to `my_faulty_macro ! (bcd) ;`
   = note: expanding `my_faulty_macro! { bcd }`

error: recursion limit reached while expanding `my_recursive_macro!`
   |
   |
LL |         my_recursive_macro!(); //~ ERROR recursion limit
...
...
LL |     my_recursive_macro!();
   |
   |
   = help: consider adding a `#![recursion_limit="8"]` attribute to your crate (`trace_faulty_macros`)
   = note: this error originates in a macro (in Nightly builds, run with -Z macro-backtrace for more info)
note: trace_macro
  --> /checkout/src/test/ui/macros/trace_faulty_macros.rs:32:5
   |
   |
LL |     my_recursive_macro!();
   |
   |
   = note: expanding `my_recursive_macro! {  }`
   = note: to `my_recursive_macro ! () ;`
   = note: expanding `my_recursive_macro! {  }`
   = note: to `my_recursive_macro ! () ;`
   = note: expanding `my_recursive_macro! {  }`
   = note: to `my_recursive_macro ! () ;`
   = note: expanding `my_recursive_macro! {  }`
   = note: to `my_recursive_macro ! () ;`
error: Trailing semicolon in macro
  --> /checkout/src/test/ui/macros/trace_faulty_macros.rs:13:41
   |
   |
LL |         pat_macro!(A{a:a, b:0, c:_, ..});
...
...
LL |     let a = pat_macro!();
   |
   = note: this error originates in a macro (in Nightly builds, run with -Z macro-backtrace for more info)


error: expected expression, found `A { a: a, b: 0, c: _, .. }`
   |
   |
LL |         $a //~ ERROR expected expression
...
...
LL |     let a = pat_macro!();
   |
   = note: this error originates in a macro (in Nightly builds, run with -Z macro-backtrace for more info)


error[E0774]: `derive` may only be applied to structs, enums and unions
   |
   |
LL | #[derive(Debug)] //~ ERROR `derive` may only be applied to structs

note: trace_macro
  --> /checkout/src/test/ui/macros/trace_faulty_macros.rs:36:13
   |
   |
LL |     let a = pat_macro!();
   |
   |
   = note: expanding `pat_macro! {  }`
   = note: to `pat_macro ! (A { a : a, b : 0, c : _, .. }) ;`
   = note: expanding `pat_macro! { A { a : a, b : 0, c : _, .. } }`
   = note: to `A { a: a, b: 0, c: _, .. }`
error: aborting due to 5 previous errors

For more information about this error, try `rustc --explain E0774`.


------------------------------------------


---- [ui] ui/proc-macro/nested-nonterminal-tokens.rs stdout ----

error: test compilation failed although it shouldn't!
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/proc-macro/nested-nonterminal-tokens.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/proc-macro/nested-nonterminal-tokens" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--edition=2018" "-Z" "span-debug" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/proc-macro/nested-nonterminal-tokens/auxiliary"
------------------------------------------
------------------------------------------
PRINT-BANG INPUT (DISPLAY): 0 + 1 + 2 + 3
PRINT-BANG INPUT (DEBUG): TokenStream [
    Group {
        delimiter: None,
        stream: TokenStream [
            Group {
                delimiter: None,
                stream: TokenStream [
                    Group {
                        delimiter: None,
                        stream: TokenStream [
                            Literal {
                                kind: Integer,
                                symbol: "0",
                                suffix: None,
                                span: /checkout/src/test/ui/proc-macro/nested-nonterminal-tokens.rs:25:26: 25:27 (#0),
                        ],
                        ],
                        span: /checkout/src/test/ui/proc-macro/nested-nonterminal-tokens.rs:17:41: 17:43 (#4),
                    Punct {
                    Punct {
                        ch: '+',
                        spacing: Alone,
                        span: /checkout/src/test/ui/proc-macro/nested-nonterminal-tokens.rs:17:44: 17:45 (#4),
                    Literal {
                        kind: Integer,
                        symbol: "1",
                        suffix: None,
                        suffix: None,
                        span: /checkout/src/test/ui/proc-macro/nested-nonterminal-tokens.rs:17:46: 17:47 (#4),
                ],
                ],
                span: /checkout/src/test/ui/proc-macro/nested-nonterminal-tokens.rs:18:41: 18:43 (#5),
            Punct {
            Punct {
                ch: '+',
                spacing: Alone,
                span: /checkout/src/test/ui/proc-macro/nested-nonterminal-tokens.rs:18:44: 18:45 (#5),
            Literal {
                kind: Integer,
                symbol: "2",
                suffix: None,
                suffix: None,
                span: /checkout/src/test/ui/proc-macro/nested-nonterminal-tokens.rs:18:46: 18:47 (#5),
        ],
        ],
        span: /checkout/src/test/ui/proc-macro/nested-nonterminal-tokens.rs:20:21: 20:23 (#6),
    Punct {
    Punct {
        ch: '+',
        spacing: Alone,
        span: /checkout/src/test/ui/proc-macro/nested-nonterminal-tokens.rs:20:24: 20:25 (#6),
    Literal {
        kind: Integer,
        symbol: "3",
        suffix: None,
        suffix: None,
        span: /checkout/src/test/ui/proc-macro/nested-nonterminal-tokens.rs:20:26: 20:27 (#6),
]

------------------------------------------
stderr:
stderr:
------------------------------------------
error: Trailing semicolon in macro
  --> /checkout/src/test/ui/proc-macro/nested-nonterminal-tokens.rs:20:28
   |
LL |         print_bang!($e + 3);
...
...
LL |     let _ = wrap!(first, 0);
   |
   = note: this error originates in a macro (in Nightly builds, run with -Z macro-backtrace for more info)

error: aborting due to previous error
---
test result: FAILED. 11338 passed; 9 failed; 92 ignored; 0 measured; 0 filtered out; finished in 134.72s



command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--suite" "ui" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-9/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python3" "--lldb-python" "/usr/bin/python3" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "9.0.0" "--llvm-components" "aarch64 aarch64asmparser aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets amdgpu amdgpuasmparser amdgpucodegen amdgpudesc amdgpudisassembler amdgpuinfo amdgpuutils analysis arm armasmparser armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter avr avrasmparser avrcodegen avrdesc avrdisassembler avrinfo binaryformat bitreader bitstreamreader bitwriter bpf bpfasmparser bpfcodegen bpfdesc bpfdisassembler bpfinfo codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfogsym debuginfomsf debuginfopdb demangle dlltooldriver engine executionengine fuzzmutate globalisel hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interpreter ipo irreader jitlink lanai lanaiasmparser lanaicodegen lanaidesc lanaidisassembler lanaiinfo libdriver lineeditor linker lto mc mca mcdisassembler mcjit mcparser mips mipsasmparser mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmparser msp430codegen msp430desc msp430disassembler msp430info native nativecodegen nvptx nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcjit passes perfjitevents powerpc powerpcasmparser powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata remarks riscv riscvasmparser riscvcodegen riscvdesc riscvdisassembler riscvinfo riscvutils runtimedyld scalaropts selectiondag sparc sparcasmparser sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target textapi transformutils vectorize webassembly webassemblyasmparser webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo windowsmanifest x86 x86asmparser x86codegen x86desc x86disassembler x86info x86utils xcore xcorecodegen xcoredesc xcoredisassembler xcoreinfo xray" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"


failed to run: /checkout/obj/build/bootstrap/debug/bootstrap --stage 2 test --exclude src/tools/tidy
Build completed unsuccessfully in 0:14:48
