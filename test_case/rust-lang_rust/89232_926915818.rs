plain
.................................................................................................... 4200/12189
.................................................................................................... 4300/12189
.................................................................................................... 4400/12189
.................................................................................................... 4500/12189
....................ii................................F..............................F.............. 4600/12189
.................................................................................................... 4800/12189
...........................................F........................................................ 4900/12189
................................................................F................................... 5000/12189
....................................................................F............................... 5100/12189
---

---- [ui] ui/error-codes/E0275.rs stdout ----
diff of stderr:

4 LL | impl<T> Foo for T where Bar<T>: Foo {}
6    |
6    |
-    = help: consider adding a `#![recursion_limit="256"]` attribute to your crate (`E0275`)
+    = help: consider increasing the recursion limit by adding a `#![recursion_limit = "256"]` attribute to your crate (`E0275`)
8 note: required because of the requirements on the impl of `Foo` for `Bar<Bar<Bar<Bar<Bar<Bar<Bar<Bar<Bar<Bar<Bar<Bar<Bar<Bar<Bar<Bar<Bar<Bar<Bar<Bar<Bar<Bar<Bar<Bar<Bar<Bar<Bar<Bar<Bar<Bar<Bar<Bar<Bar<Bar<Bar<Bar<Bar<Bar<Bar<Bar<Bar<Bar<Bar<Bar<Bar<Bar<Bar<Bar<Bar<Bar<Bar<Bar<Bar<Bar<Bar<Bar<Bar<Bar<Bar<Bar<Bar<Bar<Bar<Bar<Bar<Bar<Bar<Bar<Bar<Bar<Bar<Bar<Bar<Bar<Bar<Bar<Bar<Bar<Bar<Bar<Bar<Bar<Bar<Bar<Bar<Bar<Bar<Bar<Bar<Bar<Bar<Bar<Bar<Bar<Bar<Bar<Bar<Bar<Bar<Bar<Bar<Bar<Bar<Bar<Bar<Bar<Bar<Bar<Bar<Bar<Bar<Bar<Bar<Bar<Bar<Bar<Bar<Bar<Bar<Bar<Bar<Bar<Bar<Bar<Bar<Bar<Bar<Bar<T>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>`
9   --> $DIR/E0275.rs:5:9


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/error-codes/E0275/E0275.stderr
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/error-codes/E0275/E0275.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args error-codes/E0275.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/error-codes/E0275.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/error-codes/E0275" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/error-codes/E0275/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0275]: overflow evaluating the requirement `Bar<Bar<Bar<Bar<Bar<Bar<Bar<Bar<Bar<Bar<Bar<Bar<Bar<Bar<Bar<Bar<Bar<Bar<Bar<Bar<Bar<Bar<Bar<Bar<Bar<Bar<Bar<Bar<Bar<Bar<Bar<Bar<Bar<Bar<Bar<Bar<Bar<Bar<Bar<Bar<Bar<Bar<Bar<Bar<Bar<Bar<Bar<Bar<Bar<Bar<Bar<Bar<Bar<Bar<Bar<Bar<Bar<Bar<Bar<Bar<Bar<Bar<Bar<Bar<Bar<Bar<Bar<Bar<Bar<Bar<Bar<Bar<Bar<Bar<Bar<Bar<Bar<Bar<Bar<Bar<Bar<Bar<Bar<Bar<Bar<Bar<Bar<Bar<Bar<Bar<Bar<Bar<Bar<Bar<Bar<Bar<Bar<Bar<Bar<Bar<Bar<Bar<Bar<Bar<Bar<Bar<Bar<Bar<Bar<Bar<Bar<Bar<Bar<Bar<Bar<Bar<Bar<Bar<Bar<Bar<Bar<Bar<Bar<Bar<Bar<Bar<Bar<Bar<Bar<T>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>: Foo`
   |
   |
LL | impl<T> Foo for T where Bar<T>: Foo {} //~ ERROR E0275
   |
   |
   = help: consider increasing the recursion limit by adding a `#![recursion_limit = "256"]` attribute to your crate (`E0275`)
