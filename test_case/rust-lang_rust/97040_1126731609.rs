plain
  TOOLSTATE_REPO: https://github.com/rust-lang-nursery/rust-toolstate
  CACHE_DOMAIN: ci-caches.rust-lang.org
  IMAGE: x86_64-gnu-llvm-12
##[endgroup]
fatal: unknown commit 53fd98ca776cb875bc9e5514f56b52eb74f9e7a9
All commits in `HEAD` are present in `master`
src/ci/scripts/verify-stable-version-number.sh
shell: /usr/bin/bash --noprofile --norc -e -o pipefail {0}
env:
  CI_JOB_NAME: x86_64-gnu-llvm-12
---
......................................................iii........................................... 12800/12859
...........................................................
failures:

---- [ui] ui/impl-trait/nested-rpit-hrtb.rs stdout ----

- error[E0261]: use of undeclared lifetime name `'b`
-   --> $DIR/nested-rpit-hrtb.rs:54:77
-    |
-    |
- LL | fn two_htrb_outlives() -> impl for<'a> Foo<'a, Assoc = impl for<'b> Sized + 'b> {}
-    |                                                                             ^^ undeclared lifetime
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
-    = note: for more information on higher-ranked polymorphism, visit https://doc.rust-lang.org/nomicon/hrtb.html
- help: consider making the bound lifetime-generic with a new `'b` lifetime
-    |
- LL | fn two_htrb_outlives() -> impl for<'b, 'a> Foo<'a, Assoc = impl for<'b> Sized + 'b> {}
-    |                                    +++
- help: consider introducing lifetime `'b` here
-    |
- LL | fn two_htrb_outlives<'b>() -> impl for<'a> Foo<'a, Assoc = impl for<'b> Sized + 'b> {}
- 
- error[E0261]: use of undeclared lifetime name `'b`
-   --> $DIR/nested-rpit-hrtb.rs:61:82
-    |
-    |
- LL | fn two_htrb_outlives_uses() -> impl for<'a> Bar<'a, Assoc = impl for<'b> Sized + 'b> {}
-    |                                                                                  ^^ undeclared lifetime
-    |
- help: consider making the bound lifetime-generic with a new `'b` lifetime
-    |
- LL | fn two_htrb_outlives_uses() -> impl for<'b, 'a> Bar<'a, Assoc = impl for<'b> Sized + 'b> {}
-    |                                         +++
- help: consider introducing lifetime `'b` here
-    |
- LL | fn two_htrb_outlives_uses<'b>() -> impl for<'a> Bar<'a, Assoc = impl for<'b> Sized + 'b> {}
- 
- 
32 error: higher kinded lifetime bounds on nested opaque types are not supported yet
33   --> $DIR/nested-rpit-hrtb.rs:25:69


