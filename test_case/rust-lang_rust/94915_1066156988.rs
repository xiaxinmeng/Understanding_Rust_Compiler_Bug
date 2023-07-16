plain
---- [ui] ui/mir/ssa-analysis-regression-50041.rs stdout ----

error: test compilation failed although it shouldn't!
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/mir/ssa-analysis-regression-50041.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/mir/ssa-analysis-regression-50041" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "mir-opt-level=4" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/mir/ssa-analysis-regression-50041/auxiliary"
stdout: none
--- stderr -------------------------------
  --> /checkout/src/test/ui/mir/ssa-analysis-regression-50041.rs:19:5
   |
LL |     dealloc(ptr)
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
   |     ^^^^^^^ --- supplied 1 argument
   |     |
   |     expected 2 arguments
   |
note: function defined here
  --> /checkout/src/test/ui/mir/ssa-analysis-regression-50041.rs:23:4
   |
LL | fn dealloc<T: ?Sized>(_: *mut T, _: ()) {

error: aborting due to previous error

For more information about this error, try `rustc --explain E0061`.
