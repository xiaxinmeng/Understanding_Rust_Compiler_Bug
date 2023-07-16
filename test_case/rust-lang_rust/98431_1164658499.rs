plain
8    |
9    = note:           expected type `char`
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
10            found mutable reference `&mut _`
+ note: to declare a mutable binding use `mut variable_name`: `mut c`
+    |
+    |
+ LL |     for ((_, _), (&mut c, _)) in &mut map {
+    |                   ^^^^^^
11 help: consider removing `&mut` from the pattern
12    |
13 LL -     for ((_, _), (&mut c, _)) in &mut map {

The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/pattern/for-loop-bad-item/for-loop-bad-item.stderr
To update references, rerun the tests and pass the `--bless` flag
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args pattern/for-loop-bad-item.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/pattern/for-loop-bad-item.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/pattern/for-loop-bad-item" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/pattern/for-loop-bad-item/auxiliary"
stdout: none
--- stderr -------------------------------
  --> /checkout/src/test/ui/pattern/for-loop-bad-item.rs:7:19
   |
   |
LL |     for ((_, _), (&mut c, _)) in &mut map {
   |                   ^^^^^^         -------- this is an iterator with items of type `(&(char, char), &mut (char, char))`
   |                   expected `char`, found `&mut _`
   |
   = note:           expected type `char`
           found mutable reference `&mut _`
           found mutable reference `&mut _`
note: to declare a mutable binding use `mut variable_name`: `mut c`
   |
   |
LL |     for ((_, _), (&mut c, _)) in &mut map {
   |                   ^^^^^^
help: consider removing `&mut` from the pattern
   |
LL -     for ((_, _), (&mut c, _)) in &mut map {
LL +     for ((_, _), (c, _)) in &mut map {

error[E0308]: mismatched types
  --> /checkout/src/test/ui/pattern/for-loop-bad-item.rs:14:14
   |
   |
LL |     for Some(Qux(_)) | None in [Some(""), None] {
   |              ^^^^^^            ---------------- this is an iterator with items of type `Option<&str>`
   |              |
   |              expected `str`, found struct `Qux`
error: aborting due to 2 previous errors

For more information about this error, try `rustc --explain E0308`.
------------------------------------------
