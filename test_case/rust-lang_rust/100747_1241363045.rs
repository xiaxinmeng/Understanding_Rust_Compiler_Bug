plain
........................................................................................ 6688/13497
............................................i........................................... 6776/13497
.............ii.ii........i....i........................................................ 6864/13497
.......i................................................................................ 6952/13497
.....................................................................i....i...F......... 7040/13497
................................i....................................................... 7216/13497
.....................................................i.................................. 7304/13497
........................................................................................ 7392/13497
........................................................................................ 7480/13497
---
16    |     ^^^^^^^^^^^^^^^^^^^^^
17 help: consider adding an explicit lifetime bound...
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
18    |
- LL | fn no_restriction<T: 'a>(x: &()) -> &() {
-    |                    ++++
+ LL | fn no_restriction<'a, T: 'a>(x: &()) -> &() {
+    |                   +++  ++++
22 error: aborting due to previous error
23 



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/error-codes/E0311/E0311.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args error-codes/E0311.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/error-codes/E0311.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/error-codes/E0311" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/error-codes/E0311/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0311]: the parameter type `T` may not live long enough
   |
   |
LL |     with_restriction::<T>(x) //~ ERROR E0311
   |
   |
note: the parameter type `T` must be valid for the anonymous lifetime defined here...
   |
   |
LL | fn no_restriction<T>(x: &()) -> &() {
   |                         ^^^
note: ...so that the type `T` will meet its required lifetime bounds
   |
   |
LL |     with_restriction::<T>(x) //~ ERROR E0311
help: consider adding an explicit lifetime bound...
   |
   |
LL | fn no_restriction<'a, T: 'a>(x: &()) -> &() {
   |                   +++  ++++
error: aborting due to previous error

For more information about this error, try `rustc --explain E0311`.
------------------------------------------
---
To only update this specific test, also pass `--test-args lifetimes/suggest-introducing-and-adding-missing-lifetime.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/lifetimes/suggest-introducing-and-adding-missing-lifetime.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lifetimes/suggest-introducing-and-adding-missing-lifetime" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lifetimes/suggest-introducing-and-adding-missing-lifetime/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0311]: the parameter type `T` may not live long enough
   |
   |
LL |     with_restriction::<T>(x) //~ ERROR the parameter type `T` may not live long enough
   |
   |
note: the parameter type `T` must be valid for the anonymous lifetime defined here...
   |
   |
LL | fn no_restriction<T>(x: &()) -> &() {
   |                         ^^^
note: ...so that the type `T` will meet its required lifetime bounds
   |
   |
LL |     with_restriction::<T>(x) //~ ERROR the parameter type `T` may not live long enough
help: consider adding an explicit lifetime bound...
   |
   |
LL | fn no_restriction<'a, T: 'a>(x: &()) -> &() {
   |                   +++  ++++
error: aborting due to previous error

For more information about this error, try `rustc --explain E0311`.
------------------------------------------
