plain
.........................................i.ii....................................................... 12600/12646
..............................................
failures:

---- [ui] ui/hr-subtype/placeholder-pattern-fail.rs stdout ----

1 error[E0308]: mismatched types
-   --> $DIR/placeholder-pattern-fail.rs:8:47
+   --> $DIR/placeholder-pattern-fail.rs:9:47
+   --> $DIR/placeholder-pattern-fail.rs:9:47
3    |
4 LL |     let _: for<'a, 'b> fn(Inv<'a>, Inv<'b>) = sub;
5    |                                               ^^^ one type is more general than the other

8               found fn pointer `for<'a> fn(Inv<'a>, Inv<'a>)`
10 error[E0308]: mismatched types
-   --> $DIR/placeholder-pattern-fail.rs:13:31
+   --> $DIR/placeholder-pattern-fail.rs:14:31
12    |
12    |
13 LL |     let _x: (&'static i32,) = x;
14    |                               ^ lifetime mismatch

16    = note: expected tuple `(&'static i32,)`
17               found tuple `(&'c i32,)`
18 note: the lifetime `'c` as defined here...
-   --> $DIR/placeholder-pattern-fail.rs:12:12
+   --> $DIR/placeholder-pattern-fail.rs:13:12
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
20    |
21 LL | fn simple1<'c>(x: (&'c i32,)) {


23    = note: ...does not necessarily outlive the static lifetime
25 error[E0308]: mismatched types
-   --> $DIR/placeholder-pattern-fail.rs:18:30
+   --> $DIR/placeholder-pattern-fail.rs:19:30
27    |
27    |
28 LL |     let _: (&'static i32,) = x;
29    |                              ^ lifetime mismatch

31    = note: expected tuple `(&'static i32,)`
32               found tuple `(&'c i32,)`
33 note: the lifetime `'c` as defined here...
-   --> $DIR/placeholder-pattern-fail.rs:17:12
+   --> $DIR/placeholder-pattern-fail.rs:18:12
35    |
36 LL | fn simple2<'c>(x: (&'c i32,)) {


The actual stderr differed from the expected stderr.
The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/hr-subtype/placeholder-pattern-fail/placeholder-pattern-fail.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args hr-subtype/placeholder-pattern-fail.rs`
error: 1 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/hr-subtype/placeholder-pattern-fail.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/hr-subtype/placeholder-pattern-fail" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/hr-subtype/placeholder-pattern-fail/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0308]: mismatched types
  --> /checkout/src/test/ui/hr-subtype/placeholder-pattern-fail.rs:9:47
   |
LL |     let _: for<'a, 'b> fn(Inv<'a>, Inv<'b>) = sub;
   |                                               ^^^ one type is more general than the other
   |
   = note: expected fn pointer `for<'a, 'b> fn(Inv<'a>, Inv<'b>)`
              found fn pointer `for<'a> fn(Inv<'a>, Inv<'a>)`
error[E0308]: mismatched types
  --> /checkout/src/test/ui/hr-subtype/placeholder-pattern-fail.rs:14:31
   |
   |
LL |     let _x: (&'static i32,) = x;
   |                               ^ lifetime mismatch
   |
   = note: expected tuple `(&'static i32,)`
              found tuple `(&'c i32,)`
note: the lifetime `'c` as defined here...
  --> /checkout/src/test/ui/hr-subtype/placeholder-pattern-fail.rs:13:12
   |
LL | fn simple1<'c>(x: (&'c i32,)) {
   |            ^^
   = note: ...does not necessarily outlive the static lifetime
error[E0308]: mismatched types
  --> /checkout/src/test/ui/hr-subtype/placeholder-pattern-fail.rs:19:30
   |
   |
LL |     let _: (&'static i32,) = x;
   |                              ^ lifetime mismatch
   |
   = note: expected tuple `(&'static i32,)`
              found tuple `(&'c i32,)`
note: the lifetime `'c` as defined here...
  --> /checkout/src/test/ui/hr-subtype/placeholder-pattern-fail.rs:18:12
   |
LL | fn simple2<'c>(x: (&'c i32,)) {
   |            ^^
   = note: ...does not necessarily outlive the static lifetime
error: aborting due to 3 previous errors

For more information about this error, try `rustc --explain E0308`.

