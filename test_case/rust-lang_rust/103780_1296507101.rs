plain

1 note: external requirements
2   --> $DIR/nested-closures-regions.rs:8:24
3    |
- LL |     for<'a> || -> () { for<'c> |_: &'c &'a ()| -> () {}; };
-    |                        ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
+ LL |     for<'a> || -> () { for<'c> |_: &'a ()| -> () {}; };
6    |
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
7    = note: defining type: main::{closure#0}::{closure#0} with closure substs [


-                for<'c> extern "rust-call" fn((&'c &(),)),
+                extern "rust-call" fn((&(),)),
10                (),
12    = note: late-bound region is '_#4r

18 note: no external requirements
19   --> $DIR/nested-closures-regions.rs:8:5
19   --> $DIR/nested-closures-regions.rs:8:5
20    |
- LL |     for<'a> || -> () { for<'c> |_: &'c &'a ()| -> () {}; };
+ LL |     for<'a> || -> () { for<'c> |_: &'a ()| -> () {}; };
23    |
23    |
24    = note: defining type: main::{closure#0} with closure substs [

The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/closures/binder/nested-closures-regions/nested-closures-regions.stderr
To update references, rerun the tests and pass the `--bless` flag
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args closures/binder/nested-closures-regions.rs`

error: 1 errors occurred comparing output.
status: exit status: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/closures/binder/nested-closures-regions.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/closures/binder/nested-closures-regions" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/closures/binder/nested-closures-regions/auxiliary"
stdout: none
--- stderr -------------------------------
  --> /checkout/src/test/ui/closures/binder/nested-closures-regions.rs:8:24
   |
   |
LL |     for<'a> || -> () { for<'c> |_: &'a ()| -> () {}; };
   |
   |
   = note: defining type: main::{closure#0}::{closure#0} with closure substs [
               i8,
               extern "rust-call" fn((&(),)),
               (),
   = note: late-bound region is '_#4r
   = note: late-bound region is '_#2r
   = note: late-bound region is '_#2r
   = note: number of external vids: 3
   = note: where '_#1r: '_#2r
   = note: where '_#2r: '_#1r
note: no external requirements
  --> /checkout/src/test/ui/closures/binder/nested-closures-regions.rs:8:5
   |
   |
LL |     for<'a> || -> () { for<'c> |_: &'a ()| -> () {}; };
   |
   |
   = note: defining type: main::{closure#0} with closure substs [
               i8,
               extern "rust-call" fn(()),
               (),
   = note: late-bound region is '_#2r

note: no external requirements
  --> /checkout/src/test/ui/closures/binder/nested-closures-regions.rs:7:1