note: required because of the requirements on the impl of `Foo` for `Bar<Bar<Bar<Bar<Bar<Bar<Bar<Bar<Bar<Bar<Bar<Bar<Bar<Bar<Bar<Bar<Bar<Bar<Bar<Bar<Bar<Bar<Bar<Bar<Bar<Bar<Bar<Bar<Bar<Bar<Bar<Bar<Bar<Bar<Bar<Bar<Bar<Bar<Bar<Bar<Bar<Bar<Bar<Bar<Bar<Bar<Bar<Bar<Bar<Bar<Bar<Bar<Bar<Bar<Bar<Bar<Bar<Bar<Bar<Bar<Bar<Bar<Bar<Bar<Bar<Bar<Bar<Bar<Bar<Bar<Bar<Bar<Bar<Bar<Bar<Bar<Bar<Bar<Bar<Bar<Bar<Bar<Bar<Bar<Bar<Bar<Bar<Bar<Bar<Bar<Bar<Bar<Bar<Bar<Bar<Bar<Bar<Bar<Bar<Bar<Bar<Bar<Bar<Bar<Bar<Bar<Bar<Bar<Bar<Bar<Bar<Bar<Bar<Bar<Bar<Bar<Bar<Bar<Bar<Bar<Bar<Bar<Bar<Bar<Bar<Bar<Bar<Bar<T>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>`
   |
   |
LL | impl<T> Foo for T where Bar<T>: Foo {} //~ ERROR E0275
   |         ^^^     ^
   = note: 127 redundant requirements hidden
   = note: required because of the requirements on the impl of `Foo` for `Bar<T>`
note: required by a bound in `Foo`
   |
LL | trait Foo {}
LL | trait Foo {}
   | ^^^^^^^^^ required by this bound in `Foo`
error: aborting due to previous error

For more information about this error, try `rustc --explain E0275`.

---

7 LL |     recursive!()
8    |     ------------ in this macro invocation
9    |
-    = help: consider adding a `#![recursion_limit="256"]` attribute to your crate (`infinite_macro_expansion`)
+    = help: consider increasing the recursion limit by adding a `#![recursion_limit = "256"]` attribute to your crate (`infinite_macro_expansion`)
11    = note: this error originates in the macro `recursive` (in Nightly builds, run with -Z macro-backtrace for more info)
13 error: aborting due to previous error


The actual stderr differed from the expected stderr.
The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/infinite/infinite-macro-expansion/infinite-macro-expansion.stderr
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args infinite/infinite-macro-expansion.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/infinite/infinite-macro-expansion.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/infinite/infinite-macro-expansion" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/infinite/infinite-macro-expansion/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: recursion limit reached while expanding `recursive!`
   |
   |
LL |     () => (recursive!()) //~ ERROR recursion limit reached while expanding `recursive!`
...
LL |     recursive!()
   |     ------------ in this macro invocation
   |
   |
   = help: consider increasing the recursion limit by adding a `#![recursion_limit = "256"]` attribute to your crate (`infinite_macro_expansion`)
   = note: this error originates in the macro `recursive` (in Nightly builds, run with -Z macro-backtrace for more info)
error: aborting due to previous error


------------------------------------------
------------------------------------------


---- [ui] ui/infinite/infinite-autoderef.rs stdout ----
diff of stderr:

15 LL |     Foo.foo;
16    |     ^^^^^^^ deref recursion limit reached
17    |
-    = help: consider adding a `#![recursion_limit="256"]` attribute to your crate (`infinite_autoderef`)
+    = help: consider increasing the recursion limit by adding a `#![recursion_limit = "256"]` attribute to your crate (`infinite_autoderef`)
19 
20 error[E0055]: reached the recursion limit while auto-dereferencing `Foo`


23 LL |     Foo.foo;
24    |         ^^^ deref recursion limit reached
25    |
-    = help: consider adding a `#![recursion_limit="256"]` attribute to your crate (`infinite_autoderef`)
+    = help: consider increasing the recursion limit by adding a `#![recursion_limit = "256"]` attribute to your crate (`infinite_autoderef`)
28 error[E0609]: no field `foo` on type `Foo`
29   --> $DIR/infinite-autoderef.rs:25:9


37 LL |     Foo.bar();
38    |         ^^^ deref recursion limit reached
39    |
-    = help: consider adding a `#![recursion_limit="256"]` attribute to your crate (`infinite_autoderef`)
+    = help: consider increasing the recursion limit by adding a `#![recursion_limit = "256"]` attribute to your crate (`infinite_autoderef`)
41 
42 error[E0599]: no method named `bar` found for struct `Foo` in the current scope


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/infinite/infinite-autoderef/infinite-autoderef.stderr
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/infinite/infinite-autoderef/infinite-autoderef.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args infinite/infinite-autoderef.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/infinite/infinite-autoderef.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/infinite/infinite-autoderef" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/infinite/infinite-autoderef/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0308]: mismatched types
  --> /checkout/src/test/ui/infinite/infinite-autoderef.rs:20:13
   |
