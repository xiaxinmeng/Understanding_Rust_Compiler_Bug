plain
---- [ui] src/test/ui/custom_test_frameworks/full.rs stdout ----

error: test compilation failed although it shouldn't!
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/custom_test_frameworks/full.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "-O" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/custom_test_frameworks/full/a" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--test" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/custom_test_frameworks/full/auxiliary"
stdout: none
--- stderr -------------------------------
error: malformed `rustc_test_marker` attribute input
   |
LL | #[test_case]
LL | #[test_case]
   | ^^^^^^^^^^^^ help: must be of the form: `#[rustc_test_marker = "name"]`
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
   = note: this error originates in the attribute macro `test_case` (in Nightly builds, run with -Z macro-backtrace for more info)

error: malformed `rustc_test_marker` attribute input
error: malformed `rustc_test_marker` attribute input
  --> /checkout/src/test/ui/custom_test_frameworks/full.rs:27:1
   |
LL | #[test_case]
   | ^^^^^^^^^^^^ help: must be of the form: `#[rustc_test_marker = "name"]`
   = note: this error originates in the attribute macro `test_case` (in Nightly builds, run with -Z macro-backtrace for more info)

error: aborting due to 2 previous errors
------------------------------------------
------------------------------------------


---- [ui] src/test/ui/custom_test_frameworks/dynamic.rs stdout ----

error: test compilation failed although it shouldn't!
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/custom_test_frameworks/dynamic.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "-O" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/custom_test_frameworks/dynamic/a" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--test" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/custom_test_frameworks/dynamic/auxiliary"
stdout: none
--- stderr -------------------------------
error: malformed `rustc_test_marker` attribute input
   |
LL | #[test_case]
LL | #[test_case]
   | ^^^^^^^^^^^^ help: must be of the form: `#[rustc_test_marker = "name"]`
   = note: this error originates in the attribute macro `test_case` (in Nightly builds, run with -Z macro-backtrace for more info)

error: aborting due to previous error
------------------------------------------
------------------------------------------


---- [ui] src/test/ui/custom-test-frameworks-simple.rs stdout ----

error: test compilation failed although it shouldn't!
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/custom-test-frameworks-simple.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "-O" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/custom-test-frameworks-simple/a" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--test" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/custom-test-frameworks-simple/auxiliary"
stdout: none
--- stderr -------------------------------
error: malformed `rustc_test_marker` attribute input
   |
LL | #[test_case]
LL | #[test_case]
   | ^^^^^^^^^^^^ help: must be of the form: `#[rustc_test_marker = "name"]`
   = note: this error originates in the attribute macro `test_case` (in Nightly builds, run with -Z macro-backtrace for more info)

error: malformed `rustc_test_marker` attribute input
  --> /checkout/src/test/ui/custom-test-frameworks-simple.rs:19:1
  --> /checkout/src/test/ui/custom-test-frameworks-simple.rs:19:1
   |
LL | #[test_case]
   | ^^^^^^^^^^^^ help: must be of the form: `#[rustc_test_marker = "name"]`
   = note: this error originates in the attribute macro `test_case` (in Nightly builds, run with -Z macro-backtrace for more info)

error: aborting due to 2 previous errors
------------------------------------------
