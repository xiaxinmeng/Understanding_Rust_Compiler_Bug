plain

---- [ui] src/test/ui/coercion/coerce-issue-49593-box-never.rs#nofallback stdout ----
diff of stderr:

4 LL |     /* *mut $0 is coerced to Box<dyn Error> here */ Box::<_ /* ! */>::new(x)
5    |                                                     ^^^^^^^^^^^^^^^^^^^^^^^^ the trait `std::error::Error` is not implemented for `()`
-    = help: the following other types implement trait `std::error::Error`:
-              !
-              &'a T
-              AccessError
---
18 
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
19 error[E0277]: the trait bound `(): std::error::Error` is not satisfied

22 LL |     /* *mut $0 is coerced to *mut Error here */ raw_ptr_box::<_ /* ! */>(x)
23    |                                                 ^^^^^^^^^^^^^^^^^^^^^^^^^^^ the trait `std::error::Error` is not implemented for `()`
-    = help: the following other types implement trait `std::error::Error`:
-              !
-              &'a T
-              AccessError
---
37 error: aborting due to 2 previous errors


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/coercion/coerce-issue-49593-box-never.nofallback/coerce-issue-49593-box-never.nofallback.stderr
To only update this specific test, also pass `--test-args coercion/coerce-issue-49593-box-never.rs`


error in revision `nofallback`: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/coercion/coerce-issue-49593-box-never.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "nofallback" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/coercion/coerce-issue-49593-box-never.nofallback" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/coercion/coerce-issue-49593-box-never.nofallback/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0277]: the trait bound `(): std::error::Error` is not satisfied
   |
   |
LL |     /* *mut $0 is coerced to Box<dyn Error> here */ Box::<_ /* ! */>::new(x)
   |                                                     ^^^^^^^^^^^^^^^^^^^^^^^^ the trait `std::error::Error` is not implemented for `()`
   = note: required for the cast to the object type `dyn std::error::Error`

error[E0277]: the trait bound `(): std::error::Error` is not satisfied
  --> /checkout/src/test/ui/coercion/coerce-issue-49593-box-never.rs:22:49
  --> /checkout/src/test/ui/coercion/coerce-issue-49593-box-never.rs:22:49
   |
LL |     /* *mut $0 is coerced to *mut Error here */ raw_ptr_box::<_ /* ! */>(x)
   |                                                 ^^^^^^^^^^^^^^^^^^^^^^^^^^^ the trait `std::error::Error` is not implemented for `()`
   = note: required for the cast to the object type `(dyn std::error::Error + 'static)`

error: aborting due to 2 previous errors

