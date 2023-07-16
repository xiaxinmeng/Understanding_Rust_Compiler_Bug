plain
....................................................

failures:

---- [ui] tests/ui/lint/dead-code/offset-of.rs stdout ----


1 error: field `b` is never read
-   --> $DIR/liveness-offset-of.rs:8:5
+   --> $DIR/offset-of.rs:8:5
4 LL | struct Alpha {
5    |        ----- field in this struct

8    |     ^
---
24 error: field `a` is never read
-   --> $DIR/liveness-offset-of.rs:18:5
+   --> $DIR/offset-of.rs:18:5
26    |
27 LL | struct Gamma {

30    |     ^
31 
31 
32 error: field `b` is never read
-   --> $DIR/liveness-offset-of.rs:24:5
+   --> $DIR/offset-of.rs:24:5
35 LL | struct Delta {
36    |        ----- field in this struct

39    |     ^
39    |     ^
40 
41 error: field `a` is never read
-   --> $DIR/liveness-offset-of.rs:35:5
+   --> $DIR/offset-of.rs:35:5
43    |
44 LL | struct Project<T: Trait> {


The actual stderr differed from the expected stderr.
The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/dead-code/offset-of/offset-of.stderr
To only update this specific test, also pass `--test-args lint/dead-code/offset-of.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/tests/ui/lint/dead-code/offset-of.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--remap-path-prefix=/checkout/tests/ui=fake-test-src-base" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/dead-code/offset-of" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/dead-code/offset-of/auxiliary"
stdout: none
--- stderr -------------------------------
error: field `b` is never read
  --> fake-test-src-base/lint/dead-code/offset-of.rs:8:5
LL | struct Alpha {
   |        ----- field in this struct
   |        ----- field in this struct
LL |     a: (),
LL |     b: (), //~ ERROR field `b` is never read
   |
note: the lint level is defined here
note: the lint level is defined here
  --> fake-test-src-base/lint/dead-code/offset-of.rs:2:9
LL | #![deny(dead_code)]
   |         ^^^^^^^^^

error: field `a` is never read
error: field `a` is never read
  --> fake-test-src-base/lint/dead-code/offset-of.rs:13:5
LL | struct Beta {
   |        ---- field in this struct
   |        ---- field in this struct
LL |     a: (), //~ ERROR field `a` is never read

error: field `a` is never read
error: field `a` is never read
  --> fake-test-src-base/lint/dead-code/offset-of.rs:18:5
   |
LL | struct Gamma {
   |        ----- field in this struct
LL |     a: (), //~ ERROR field `a` is never read


error: field `b` is never read
  --> fake-test-src-base/lint/dead-code/offset-of.rs:24:5
LL | struct Delta {
   |        ----- field in this struct
   |        ----- field in this struct
LL |     a: (),
LL |     b: (), //~ ERROR field `b` is never read

error: field `a` is never read
error: field `a` is never read
  --> fake-test-src-base/lint/dead-code/offset-of.rs:35:5
   |
LL | struct Project<T: Trait> {
   |        ------- field in this struct
LL |     a: u8, //~ ERROR field `a` is never read

error: aborting due to 5 previous errors
------------------------------------------

