plain

---- [ui] src/test/ui/span/lint-unused-unsafe-thir.rs stdout ----
diff of stderr:

22 LL | unsafe fn bad3() { unsafe {} }
23    | ----------------   ^^^^^^ unnecessary `unsafe` block
24    | |
-    | because it's nested under this `unsafe` fn
+    | because it's nested under this `unsafe` block
26 
27 error: unnecessary `unsafe` block


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/span/lint-unused-unsafe-thir/lint-unused-unsafe-thir.stderr
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/span/lint-unused-unsafe-thir/lint-unused-unsafe-thir.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args span/lint-unused-unsafe-thir.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/span/lint-unused-unsafe-thir.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/span/lint-unused-unsafe-thir" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Zthir-unsafeck" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/span/lint-unused-unsafe-thir/auxiliary"
stdout: none
--- stderr -------------------------------
error: unnecessary `unsafe` block
   |
   |
LL | fn bad1() { unsafe {} }                  //~ ERROR: unnecessary `unsafe` block
   |             ^^^^^^ unnecessary `unsafe` block
note: the lint level is defined here
  --> /checkout/src/test/ui/span/lint-unused-unsafe-thir.rs:9:9
   |
LL | #![deny(unused_unsafe)]
LL | #![deny(unused_unsafe)]
   |         ^^^^^^^^^^^^^

error: unnecessary `unsafe` block
   |
   |
LL | fn bad2() { unsafe { bad1() } }          //~ ERROR: unnecessary `unsafe` block
   |             ^^^^^^ unnecessary `unsafe` block

error: unnecessary `unsafe` block
   |
   |
LL | unsafe fn bad3() { unsafe {} }           //~ ERROR: unnecessary `unsafe` block
   | ----------------   ^^^^^^ unnecessary `unsafe` block
   | |
   | because it's nested under this `unsafe` block

error: unnecessary `unsafe` block
   |
   |
LL | fn bad4() { unsafe { callback(||{}) } }  //~ ERROR: unnecessary `unsafe` block
   |             ^^^^^^ unnecessary `unsafe` block

error: unnecessary `unsafe` block
   |
   |
LL |     unsafe {                             // don't put the warning here
   |     ------ because it's nested under this `unsafe` block
LL |         unsafe {                         //~ ERROR: unnecessary `unsafe` block
   |         ^^^^^^ unnecessary `unsafe` block

error: unnecessary `unsafe` block
   |
LL |     unsafe {
LL |     unsafe {
   |     ------ because it's nested under this `unsafe` block
LL |         unsafe {                         //~ ERROR: unnecessary `unsafe` block
   |         ^^^^^^ unnecessary `unsafe` block
error: aborting due to 6 previous errors
------------------------------------------