LL |         x = box x;
   |             ^^^^^ cyclic type of infinite size
help: try using a conversion method
   |
   |
LL |         x = (box x).to_string();
   |             +     +++++++++++++

error[E0055]: reached the recursion limit while auto-dereferencing `Foo`
   |
   |
LL |     Foo.foo;
   |     ^^^^^^^ deref recursion limit reached
   |
   = help: consider increasing the recursion limit by adding a `#![recursion_limit = "256"]` attribute to your crate (`infinite_autoderef`)

error[E0055]: reached the recursion limit while auto-dereferencing `Foo`
   |
   |
LL |     Foo.foo;
   |         ^^^ deref recursion limit reached
   |
   = help: consider increasing the recursion limit by adding a `#![recursion_limit = "256"]` attribute to your crate (`infinite_autoderef`)
error[E0609]: no field `foo` on type `Foo`
  --> /checkout/src/test/ui/infinite/infinite-autoderef.rs:25:9
   |
   |
LL |     Foo.foo;


error[E0055]: reached the recursion limit while auto-dereferencing `Foo`
   |
   |
LL |     Foo.bar();
   |         ^^^ deref recursion limit reached
   |
   = help: consider increasing the recursion limit by adding a `#![recursion_limit = "256"]` attribute to your crate (`infinite_autoderef`)

error[E0599]: no method named `bar` found for struct `Foo` in the current scope
   |
LL | struct Foo;
LL | struct Foo;
   | ----------- method `bar` not found for this
...
LL |     Foo.bar();
   |         ^^^ method not found in `Foo`
error: aborting due to 6 previous errors

Some errors have detailed explanations: E0055, E0308, E0599, E0609.
For more information about an error, try `rustc --explain E0055`.
For more information about an error, try `rustc --explain E0055`.

------------------------------------------


---- [ui] ui/issues/issue-16098.rs stdout ----
diff of stderr:

7 LL |     println!("Problem 1: {}", prob1!(1000));
9    |
9    |
-    = help: consider adding a `#![recursion_limit="256"]` attribute to your crate (`issue_16098`)
+    = help: consider increasing the recursion limit by adding a `#![recursion_limit = "256"]` attribute to your crate (`issue_16098`)
11    = note: this error originates in the macro `prob1` (in Nightly builds, run with -Z macro-backtrace for more info)
13 error: aborting due to previous error


The actual stderr differed from the expected stderr.
The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-16098/issue-16098.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args issues/issue-16098.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues/issue-16098.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-16098" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-16098/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: recursion limit reached while expanding `prob1!`
  --> /checkout/src/test/ui/issues/issue-16098.rs:7:18
   |
LL |             $n + prob1!($n - 1); //~ ERROR recursion limit reached while expanding `prob1!`
...
...
LL |     println!("Problem 1: {}", prob1!(1000));
   |
   |
   = help: consider increasing the recursion limit by adding a `#![recursion_limit = "256"]` attribute to your crate (`issue_16098`)
   = note: this error originates in the macro `prob1` (in Nightly builds, run with -Z macro-backtrace for more info)
error: aborting due to previous error


------------------------------------------
------------------------------------------


---- [ui] ui/issues/issue-18400.rs stdout ----
diff of stderr:

4 LL |     0.contains(bits);
6    |
6    |
-    = help: consider adding a `#![recursion_limit="256"]` attribute to your crate (`issue_18400`)
+    = help: consider increasing the recursion limit by adding a `#![recursion_limit = "256"]` attribute to your crate (`issue_18400`)
8 note: required because of the requirements on the impl of `Set<&[_]>` for `{integer}`
10    |


The actual stderr differed from the expected stderr.
The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-18400/issue-18400.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args issues/issue-18400.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues/issue-18400.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-18400" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-18400/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0275]: overflow evaluating the requirement `_: Sized`
   |
   |
LL |     0.contains(bits);
   |
   |
   = help: consider increasing the recursion limit by adding a `#![recursion_limit = "256"]` attribute to your crate (`issue_18400`)
note: required because of the requirements on the impl of `Set<&[_]>` for `{integer}`
   |
   |