77 LL | fn one_hrtb_trait_param_uses() -> impl for<'a> Bar<'a, Assoc = impl Qux<'a>> {}
79 
- error: aborting due to 6 previous errors
- error: aborting due to 6 previous errors
+ error[E0261]: use of undeclared lifetime name `'b`
+   --> $DIR/nested-rpit-hrtb.rs:54:77
+    |
+ LL | fn two_htrb_outlives() -> impl for<'a> Foo<'a, Assoc = impl for<'b> Sized + 'b> {}
+    |                     - help: consider introducing lifetime `'b` here: `<'b>` ^^ undeclared lifetime
- For more information about this error, try `rustc --explain E0261`.
- For more information about this error, try `rustc --explain E0261`.
+ error[E0496]: lifetime name `'b` shadows a lifetime name that is already in scope
+   --> $DIR/nested-rpit-hrtb.rs:54:65
+    |
+ LL | fn two_htrb_outlives() -> impl for<'a> Foo<'a, Assoc = impl for<'b> Sized + 'b> {}
+    |                                                                 ^^          -- first declared here
+    |                                                                 |
+    |                                                                 lifetime `'b` already in scope
+ 
+ error[E0261]: use of undeclared lifetime name `'b`
+   --> $DIR/nested-rpit-hrtb.rs:61:82
+    |
+ LL | fn two_htrb_outlives_uses() -> impl for<'a> Bar<'a, Assoc = impl for<'b> Sized + 'b> {}
+    |                          - help: consider introducing lifetime `'b` here: `<'b>` ^^ undeclared lifetime
+ 
+ error[E0496]: lifetime name `'b` shadows a lifetime name that is already in scope
+   --> $DIR/nested-rpit-hrtb.rs:61:70
+    |
+ LL | fn two_htrb_outlives_uses() -> impl for<'a> Bar<'a, Assoc = impl for<'b> Sized + 'b> {}
+    |                                                                      ^^          -- first declared here
+    |                                                                      |
+    |                                                                      lifetime `'b` already in scope
+ error: aborting due to 8 previous errors
+ 
+ Some errors have detailed explanations: E0261, E0496.
+ For more information about an error, try `rustc --explain E0261`.
+ For more information about an error, try `rustc --explain E0261`.
83 


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/impl-trait/nested-rpit-hrtb/nested-rpit-hrtb.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args impl-trait/nested-rpit-hrtb.rs`
error: 1 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/impl-trait/nested-rpit-hrtb.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/impl-trait/nested-rpit-hrtb" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/impl-trait/nested-rpit-hrtb/auxiliary"
stdout: none
--- stderr -------------------------------
error: higher kinded lifetime bounds on nested opaque types are not supported yet
  --> /checkout/src/test/ui/impl-trait/nested-rpit-hrtb.rs:25:69
   |
LL | fn one_hrtb_outlives() -> impl for<'a> Foo<'a, Assoc = impl Sized + 'a> {}
   |
note: lifetime declared here
  --> /checkout/src/test/ui/impl-trait/nested-rpit-hrtb.rs:25:36
   |
   |
LL | fn one_hrtb_outlives() -> impl for<'a> Foo<'a, Assoc = impl Sized + 'a> {}


error: higher kinded lifetime bounds on nested opaque types are not supported yet
  --> /checkout/src/test/ui/impl-trait/nested-rpit-hrtb.rs:29:68
   |
LL | fn one_hrtb_trait_param() -> impl for<'a> Foo<'a, Assoc = impl Qux<'a>> {}
   |
note: lifetime declared here
  --> /checkout/src/test/ui/impl-trait/nested-rpit-hrtb.rs:29:39
   |
   |
LL | fn one_hrtb_trait_param() -> impl for<'a> Foo<'a, Assoc = impl Qux<'a>> {}


error: higher kinded lifetime bounds on nested opaque types are not supported yet
  --> /checkout/src/test/ui/impl-trait/nested-rpit-hrtb.rs:32:74
   |
LL | fn one_hrtb_outlives_uses() -> impl for<'a> Bar<'a, Assoc = impl Sized + 'a> {}
   |
note: lifetime declared here
  --> /checkout/src/test/ui/impl-trait/nested-rpit-hrtb.rs:32:41
   |
   |
LL | fn one_hrtb_outlives_uses() -> impl for<'a> Bar<'a, Assoc = impl Sized + 'a> {}


error: higher kinded lifetime bounds on nested opaque types are not supported yet
  --> /checkout/src/test/ui/impl-trait/nested-rpit-hrtb.rs:35:73
   |
LL | fn one_hrtb_trait_param_uses() -> impl for<'a> Bar<'a, Assoc = impl Qux<'a>> {}
   |
note: lifetime declared here
  --> /checkout/src/test/ui/impl-trait/nested-rpit-hrtb.rs:35:44
   |
   |
LL | fn one_hrtb_trait_param_uses() -> impl for<'a> Bar<'a, Assoc = impl Qux<'a>> {}

error[E0261]: use of undeclared lifetime name `'b`
  --> /checkout/src/test/ui/impl-trait/nested-rpit-hrtb.rs:54:77
   |
   |
LL | fn two_htrb_outlives() -> impl for<'a> Foo<'a, Assoc = impl for<'b> Sized + 'b> {}
   |                     - help: consider introducing lifetime `'b` here: `<'b>` ^^ undeclared lifetime

error[E0496]: lifetime name `'b` shadows a lifetime name that is already in scope
  --> /checkout/src/test/ui/impl-trait/nested-rpit-hrtb.rs:54:65
   |
LL | fn two_htrb_outlives() -> impl for<'a> Foo<'a, Assoc = impl for<'b> Sized + 'b> {}
   |                                                                 ^^          -- first declared here
   |                                                                 |
   |                                                                 lifetime `'b` already in scope
error[E0261]: use of undeclared lifetime name `'b`
  --> /checkout/src/test/ui/impl-trait/nested-rpit-hrtb.rs:61:82
   |
   |
LL | fn two_htrb_outlives_uses() -> impl for<'a> Bar<'a, Assoc = impl for<'b> Sized + 'b> {}
   |                          - help: consider introducing lifetime `'b` here: `<'b>` ^^ undeclared lifetime

error[E0496]: lifetime name `'b` shadows a lifetime name that is already in scope
  --> /checkout/src/test/ui/impl-trait/nested-rpit-hrtb.rs:61:70
   |
LL | fn two_htrb_outlives_uses() -> impl for<'a> Bar<'a, Assoc = impl for<'b> Sized + 'b> {}
   |                                                                      ^^          -- first declared here
   |                                                                      |
   |                                                                      lifetime `'b` already in scope
error: aborting due to 8 previous errors

Some errors have detailed explanations: E0261, E0496.
For more information about an error, try `rustc --explain E0261`.
