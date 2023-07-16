plain

---- [ui] src/test/ui/unsafe/rfc-2585-unsafe_op_in_unsafe_fn.rs#mir stdout ----
diff of stderr:

62    = note: consult the function's documentation for information on how to avoid undefined behavior
63 help: consider wrapping the function in an unsafe block
64    |
- LL ~ unsafe fn warning_level() { /* FIXME */ unsafe {
+ LL ~ unsafe fn warning_level() { unsafe {
66 LL |     unsf();
68 LL |


The actual stderr differed from the expected stderr.
The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/unsafe/rfc-2585-unsafe_op_in_unsafe_fn.mir/rfc-2585-unsafe_op_in_unsafe_fn.mir.stderr
To only update this specific test, also pass `--test-args unsafe/rfc-2585-unsafe_op_in_unsafe_fn.rs`


error in revision `mir`: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/unsafe/rfc-2585-unsafe_op_in_unsafe_fn.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "mir" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/unsafe/rfc-2585-unsafe_op_in_unsafe_fn.mir" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/unsafe/rfc-2585-unsafe_op_in_unsafe_fn.mir/auxiliary"
stdout: none
--- stderr -------------------------------
error: call to unsafe function is unsafe and requires unsafe block (error E0133)
   |
LL |     unsf();
   |     ^^^^^^ call to unsafe function
   |
   |
note: the lint level is defined here
  --> /checkout/src/test/ui/unsafe/rfc-2585-unsafe_op_in_unsafe_fn.rs:4:9
   |
LL | #![deny(unsafe_op_in_unsafe_fn)]
   |         ^^^^^^^^^^^^^^^^^^^^^^
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
   = note: consult the function's documentation for information on how to avoid undefined behavior
help: consider wrapping the function in an unsafe block
   |
LL ~ unsafe fn deny_level() { unsafe {
LL |     unsf();
 ...
LL |     //~^ ERROR unnecessary `unsafe` block
LL ~ }}


error: dereference of raw pointer is unsafe and requires unsafe block (error E0133)
   |
LL |     *PTR;
   |     ^^^^ dereference of raw pointer
   |
   |
   = note: raw pointers may be null, dangling or unaligned; they can violate aliasing rules and cause data races: all of these are undefined behavior

error: use of mutable static is unsafe and requires unsafe block (error E0133)
   |
LL |     VOID = ();
   |     ^^^^^^^^^ use of mutable static
   |
   |
   = note: mutable statics can be mutated by multiple threads: aliasing violations or data races will cause undefined behavior

error: unnecessary `unsafe` block
   |
LL |     unsafe {}
LL |     unsafe {}
   |     ^^^^^^ unnecessary `unsafe` block
note: the lint level is defined here
  --> /checkout/src/test/ui/unsafe/rfc-2585-unsafe_op_in_unsafe_fn.rs:5:9
   |
LL | #![deny(unused_unsafe)]
LL | #![deny(unused_unsafe)]
   |         ^^^^^^^^^^^^^

error: call to unsafe function is unsafe and requires unsafe block (error E0133)
   |
LL |     unsf();
   |     ^^^^^^ call to unsafe function
   |
   |
note: the lint level is defined here
  --> /checkout/src/test/ui/unsafe/rfc-2585-unsafe_op_in_unsafe_fn.rs:26:8
   |
LL | #[deny(warnings)]
   |        ^^^^^^^^
   = note: `#[deny(unsafe_op_in_unsafe_fn)]` implied by `#[deny(warnings)]`
   = note: consult the function's documentation for information on how to avoid undefined behavior
help: consider wrapping the function in an unsafe block
   |
LL ~ unsafe fn warning_level() { unsafe {
LL |     unsf();
 ...
LL |     //~^ ERROR unnecessary `unsafe` block
LL ~ }}


error: dereference of raw pointer is unsafe and requires unsafe block (error E0133)
   |
LL |     *PTR;
   |     ^^^^ dereference of raw pointer
   |
   |
   = note: raw pointers may be null, dangling or unaligned; they can violate aliasing rules and cause data races: all of these are undefined behavior

error: use of mutable static is unsafe and requires unsafe block (error E0133)
   |
LL |     VOID = ();
   |     ^^^^^^^^^ use of mutable static
   |
   |
   = note: mutable statics can be mutated by multiple threads: aliasing violations or data races will cause undefined behavior

error: unnecessary `unsafe` block
   |
LL |     unsafe {}
LL |     unsafe {}
   |     ^^^^^^ unnecessary `unsafe` block

error: unnecessary `unsafe` block
   |
   |
LL |     unsafe { unsafe { unsf() } }
   |     ^^^^^^ unnecessary `unsafe` block

error: unnecessary `unsafe` block
   |
   |
LL | unsafe fn allow_level() {
   | ----------------------- because it's nested under this `unsafe` fn
...
LL |     unsafe { unsf() }
   |     ^^^^^^ unnecessary `unsafe` block
   |
   = note: this `unsafe` block does contain unsafe operations, but those are already allowed in an `unsafe fn`
  --> /checkout/src/test/ui/unsafe/rfc-2585-unsafe_op_in_unsafe_fn.rs:53:9
   |
LL | #[allow(unsafe_op_in_unsafe_fn)]
   |         ^^^^^^^^^^^^^^^^^^^^^^
   |         ^^^^^^^^^^^^^^^^^^^^^^

error: unnecessary `unsafe` block
   |
   |
LL | unsafe fn nested_allow_level() {
   | ------------------------------ because it's nested under this `unsafe` fn
...
LL |         unsafe { unsf() }
   |         ^^^^^^ unnecessary `unsafe` block
   |
   = note: this `unsafe` block does contain unsafe operations, but those are already allowed in an `unsafe fn`
  --> /checkout/src/test/ui/unsafe/rfc-2585-unsafe_op_in_unsafe_fn.rs:65:13
   |
LL |     #[allow(unsafe_op_in_unsafe_fn)]
   |             ^^^^^^^^^^^^^^^^^^^^^^
   |             ^^^^^^^^^^^^^^^^^^^^^^

error[E0133]: call to unsafe function is unsafe and requires unsafe block
  --> /checkout/src/test/ui/unsafe/rfc-2585-unsafe_op_in_unsafe_fn.rs:78:5
   |
LL |     unsf();
   |     ^^^^^^ call to unsafe function
   |
   = note: consult the function's documentation for information on how to avoid undefined behavior
error[E0133]: call to unsafe function is unsafe and requires unsafe function or block
  --> /checkout/src/test/ui/unsafe/rfc-2585-unsafe_op_in_unsafe_fn.rs:83:9
   |
LL |         unsf();
LL |         unsf();
   |         ^^^^^^ call to unsafe function
   |
   = note: consult the function's documentation for information on how to avoid undefined behavior
error: aborting due to 13 previous errors

For more information about this error, try `rustc --explain E0133`.
------------------------------------------