LL | impl<'a, T, S> Set<&'a [T]> for S where
   |                ^^^^^^^^^^^^     ^
   = note: 128 redundant requirements hidden
   = note: required because of the requirements on the impl of `Set<&[&[&[&[&[&[&[&[&[&[&[&[&[&[&[&[&[&[&[&[&[&[&[&[&[&[&[&[&[&[&[&[&[&[&[&[&[&[&[&[&[&[&[&[&[&[&[&[&[&[&[&[&[&[&[&[&[&[&[&[&[&[&[&[&[&[&[&[&[&[&[&[&[&[&[&[&[&[&[&[&[&[&[&[&[&[&[&[&[&[&[&[&[&[&[&[&[&[&[&[&[&[&[&[&[&[&[&[&[&[&[&[&[&[&[&[&[&[&[&[&[&[&[&[&[&[&[&[&[_]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]>` for `{integer}`
error: aborting due to previous error

For more information about this error, try `rustc --explain E0275`.


------------------------------------------


---- [ui] ui/issues/issue-20413.rs stdout ----
diff of stderr:

13 LL | impl<T> Foo for T where NoData<T>: Foo {
15    |
15    |
-    = help: consider adding a `#![recursion_limit="256"]` attribute to your crate (`issue_20413`)
+    = help: consider increasing the recursion limit by adding a `#![recursion_limit = "256"]` attribute to your crate (`issue_20413`)
17 note: required because of the requirements on the impl of `Foo` for `NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<T>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>`
19    |


33 LL | impl<T> Foo for T where NoData<T>: Foo {
35    |
35    |
-    = help: consider adding a `#![recursion_limit="256"]` attribute to your crate (`issue_20413`)
+    = help: consider increasing the recursion limit by adding a `#![recursion_limit = "256"]` attribute to your crate (`issue_20413`)
37 note: required because of the requirements on the impl of `Foo` for `NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<T>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>`
39    |


53 LL | impl<T> Bar for T where EvenLessData<T>: Baz {
55    |
55    |
-    = help: consider adding a `#![recursion_limit="256"]` attribute to your crate (`issue_20413`)
+    = help: consider increasing the recursion limit by adding a `#![recursion_limit = "256"]` attribute to your crate (`issue_20413`)
57 note: required because of the requirements on the impl of `Bar` for `AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<T>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>`
59    |


78 LL | impl<T> Bar for T where EvenLessData<T>: Baz {
80    |
80    |
-    = help: consider adding a `#![recursion_limit="256"]` attribute to your crate (`issue_20413`)
+    = help: consider increasing the recursion limit by adding a `#![recursion_limit = "256"]` attribute to your crate (`issue_20413`)
82 note: required because of the requirements on the impl of `Bar` for `AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<T>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>`
84    |


103 LL | impl<T> Baz for T where AlmostNoData<T>: Bar {
105    |
105    |
-    = help: consider adding a `#![recursion_limit="256"]` attribute to your crate (`issue_20413`)
+    = help: consider increasing the recursion limit by adding a `#![recursion_limit = "256"]` attribute to your crate (`issue_20413`)
107 note: required because of the requirements on the impl of `Baz` for `EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<T>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>`
109    |


128 LL | impl<T> Baz for T where AlmostNoData<T>: Bar {
130    |
130    |
-    = help: consider adding a `#![recursion_limit="256"]` attribute to your crate (`issue_20413`)
+    = help: consider increasing the recursion limit by adding a `#![recursion_limit = "256"]` attribute to your crate (`issue_20413`)
132 note: required because of the requirements on the impl of `Baz` for `EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<T>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>`
134    |


The actual stderr differed from the expected stderr.
The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-20413/issue-20413.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args issues/issue-20413.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues/issue-20413.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-20413" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-20413/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0392]: parameter `T` is never used
  --> /checkout/src/test/ui/issues/issue-20413.rs:5:15
   |
LL | struct NoData<T>;
   |               ^ unused parameter
   |
   = help: consider removing `T`, referring to it in a field, or using a marker such as `PhantomData`
   = help: if you intended `T` to be a const parameter, use `const T: usize` instead

error[E0275]: overflow evaluating the requirement `NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<T>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>: Foo`
   |
   |
LL | impl<T> Foo for T where NoData<T>: Foo {
   |
   |
   = help: consider increasing the recursion limit by adding a `#![recursion_limit = "256"]` attribute to your crate (`issue_20413`)
note: required because of the requirements on the impl of `Foo` for `NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<T>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>`
   |
   |
LL | impl<T> Foo for T where NoData<T>: Foo {
   |         ^^^     ^
   = note: 127 redundant requirements hidden
   = note: required because of the requirements on the impl of `Foo` for `NoData<T>`
note: required by a bound in `Foo`
   |
LL | trait Foo {
LL | trait Foo {
   | ^^^^^^^^^ required by this bound in `Foo`

error[E0275]: overflow evaluating the requirement `NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<T>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>: Foo`
   |
   |
LL | impl<T> Foo for T where NoData<T>: Foo {
   |
   |
   = help: consider increasing the recursion limit by adding a `#![recursion_limit = "256"]` attribute to your crate (`issue_20413`)
note: required because of the requirements on the impl of `Foo` for `NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<T>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>`
   |
   |
LL | impl<T> Foo for T where NoData<T>: Foo {
   |         ^^^     ^
   = note: 127 redundant requirements hidden
   = note: required because of the requirements on the impl of `Foo` for `NoData<T>`
note: required by a bound in `Foo`
   |
LL | trait Foo {
LL | trait Foo {
   | ^^^^^^^^^ required by this bound in `Foo`

error[E0275]: overflow evaluating the requirement `EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<T>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>: Baz`
   |
   |
LL | impl<T> Bar for T where EvenLessData<T>: Baz {
   |
   |
   = help: consider increasing the recursion limit by adding a `#![recursion_limit = "256"]` attribute to your crate (`issue_20413`)
note: required because of the requirements on the impl of `Bar` for `AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<T>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>`
   |
   |
LL | impl<T> Bar for T where EvenLessData<T>: Baz {
   |         ^^^     ^
note: required because of the requirements on the impl of `Baz` for `EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<T>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>`
   |
   |
LL | impl<T> Baz for T where AlmostNoData<T>: Bar {
   |         ^^^     ^
   = note: 126 redundant requirements hidden
   = note: required because of the requirements on the impl of `Baz` for `EvenLessData<T>`
note: required by a bound in `Baz`
   |
LL | trait Baz {
LL | trait Baz {
   | ^^^^^^^^^ required by this bound in `Baz`

error[E0275]: overflow evaluating the requirement `EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<T>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>: Baz`
   |
   |
LL | impl<T> Bar for T where EvenLessData<T>: Baz {
   |
   |
   = help: consider increasing the recursion limit by adding a `#![recursion_limit = "256"]` attribute to your crate (`issue_20413`)
note: required because of the requirements on the impl of `Bar` for `AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<T>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>`
   |
   |
LL | impl<T> Bar for T where EvenLessData<T>: Baz {
   |         ^^^     ^
note: required because of the requirements on the impl of `Baz` for `EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<T>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>`
   |
   |
LL | impl<T> Baz for T where AlmostNoData<T>: Bar {
   |         ^^^     ^
   = note: 126 redundant requirements hidden
   = note: required because of the requirements on the impl of `Baz` for `EvenLessData<T>`
note: required by a bound in `Baz`
   |
LL | trait Baz {
LL | trait Baz {
   | ^^^^^^^^^ required by this bound in `Baz`

error[E0275]: overflow evaluating the requirement `AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<T>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>: Bar`
   |
   |
LL | impl<T> Baz for T where AlmostNoData<T>: Bar {
   |
   |
   = help: consider increasing the recursion limit by adding a `#![recursion_limit = "256"]` attribute to your crate (`issue_20413`)
note: required because of the requirements on the impl of `Baz` for `EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<T>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>`
   |
   |
LL | impl<T> Baz for T where AlmostNoData<T>: Bar {
   |         ^^^     ^
note: required because of the requirements on the impl of `Bar` for `AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<T>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>`
   |
   |
LL | impl<T> Bar for T where EvenLessData<T>: Baz {
   |         ^^^     ^
   = note: 126 redundant requirements hidden
   = note: required because of the requirements on the impl of `Bar` for `AlmostNoData<T>`
note: required by a bound in `Bar`
   |
LL | trait Bar {
LL | trait Bar {
   | ^^^^^^^^^ required by this bound in `Bar`

error[E0275]: overflow evaluating the requirement `AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<T>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>: Bar`
   |
   |
LL | impl<T> Baz for T where AlmostNoData<T>: Bar {
   |
   |
   = help: consider increasing the recursion limit by adding a `#![recursion_limit = "256"]` attribute to your crate (`issue_20413`)
note: required because of the requirements on the impl of `Baz` for `EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<T>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>`
   |
   |
LL | impl<T> Baz for T where AlmostNoData<T>: Bar {
   |         ^^^     ^
note: required because of the requirements on the impl of `Bar` for `AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<T>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>`
   |
   |
LL | impl<T> Bar for T where EvenLessData<T>: Baz {
   |         ^^^     ^
   = note: 126 redundant requirements hidden
   = note: required because of the requirements on the impl of `Bar` for `AlmostNoData<T>`
note: required by a bound in `Bar`
   |
LL | trait Bar {
LL | trait Bar {
   | ^^^^^^^^^ required by this bound in `Bar`
error: aborting due to 7 previous errors

Some errors have detailed explanations: E0275, E0392.
For more information about an error, try `rustc --explain E0275`.
For more information about an error, try `rustc --explain E0275`.

------------------------------------------


---- [ui] ui/issues/issue-23122-2.rs stdout ----
diff of stderr:

4 LL |     type Next = <GetNext<T::Next> as Next>::Next;
6    |
6    |
-    = help: consider adding a `#![recursion_limit="256"]` attribute to your crate (`issue_23122_2`)
+    = help: consider increasing the recursion limit by adding a `#![recursion_limit = "256"]` attribute to your crate (`issue_23122_2`)
8 note: required because of the requirements on the impl of `Next` for `GetNext<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<T as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next>`
10    |


The actual stderr differed from the expected stderr.
The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-23122-2/issue-23122-2.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args issues/issue-23122-2.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues/issue-23122-2.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-23122-2" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-23122-2/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0275]: overflow evaluating the requirement `<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<T as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next: Sized`
   |
   |
---

4 LL |     iso(left, right)
5    |     ^^^
6    |
-    = help: consider adding a `#![recursion_limit="256"]` attribute to your crate (`mutual_recursion_issue_75860`)
+    = help: consider increasing the recursion limit by adding a `#![recursion_limit = "256"]` attribute to your crate (`mutual_recursion_issue_75860`)
8 note: required by a bound in `Option`
9   --> $SRC_DIR/core/src/option.rs:LL:COL


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/traits/mutual-recursion-issue-75860/mutual-recursion-issue-75860.stderr
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/traits/mutual-recursion-issue-75860/mutual-recursion-issue-75860.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args traits/mutual-recursion-issue-75860.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/traits/mutual-recursion-issue-75860.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/traits/mutual-recursion-issue-75860" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/traits/mutual-recursion-issue-75860/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0275]: overflow evaluating the requirement `Option<_>: Sized`
   |
LL |     iso(left, right)
   |     ^^^
   |
   |
   = help: consider increasing the recursion limit by adding a `#![recursion_limit = "256"]` attribute to your crate (`mutual_recursion_issue_75860`)
note: required by a bound in `Option`
   |
LL | pub enum Option<T> {
   |                 ^ required by this bound in `Option`

---
test result: FAILED. 12078 passed; 9 failed; 102 ignored; 0 measured; 0 filtered out; finished in 111.62s



command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--suite" "ui" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-10/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python3" "--lldb-python" "/usr/bin/python3" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "10.0.0" "--llvm-components" "aarch64 aarch64asmparser aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets amdgpu amdgpuasmparser amdgpucodegen amdgpudesc amdgpudisassembler amdgpuinfo amdgpuutils analysis arm armasmparser armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter avr avrasmparser avrcodegen avrdesc avrdisassembler avrinfo binaryformat bitreader bitstreamreader bitwriter bpf bpfasmparser bpfcodegen bpfdesc bpfdisassembler bpfinfo cfguard codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfogsym debuginfomsf debuginfopdb demangle dlltooldriver dwarflinker engine executionengine frontendopenmp fuzzmutate globalisel hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interpreter ipo irreader jitlink lanai lanaiasmparser lanaicodegen lanaidesc lanaidisassembler lanaiinfo libdriver lineeditor linker lto mc mca mcdisassembler mcjit mcparser mips mipsasmparser mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmparser msp430codegen msp430desc msp430disassembler msp430info native nativecodegen nvptx nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcerror orcjit passes perfjitevents powerpc powerpcasmparser powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata remarks riscv riscvasmparser riscvcodegen riscvdesc riscvdisassembler riscvinfo riscvutils runtimedyld scalaropts selectiondag sparc sparcasmparser sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target textapi transformutils vectorize webassembly webassemblyasmparser webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo windowsmanifest x86 x86asmparser x86codegen x86desc x86disassembler x86info x86utils xcore xcorecodegen xcoredesc xcoredisassembler xcoreinfo xray" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--channel" "nightly" "--color" "always"


Build completed unsuccessfully in 0:11:06
