
---- [run-pass] run-pass/cross-crate/cci_capture_clause.rs stdout ----

error: auxiliary build of "/checkout/src/test/run-pass/cross-crate/auxiliary/cci_capture_clause.rs" failed to compile: 
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/run-pass/cross-crate/auxiliary/cci_capture_clause.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/cross-crate/cci_capture_clause/auxiliary" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--crate-type" "dylib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/cross-crate/cci_capture_clause/auxiliary"
stdout:
------------------------------------------

------------------------------------------
stderr:
------------------------------------------
error: unused `std::result::Result` that must be used
  --> /checkout/src/test/run-pass/cross-crate/auxiliary/cci_capture_clause.rs:7:9
   |
LL |         tx.send(x.clone());
   |         ^^^^^^^^^^^^^^^^^^^
   |
   = note: `#[deny(unused_must_use)]` on by default
   = note: this `Result` may be an `Err` variant, which should be handled

error: aborting due to previous error


------------------------------------------


---- [run-pass] run-pass/issues/issue-2723-b.rs stdout ----

error: auxiliary build of "/checkout/src/test/run-pass/issues/auxiliary/issue-2723-a.rs" failed to compile: 
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/run-pass/issues/auxiliary/issue-2723-a.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/issues/issue-2723-b/auxiliary" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--crate-type" "dylib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/issues/issue-2723-b/auxiliary"
stdout:
------------------------------------------

------------------------------------------
stderr:
------------------------------------------
warning: function is never used: `q`
  --> /checkout/src/test/run-pass/issues/auxiliary/issue-2723-a.rs:2:26
   |
LL |     xs.iter().map(|_x| { unsafe fn q() { panic!(); } }).collect::<Vec<()>>();
   |                          ^^^^^^^^^^^^^
   |
   = note: `#[warn(dead_code)]` on by default

error: unused return value of `std::iter::Iterator::collect` that must be used
  --> /checkout/src/test/run-pass/issues/auxiliary/issue-2723-a.rs:2:5
   |
LL |     xs.iter().map(|_x| { unsafe fn q() { panic!(); } }).collect::<Vec<()>>();
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
   |
   = note: `#[deny(unused_must_use)]` on by default
   = note: if you really need to exhaust the iterator, consider `.for_each(drop)` instead

error: aborting due to previous error


------------------------------------------


---- [run-pass] run-pass/issues/issue-9906.rs stdout ----

error: auxiliary build of "/checkout/src/test/run-pass/issues/auxiliary/issue-9906.rs" failed to compile: 
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/run-pass/issues/auxiliary/issue-9906.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/issues/issue-9906/auxiliary" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--crate-type" "dylib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/issues/issue-9906/auxiliary"
stdout:
------------------------------------------

------------------------------------------
stderr:
------------------------------------------
warning: field is never used: `value`
  --> /checkout/src/test/run-pass/issues/auxiliary/issue-9906.rs:5:23
   |
LL |     pub struct FooBar{value: isize}
   |                       ^^^^^^^^^^^^
   |
   = note: `#[warn(dead_code)]` on by default

error: unused arithmetic operation that must be used
  --> /checkout/src/test/run-pass/issues/auxiliary/issue-9906.rs:13:9
   |
LL |         1+1;
   |         ^^^
   |
   = note: `#[deny(unused_must_use)]` on by default

error: aborting due to previous error


------------------------------------------



failures:
    [run-pass] run-pass/cross-crate/cci_capture_clause.rs
    [run-pass] run-pass/issues/issue-2723-b.rs
    [run-pass] run-pass/issues/issue-9906.rs
