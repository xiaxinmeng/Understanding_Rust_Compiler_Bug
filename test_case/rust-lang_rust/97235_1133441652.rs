plain
---- [ui] src/test/ui/impl-trait/auto-trait-leak.rs stdout ----
diff of stderr:

19    |
20 LL | fn cycle1() -> impl Clone {
21    | ^^^^^^^^^^^^^^^^^^^^^^^^^
- note: ...which requires unsafety-checking `cycle1`...
+ note: ...which requires check if `cycle1` contains FFI-unwind calls...
24    |
24    |
25 LL | fn cycle1() -> impl Clone {
54    |
54    |
55 LL | fn cycle2() -> impl Clone {
56    | ^^^^^^^^^^^^^^^^^^^^^^^^^
- note: ...which requires unsafety-checking `cycle2`...
+ note: ...which requires check if `cycle2` contains FFI-unwind calls...
59    |
59    |
60 LL | fn cycle2() -> impl Clone {
103    |
103    |
104 LL | fn cycle1() -> impl Clone {
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
- note: ...which requires unsafety-checking `cycle1`...
+ note: ...which requires check if `cycle1` contains FFI-unwind calls...
108    |
108    |
109 LL | fn cycle1() -> impl Clone {
138    |
138    |
139 LL | fn cycle2() -> impl Clone {
140    | ^^^^^^^^^^^^^^^^^^^^^^^^^
- note: ...which requires unsafety-checking `cycle2`...
+ note: ...which requires check if `cycle2` contains FFI-unwind calls...
143    |
143    |
144 LL | fn cycle2() -> impl Clone {

The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/impl-trait/auto-trait-leak/auto-trait-leak.stderr
To update references, rerun the tests and pass the `--bless` flag
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args impl-trait/auto-trait-leak.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/impl-trait/auto-trait-leak.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/impl-trait/auto-trait-leak" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/impl-trait/auto-trait-leak/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0391]: cycle detected when computing type of `cycle1::{opaque#0}`
   |
   |
LL | fn cycle1() -> impl Clone {
   |
   |
note: ...which requires borrow-checking `cycle1`...
   |
   |
LL | fn cycle1() -> impl Clone {
   | ^^^^^^^^^^^^^^^^^^^^^^^^^
note: ...which requires processing `cycle1`...
   |
   |
LL | fn cycle1() -> impl Clone {
   | ^^^^^^^^^^^^^^^^^^^^^^^^^
note: ...which requires processing MIR for `cycle1`...
   |
   |
LL | fn cycle1() -> impl Clone {
   | ^^^^^^^^^^^^^^^^^^^^^^^^^
note: ...which requires check if `cycle1` contains FFI-unwind calls...
   |
   |
LL | fn cycle1() -> impl Clone {
   | ^^^^^^^^^^^^^^^^^^^^^^^^^
note: ...which requires building MIR for `cycle1`...
   |
   |
LL | fn cycle1() -> impl Clone {
   | ^^^^^^^^^^^^^^^^^^^^^^^^^
note: ...which requires type-checking `cycle1`...
   |
   |
LL | fn cycle1() -> impl Clone {
   | ^^^^^^^^^^^^^^^^^^^^^^^^^
note: ...which requires computing type of `cycle2::{opaque#0}`...
   |
   |
LL | fn cycle2() -> impl Clone {
   |                ^^^^^^^^^^
note: ...which requires borrow-checking `cycle2`...
   |
   |
LL | fn cycle2() -> impl Clone {
   | ^^^^^^^^^^^^^^^^^^^^^^^^^
note: ...which requires processing `cycle2`...
   |
   |
LL | fn cycle2() -> impl Clone {
   | ^^^^^^^^^^^^^^^^^^^^^^^^^
note: ...which requires processing MIR for `cycle2`...
   |
   |
LL | fn cycle2() -> impl Clone {
   | ^^^^^^^^^^^^^^^^^^^^^^^^^
note: ...which requires check if `cycle2` contains FFI-unwind calls...
   |
   |
LL | fn cycle2() -> impl Clone {
   | ^^^^^^^^^^^^^^^^^^^^^^^^^
note: ...which requires building MIR for `cycle2`...
   |
   |
LL | fn cycle2() -> impl Clone {
   | ^^^^^^^^^^^^^^^^^^^^^^^^^
note: ...which requires type-checking `cycle2`...
   |
   |
LL | fn cycle2() -> impl Clone {
   | ^^^^^^^^^^^^^^^^^^^^^^^^^
   = note: ...which again requires computing type of `cycle1::{opaque#0}`, completing the cycle
note: cycle used when checking item types in top-level module
   |
LL | / use std::cell::Cell;
LL | / use std::cell::Cell;
LL | | use std::rc::Rc;
LL | |
LL | | fn send<T: Send>(_: T) {}
...  |
LL | |     Rc::new(String::from("foo"))
LL | | }


error[E0391]: cycle detected when computing type of `cycle1::{opaque#0}`
   |
   |
LL | fn cycle1() -> impl Clone {
   |
   |
note: ...which requires borrow-checking `cycle1`...
   |
   |
LL | fn cycle1() -> impl Clone {
   | ^^^^^^^^^^^^^^^^^^^^^^^^^
note: ...which requires processing `cycle1`...
   |
   |
LL | fn cycle1() -> impl Clone {
   | ^^^^^^^^^^^^^^^^^^^^^^^^^
note: ...which requires processing MIR for `cycle1`...
   |
   |
LL | fn cycle1() -> impl Clone {
   | ^^^^^^^^^^^^^^^^^^^^^^^^^
note: ...which requires check if `cycle1` contains FFI-unwind calls...
   |
   |
LL | fn cycle1() -> impl Clone {
   | ^^^^^^^^^^^^^^^^^^^^^^^^^
note: ...which requires building MIR for `cycle1`...
   |
   |
LL | fn cycle1() -> impl Clone {
   | ^^^^^^^^^^^^^^^^^^^^^^^^^
note: ...which requires type-checking `cycle1`...
   |
   |
LL | fn cycle1() -> impl Clone {
   | ^^^^^^^^^^^^^^^^^^^^^^^^^
note: ...which requires computing type of `cycle2::{opaque#0}`...
   |
   |
LL | fn cycle2() -> impl Clone {
   |                ^^^^^^^^^^
note: ...which requires borrow-checking `cycle2`...
   |
   |
LL | fn cycle2() -> impl Clone {
   | ^^^^^^^^^^^^^^^^^^^^^^^^^
note: ...which requires processing `cycle2`...
   |
   |
LL | fn cycle2() -> impl Clone {
   | ^^^^^^^^^^^^^^^^^^^^^^^^^
note: ...which requires processing MIR for `cycle2`...
   |
   |
LL | fn cycle2() -> impl Clone {
   | ^^^^^^^^^^^^^^^^^^^^^^^^^
note: ...which requires check if `cycle2` contains FFI-unwind calls...
   |
   |
LL | fn cycle2() -> impl Clone {
   | ^^^^^^^^^^^^^^^^^^^^^^^^^
note: ...which requires building MIR for `cycle2`...
   |
   |
LL | fn cycle2() -> impl Clone {
   | ^^^^^^^^^^^^^^^^^^^^^^^^^
note: ...which requires type-checking `cycle2`...
   |
   |
LL | fn cycle2() -> impl Clone {
   | ^^^^^^^^^^^^^^^^^^^^^^^^^
   = note: ...which again requires computing type of `cycle1::{opaque#0}`, completing the cycle
note: cycle used when checking item types in top-level module
   |
LL | / use std::cell::Cell;
LL | / use std::cell::Cell;
LL | | use std::rc::Rc;
LL | |
LL | | fn send<T: Send>(_: T) {}
...  |
LL | |     Rc::new(String::from("foo"))
LL | | }

error: aborting due to 2 previous errors

For more information about this error, try `rustc --explain E0391`.
