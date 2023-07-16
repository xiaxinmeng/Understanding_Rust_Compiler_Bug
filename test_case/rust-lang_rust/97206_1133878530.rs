plain
test [ui] src/test/ui/lto/thin-lto-global-allocator.rs ... ok
test [ui] src/test/ui/lto/thin-lto-inlines.rs ... ok
test [ui] src/test/ui/lub-glb/old-lub-glb-hr-eq.rs ... ok
test [ui] src/test/ui/lub-glb/old-lub-glb-hr-noteq1.rs#baseleak ... ok
test [ui] src/test/ui/lub-glb/old-lub-glb-hr-noteq1.rs#basenoleak ... ok
test [ui] src/test/ui/lub-glb/old-lub-glb-hr-noteq1.rs#nllnoleak ... ok
test [ui] src/test/ui/lto/weak-works.rs ... ok
test [ui] src/test/ui/lub-glb/old-lub-glb-hr-noteq1.rs#nllleak ... ok
test [ui] src/test/ui/lub-glb/old-lub-glb-hr-noteq2.rs#baseleak ... ok
test [ui] src/test/ui/lub-glb/old-lub-glb-hr-noteq2.rs#baseleak ... ok
test [ui] src/test/ui/lub-glb/old-lub-glb-hr-noteq2.rs#basenoleak ... ok
test [ui] src/test/ui/lub-glb/old-lub-glb-hr-noteq2.rs#nllleak ... ok
test [ui] src/test/ui/lub-glb/old-lub-glb-hr-noteq2.rs#nllnoleak ... ok
test [ui] src/test/ui/macros/ambiguity-legacy-vs-modern.rs ... ok
test [ui] src/test/ui/macro_backtrace/main.rs#-Zmacro-backtrace ... ok
test [ui] src/test/ui/lto/lto-duplicate-symbols.rs ... ok
test [ui] src/test/ui/macro_backtrace/main.rs#default ... ok
---
test [ui (nll)] src/test/ui/logging-only-prints-once.rs ... ok
test [ui (nll)] src/test/ui/lto/dylib-works.rs ... ok
test [ui (nll)] src/test/ui/lub-glb/old-lub-glb-hr-eq.rs ... ok
test [ui (nll)] src/test/ui/lub-glb/old-lub-glb-hr-noteq1.rs#baseleak ... ignored
test [ui (nll)] src/test/ui/lub-glb/old-lub-glb-hr-noteq1.rs#basenoleak ... ignored
test [ui (nll)] src/test/ui/lub-glb/old-lub-glb-hr-noteq1.rs#nllleak ... ignored
test [ui (nll)] src/test/ui/lub-glb/old-lub-glb-hr-noteq1.rs#nllnoleak ... ignored
test [ui (nll)] src/test/ui/lub-glb/old-lub-glb-hr-noteq2.rs#baseleak ... ignored
test [ui (nll)] src/test/ui/lub-glb/old-lub-glb-hr-noteq2.rs#basenoleak ... ignored
test [ui (nll)] src/test/ui/lub-glb/old-lub-glb-hr-noteq2.rs#nllleak ... ignored
test [ui (nll)] src/test/ui/lub-glb/old-lub-glb-hr-noteq2.rs#nllnoleak ... ignored
test [ui (nll)] src/test/ui/lto/thin-lto-global-allocator.rs ... ok
test [ui (nll)] src/test/ui/lub-glb/old-lub-glb-object.rs ... ok
test [ui (nll)] src/test/ui/lto/msvc-imp-present.rs ... ok
test [ui (nll)] src/test/ui/lto/weak-works.rs ... ok
---
1 error[E0308]: mismatched types
-   --> $DIR/placeholder-pattern-fail.rs:9:12
+   --> $DIR/placeholder-pattern-fail.rs:9:47
3    |
4 LL |     let _: for<'a, 'b> fn(Inv<'a>, Inv<'b>) = sub;
-    |            ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ one type is more general than the other
+    |                                               ^^^ one type is more general than the other
6    |
7    = note: expected fn pointer `for<'a, 'b> fn(Inv<'a>, Inv<'b>)`
8               found fn pointer `for<'a> fn(Inv<'a>, Inv<'a>)`
9 
- error[E0308]: mismatched types
-   --> $DIR/placeholder-pattern-fail.rs:9:12
-    |
-    |
- LL |     let _: for<'a, 'b> fn(Inv<'a>, Inv<'b>) = sub;
-    |            ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ one type is more general than the other
-    |
-    = note: expected fn pointer `for<'a, 'b> fn(Inv<'a>, Inv<'b>)`
-               found fn pointer `for<'a> fn(Inv<'a>, Inv<'a>)`
- error: lifetime may not live long enough
-   --> $DIR/placeholder-pattern-fail.rs:14:13
-    |
-    |
- LL | fn simple1<'c>(x: (&'c i32,)) {
-    |            -- lifetime `'c` defined here
- LL |     let _x: (&'static i32,) = x;
-    |             ^^^^^^^^^^^^^^^ type annotation requires that `'c` must outlive `'static`
- error: lifetime may not live long enough
-   --> $DIR/placeholder-pattern-fail.rs:19:12
-    |
-    |
- LL | fn simple2<'c>(x: (&'c i32,)) {
-    |            -- lifetime `'c` defined here
- LL |     let _: (&'static i32,) = x;
-    |            ^^^^^^^^^^^^^^^ type annotation requires that `'c` must outlive `'static`
- error: aborting due to 4 previous errors
+ error: aborting due to previous error
36 
37 For more information about this error, try `rustc --explain E0308`.
37 For more information about this error, try `rustc --explain E0308`.
38 


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/hr-subtype/placeholder-pattern-fail.nll/placeholder-pattern-fail.nll.stderr
To only update this specific test, also pass `--test-args hr-subtype/placeholder-pattern-fail.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/hr-subtype/placeholder-pattern-fail.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/hr-subtype/placeholder-pattern-fail.nll" "-Zborrowck=mir" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/hr-subtype/placeholder-pattern-fail.nll/auxiliary"
stdout: none
--- stderr -------------------------------
  --> /checkout/src/test/ui/hr-subtype/placeholder-pattern-fail.rs:9:47
   |
   |
LL |     let _: for<'a, 'b> fn(Inv<'a>, Inv<'b>) = sub;
   |                                               ^^^ one type is more general than the other
   |
   = note: expected fn pointer `for<'a, 'b> fn(Inv<'a>, Inv<'b>)`
              found fn pointer `for<'a> fn(Inv<'a>, Inv<'a>)`
error: aborting due to previous error

For more information about this error, try `rustc --explain E0308`.
------------------------------------------
---
    [ui (nll)] src/test/ui/hr-subtype/placeholder-pattern-fail.rs

test result: FAILED. 12761 passed; 1 failed; 365 ignored; 0 measured; 0 filtered out; finished in 104.82s

Some tests failed in compiletest suite=ui compare_mode=Nll mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
