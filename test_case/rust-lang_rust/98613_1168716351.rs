plain

---- [ui] src/test/ui/associated-types/hr-associated-type-bound-2.rs stdout ----
diff of stderr:

- error[E0275]: overflow evaluating the requirement `for<'b> u32: X<'b>`
+ error[E0275]: overflow evaluating the requirement `<u32 as X<'b>>::U`
2   --> $DIR/hr-associated-type-bound-2.rs:11:1
3    |
4 LL | / impl X<'_> for u32

The actual stderr differed from the expected stderr.
The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/associated-types/hr-associated-type-bound-2/hr-associated-type-bound-2.stderr
To only update this specific test, also pass `--test-args associated-types/hr-associated-type-bound-2.rs`

error: 1 errors occurred comparing output.
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/associated-types/hr-associated-type-bound-2.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/associated-types/hr-associated-type-bound-2" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/associated-types/hr-associated-type-bound-2/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0275]: overflow evaluating the requirement `<u32 as X<'b>>::U`
   |
   |
LL | / impl X<'_> for u32 //~ overflow evaluating the requirement `for<'b> u32: X<'b>`
LL | | where
LL | |     for<'b> <Self as X<'b>>::U: Clone,
LL | | {
LL | |     type U = str;
LL | | }
   |
   |
   = help: consider increasing the recursion limit by adding a `#![recursion_limit = "256"]` attribute to your crate (`hr_associated_type_bound_2`)
note: required because of the requirements on the impl of `for<'b> X<'b>` for `u32`
   |
   |
LL | impl X<'_> for u32 //~ overflow evaluating the requirement `for<'b> u32: X<'b>`
   = note: 128 redundant requirements hidden
   = note: 128 redundant requirements hidden
   = note: required because of the requirements on the impl of `for<'b> X<'b>` for `u32`
error: aborting due to previous error

For more information about this error, try `rustc --explain E0275`.
------------------------------------------
------------------------------------------


---- [ui] src/test/ui/associated-types/project-recursion-limit-non-fatal.rs stdout ----

error: test compilation failed although it shouldn't!
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/associated-types/project-recursion-limit-non-fatal.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/associated-types/project-recursion-limit-non-fatal" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/associated-types/project-recursion-limit-non-fatal/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0275]: overflow evaluating the requirement `<Self as FilterDsl<_>>::Output`
   |
   = help: consider increasing the recursion limit by adding a `#![recursion_limit = "256"]` attribute to your crate (`project_recursion_limit_non_fatal`)
note: required because of the requirements on the impl of `HandleDelete` for `Self`
   |
   |
LL | impl<T> HandleDelete for T

error: aborting due to previous error

For more information about this error, try `rustc --explain E0275`.
For more information about this error, try `rustc --explain E0275`.
------------------------------------------


---- [ui] src/test/ui/issues/issue-23122-2.rs stdout ----
diff of stderr:

- error[E0275]: overflow evaluating the requirement `<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<T as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next: Sized`
+ error[E0275]: overflow evaluating the requirement `<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<T as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next`
3    |
3    |
4 LL |     type Next = <GetNext<T::Next> as Next>::Next;
5    |                 ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
6    |
6    |
7    = help: consider increasing the recursion limit by adding a `#![recursion_limit = "256"]` attribute to your crate (`issue_23122_2`)
- note: required because of the requirements on the impl of `Next` for `GetNext<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<T as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next>`
-   --> $DIR/issue-23122-2.rs:9:15
-    |
- LL | impl<T: Next> Next for GetNext<T> {
13 
14 error: aborting due to previous error
15 

---
To only update this specific test, also pass `--test-args issues/issue-23122-2.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues/issue-23122-2.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-23122-2" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-23122-2/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0275]: overflow evaluating the requirement `<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<T as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next`
   |
   |
LL |     type Next = <GetNext<T::Next> as Next>::Next;
   |
   |
   = help: consider increasing the recursion limit by adding a `#![recursion_limit = "256"]` attribute to your crate (`issue_23122_2`)
error: aborting due to previous error

For more information about this error, try `rustc --explain E0275`.
------------------------------------------
------------------------------------------


---- [ui] src/test/ui/recursion/issue-83150.rs stdout ----
diff of stderr:

9    = note: `#[warn(unconditional_recursion)]` on by default
10    = help: a `loop` may express intention better if this is on purpose
11 
- error[E0275]: overflow evaluating the requirement `Map<&mut Map<&mut Map<&mut Map<&mut Map<&mut Map<&mut Map<&mut Map<&mut Map<&mut Map<&mut Map<&mut Map<&mut Map<&mut Map<&mut Map<&mut Map<&mut Map<&mut Map<&mut Map<&mut Map<&mut Map<&mut Map<&mut Map<&mut Map<&mut Map<&mut Map<&mut Map<&mut Map<&mut Map<&mut Map<&mut Map<&mut Map<&mut Map<&mut Map<&mut Map<&mut Map<&mut Map<&mut Map<&mut Map<&mut Map<&mut Map<&mut Map<&mut Map<&mut Map<&mut Map<&mut Map<&mut Map<&mut Map<&mut Map<&mut Map<&mut Map<&mut Map<&mut Map<&mut Map<&mut Map<&mut Map<&mut Map<&mut Map<&mut Map<&mut Map<&mut Map<&mut Map<&mut Map<&mut Map<&mut std::ops::Range<u8>, [closure@$DIR/issue-83150.rs:10:24: 10:33]>, [closure@$DIR/issue-83150.rs:10:24: 10:33]>, [closure@$DIR/issue-83150.rs:10:24: 10:33]>, [closure@$DIR/issue-83150.rs:10:24: 10:33]>, [closure@$DIR/issue-83150.rs:10:24: 10:33]>, [closure@$DIR/issue-83150.rs:10:24: 10:33]>, [closure@$DIR/issue-83150.rs:10:24: 10:33]>, [closure@$DIR/issue-83150.rs:10:24: 10:33]>, [closure@$DIR/issue-83150.rs:10:24: 10:33]>, [closure@$DIR/issue-83150.rs:10:24: 10:33]>, [closure@$DIR/issue-83150.rs:10:24: 10:33]>, [closure@$DIR/issue-83150.rs:10:24: 10:33]>, [closure@$DIR/issue-83150.rs:10:24: 10:33]>, [closure@$DIR/issue-83150.rs:10:24: 10:33]>, [closure@$DIR/issue-83150.rs:10:24: 10:33]>, [closure@$DIR/issue-83150.rs:10:24: 10:33]>, [closure@$DIR/issue-83150.rs:10:24: 10:33]>, [closure@$DIR/issue-83150.rs:10:24: 10:33]>, [closure@$DIR/issue-83150.rs:10:24: 10:33]>, [closure@$DIR/issue-83150.rs:10:24: 10:33]>, [closure@$DIR/issue-83150.rs:10:24: 10:33]>, [closure@$DIR/issue-83150.rs:10:24: 10:33]>, [closure@$DIR/issue-83150.rs:10:24: 10:33]>, [closure@$DIR/issue-83150.rs:10:24: 10:33]>, [closure@$DIR/issue-83150.rs:10:24: 10:33]>, [closure@$DIR/issue-83150.rs:10:24: 10:33]>, [closure@$DIR/issue-83150.rs:10:24: 10:33]>, [closure@$DIR/issue-83150.rs:10:24: 10:33]>, [closure@$DIR/issue-83150.rs:10:24: 10:33]>, [closure@$DIR/issue-83150.rs:10:24: 10:33]>, [closure@$DIR/issue-83150.rs:10:24: 10:33]>, [closure@$DIR/issue-83150.rs:10:24: 10:33]>, [closure@$DIR/issue-83150.rs:10:24: 10:33]>, [closure@$DIR/issue-83150.rs:10:24: 10:33]>, [closure@$DIR/issue-83150.rs:10:24: 10:33]>, [closure@$DIR/issue-83150.rs:10:24: 10:33]>, [closure@$DIR/issue-83150.rs:10:24: 10:33]>, [closure@$DIR/issue-83150.rs:10:24: 10:33]>, [closure@$DIR/issue-83150.rs:10:24: 10:33]>, [closure@$DIR/issue-83150.rs:10:24: 10:33]>, [closure@$DIR/issue-83150.rs:10:24: 10:33]>, [closure@$DIR/issue-83150.rs:10:24: 10:33]>, [closure@$DIR/issue-83150.rs:10:24: 10:33]>, [closure@$DIR/issue-83150.rs:10:24: 10:33]>, [closure@$DIR/issue-83150.rs:10:24: 10:33]>, [closure@$DIR/issue-83150.rs:10:24: 10:33]>, [closure@$DIR/issue-83150.rs:10:24: 10:33]>, [closure@$DIR/issue-83150.rs:10:24: 10:33]>, [closure@$DIR/issue-83150.rs:10:24: 10:33]>, [closure@$DIR/issue-83150.rs:10:24: 10:33]>, [closure@$DIR/issue-83150.rs:10:24: 10:33]>, [closure@$DIR/issue-83150.rs:10:24: 10:33]>, [closure@$DIR/issue-83150.rs:10:24: 10:33]>, [closure@$DIR/issue-83150.rs:10:24: 10:33]>, [closure@$DIR/issue-83150.rs:10:24: 10:33]>, [closure@$DIR/issue-83150.rs:10:24: 10:33]>, [closure@$DIR/issue-83150.rs:10:24: 10:33]>, [closure@$DIR/issue-83150.rs:10:24: 10:33]>, [closure@$DIR/issue-83150.rs:10:24: 10:33]>, [closure@$DIR/issue-83150.rs:10:24: 10:33]>, [closure@$DIR/issue-83150.rs:10:24: 10:33]>, [closure@$DIR/issue-83150.rs:10:24: 10:33]>, [closure@$DIR/issue-83150.rs:10:24: 10:33]>, [closure@$DIR/issue-83150.rs:10:24: 10:33]>: Iterator`
+ error[E0275]: overflow evaluating the requirement `<std::ops::Range<u8> as Iterator>::Item`
13    |
14    = help: consider increasing the recursion limit by adding a `#![recursion_limit = "256"]` attribute to your crate (`issue_83150`)
+    = note: required because of the requirements on the impl of `Iterator` for `Map<&mut std::ops::Range<u8>, [closure@$DIR/issue-83150.rs:10:24: 10:33]>`
+    = note: 64 redundant requirements hidden
15    = note: required because of the requirements on the impl of `Iterator` for `&mut Map<&mut Map<&mut Map<&mut Map<&mut Map<&mut Map<&mut Map<&mut Map<&mut Map<&mut Map<&mut Map<&mut Map<&mut Map<&mut Map<&mut Map<&mut Map<&mut Map<&mut Map<&mut Map<&mut Map<&mut Map<&mut Map<&mut Map<&mut Map<&mut Map<&mut Map<&mut Map<&mut Map<&mut Map<&mut Map<&mut Map<&mut Map<&mut Map<&mut Map<&mut Map<&mut Map<&mut Map<&mut Map<&mut Map<&mut Map<&mut Map<&mut Map<&mut Map<&mut Map<&mut Map<&mut Map<&mut Map<&mut Map<&mut Map<&mut Map<&mut Map<&mut Map<&mut Map<&mut Map<&mut Map<&mut Map<&mut Map<&mut Map<&mut Map<&mut Map<&mut Map<&mut Map<&mut Map<&mut Map<&mut std::ops::Range<u8>, [closure@$DIR/issue-83150.rs:10:24: 10:33]>, [closure@$DIR/issue-83150.rs:10:24: 10:33]>, [closure@$DIR/issue-83150.rs:10:24: 10:33]>, [closure@$DIR/issue-83150.rs:10:24: 10:33]>, [closure@$DIR/issue-83150.rs:10:24: 10:33]>, [closure@$DIR/issue-83150.rs:10:24: 10:33]>, [closure@$DIR/issue-83150.rs:10:24: 10:33]>, [closure@$DIR/issue-83150.rs:10:24: 10:33]>, [closure@$DIR/issue-83150.rs:10:24: 10:33]>, [closure@$DIR/issue-83150.rs:10:24: 10:33]>, [closure@$DIR/issue-83150.rs:10:24: 10:33]>, [closure@$DIR/issue-83150.rs:10:24: 10:33]>, [closure@$DIR/issue-83150.rs:10:24: 10:33]>, [closure@$DIR/issue-83150.rs:10:24: 10:33]>, [closure@$DIR/issue-83150.rs:10:24: 10:33]>, [closure@$DIR/issue-83150.rs:10:24: 10:33]>, [closure@$DIR/issue-83150.rs:10:24: 10:33]>, [closure@$DIR/issue-83150.rs:10:24: 10:33]>, [closure@$DIR/issue-83150.rs:10:24: 10:33]>, [closure@$DIR/issue-83150.rs:10:24: 10:33]>, [closure@$DIR/issue-83150.rs:10:24: 10:33]>, [closure@$DIR/issue-83150.rs:10:24: 10:33]>, [closure@$DIR/issue-83150.rs:10:24: 10:33]>, [closure@$DIR/issue-83150.rs:10:24: 10:33]>, [closure@$DIR/issue-83150.rs:10:24: 10:33]>, [closure@$DIR/issue-83150.rs:10:24: 10:33]>, [closure@$DIR/issue-83150.rs:10:24: 10:33]>, [closure@$DIR/issue-83150.rs:10:24: 10:33]>, [closure@$DIR/issue-83150.rs:10:24: 10:33]>, [closure@$DIR/issue-83150.rs:10:24: 10:33]>, [closure@$DIR/issue-83150.rs:10:24: 10:33]>, [closure@$DIR/issue-83150.rs:10:24: 10:33]>, [closure@$DIR/issue-83150.rs:10:24: 10:33]>, [closure@$DIR/issue-83150.rs:10:24: 10:33]>, [closure@$DIR/issue-83150.rs:10:24: 10:33]>, [closure@$DIR/issue-83150.rs:10:24: 10:33]>, [closure@$DIR/issue-83150.rs:10:24: 10:33]>, [closure@$DIR/issue-83150.rs:10:24: 10:33]>, [closure@$DIR/issue-83150.rs:10:24: 10:33]>, [closure@$DIR/issue-83150.rs:10:24: 10:33]>, [closure@$DIR/issue-83150.rs:10:24: 10:33]>, [closure@$DIR/issue-83150.rs:10:24: 10:33]>, [closure@$DIR/issue-83150.rs:10:24: 10:33]>, [closure@$DIR/issue-83150.rs:10:24: 10:33]>, [closure@$DIR/issue-83150.rs:10:24: 10:33]>, [closure@$DIR/issue-83150.rs:10:24: 10:33]>, [closure@$DIR/issue-83150.rs:10:24: 10:33]>, [closure@$DIR/issue-83150.rs:10:24: 10:33]>, [closure@$DIR/issue-83150.rs:10:24: 10:33]>, [closure@$DIR/issue-83150.rs:10:24: 10:33]>, [closure@$DIR/issue-83150.rs:10:24: 10:33]>, [closure@$DIR/issue-83150.rs:10:24: 10:33]>, [closure@$DIR/issue-83150.rs:10:24: 10:33]>, [closure@$DIR/issue-83150.rs:10:24: 10:33]>, [closure@$DIR/issue-83150.rs:10:24: 10:33]>, [closure@$DIR/issue-83150.rs:10:24: 10:33]>, [closure@$DIR/issue-83150.rs:10:24: 10:33]>, [closure@$DIR/issue-83150.rs:10:24: 10:33]>, [closure@$DIR/issue-83150.rs:10:24: 10:33]>, [closure@$DIR/issue-83150.rs:10:24: 10:33]>, [closure@$DIR/issue-83150.rs:10:24: 10:33]>, [closure@$DIR/issue-83150.rs:10:24: 10:33]>, [closure@$DIR/issue-83150.rs:10:24: 10:33]>, [closure@$DIR/issue-83150.rs:10:24: 10:33]>`
17 error: aborting due to previous error; 1 warning emitted


The actual stderr differed from the expected stderr.
The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/recursion/issue-83150/issue-83150.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args recursion/issue-83150.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/recursion/issue-83150.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/recursion/issue-83150" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/recursion/issue-83150/auxiliary"
stdout: none
--- stderr -------------------------------
warning: function cannot return without recursing
   |
   |
LL | fn func<T: Iterator<Item = u8>>(iter: &mut T) { //~ WARN function cannot return without recursing
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ cannot return without recursing
LL |     func(&mut iter.map(|x| x + 1))
   |     ------------------------------ recursive call site
   = note: `#[warn(unconditional_recursion)]` on by default
   = note: `#[warn(unconditional_recursion)]` on by default
   = help: a `loop` may express intention better if this is on purpose

error[E0275]: overflow evaluating the requirement `<std::ops::Range<u8> as Iterator>::Item`
   |
   = help: consider increasing the recursion limit by adding a `#![recursion_limit = "256"]` attribute to your crate (`issue_83150`)
   = note: required because of the requirements on the impl of `Iterator` for `Map<&mut std::ops::Range<u8>, [closure@/checkout/src/test/ui/recursion/issue-83150.rs:10:24: 10:33]>`
   = note: 64 redundant requirements hidden
   = note: required because of the requirements on the impl of `Iterator` for `&mut Map<&mut Map<&mut Map<&mut Map<&mut Map<&mut Map<&mut Map<&mut Map<&mut Map<&mut Map<&mut Map<&mut Map<&mut Map<&mut Map<&mut Map<&mut Map<&mut Map<&mut Map<&mut Map<&mut Map<&mut Map<&mut Map<&mut Map<&mut Map<&mut Map<&mut Map<&mut Map<&mut Map<&mut Map<&mut Map<&mut Map<&mut Map<&mut Map<&mut Map<&mut Map<&mut Map<&mut Map<&mut Map<&mut Map<&mut Map<&mut Map<&mut Map<&mut Map<&mut Map<&mut Map<&mut Map<&mut Map<&mut Map<&mut Map<&mut Map<&mut Map<&mut Map<&mut Map<&mut Map<&mut Map<&mut Map<&mut Map<&mut Map<&mut Map<&mut Map<&mut Map<&mut Map<&mut Map<&mut Map<&mut std::ops::Range<u8>, [closure@/checkout/src/test/ui/recursion/issue-83150.rs:10:24: 10:33]>, [closure@/checkout/src/test/ui/recursion/issue-83150.rs:10:24: 10:33]>, [closure@/checkout/src/test/ui/recursion/issue-83150.rs:10:24: 10:33]>, [closure@/checkout/src/test/ui/recursion/issue-83150.rs:10:24: 10:33]>, [closure@/checkout/src/test/ui/recursion/issue-83150.rs:10:24: 10:33]>, [closure@/checkout/src/test/ui/recursion/issue-83150.rs:10:24: 10:33]>, [closure@/checkout/src/test/ui/recursion/issue-83150.rs:10:24: 10:33]>, [closure@/checkout/src/test/ui/recursion/issue-83150.rs:10:24: 10:33]>, [closure@/checkout/src/test/ui/recursion/issue-83150.rs:10:24: 10:33]>, [closure@/checkout/src/test/ui/recursion/issue-83150.rs:10:24: 10:33]>, [closure@/checkout/src/test/ui/recursion/issue-83150.rs:10:24: 10:33]>, [closure@/checkout/src/test/ui/recursion/issue-83150.rs:10:24: 10:33]>, [closure@/checkout/src/test/ui/recursion/issue-83150.rs:10:24: 10:33]>, [closure@/checkout/src/test/ui/recursion/issue-83150.rs:10:24: 10:33]>, [closure@/checkout/src/test/ui/recursion/issue-83150.rs:10:24: 10:33]>, [closure@/checkout/src/test/ui/recursion/issue-83150.rs:10:24: 10:33]>, [closure@/checkout/src/test/ui/recursion/issue-83150.rs:10:24: 10:33]>, [closure@/checkout/src/test/ui/recursion/issue-83150.rs:10:24: 10:33]>, [closure@/checkout/src/test/ui/recursion/issue-83150.rs:10:24: 10:33]>, [closure@/checkout/src/test/ui/recursion/issue-83150.rs:10:24: 10:33]>, [closure@/checkout/src/test/ui/recursion/issue-83150.rs:10:24: 10:33]>, [closure@/checkout/src/test/ui/recursion/issue-83150.rs:10:24: 10:33]>, [closure@/checkout/src/test/ui/recursion/issue-83150.rs:10:24: 10:33]>, [closure@/checkout/src/test/ui/recursion/issue-83150.rs:10:24: 10:33]>, [closure@/checkout/src/test/ui/recursion/issue-83150.rs:10:24: 10:33]>, [closure@/checkout/src/test/ui/recursion/issue-83150.rs:10:24: 10:33]>, [closure@/checkout/src/test/ui/recursion/issue-83150.rs:10:24: 10:33]>, [closure@/checkout/src/test/ui/recursion/issue-83150.rs:10:24: 10:33]>, [closure@/checkout/src/test/ui/recursion/issue-83150.rs:10:24: 10:33]>, [closure@/checkout/src/test/ui/recursion/issue-83150.rs:10:24: 10:33]>, [closure@/checkout/src/test/ui/recursion/issue-83150.rs:10:24: 10:33]>, [closure@/checkout/src/test/ui/recursion/issue-83150.rs:10:24: 10:33]>, [closure@/checkout/src/test/ui/recursion/issue-83150.rs:10:24: 10:33]>, [closure@/checkout/src/test/ui/recursion/issue-83150.rs:10:24: 10:33]>, [closure@/checkout/src/test/ui/recursion/issue-83150.rs:10:24: 10:33]>, [closure@/checkout/src/test/ui/recursion/issue-83150.rs:10:24: 10:33]>, [closure@/checkout/src/test/ui/recursion/issue-83150.rs:10:24: 10:33]>, [closure@/checkout/src/test/ui/recursion/issue-83150.rs:10:24: 10:33]>, [closure@/checkout/src/test/ui/recursion/issue-83150.rs:10:24: 10:33]>, [closure@/checkout/src/test/ui/recursion/issue-83150.rs:10:24: 10:33]>, [closure@/checkout/src/test/ui/recursion/issue-83150.rs:10:24: 10:33]>, [closure@/checkout/src/test/ui/recursion/issue-83150.rs:10:24: 10:33]>, [closure@/checkout/src/test/ui/recursion/issue-83150.rs:10:24: 10:33]>, [closure@/checkout/src/test/ui/recursion/issue-83150.rs:10:24: 10:33]>, [closure@/checkout/src/test/ui/recursion/issue-83150.rs:10:24: 10:33]>, [closure@/checkout/src/test/ui/recursion/issue-83150.rs:10:24: 10:33]>, [closure@/checkout/src/test/ui/recursion/issue-83150.rs:10:24: 10:33]>, [closure@/checkout/src/test/ui/recursion/issue-83150.rs:10:24: 10:33]>, [closure@/checkout/src/test/ui/recursion/issue-83150.rs:10:24: 10:33]>, [closure@/checkout/src/test/ui/recursion/issue-83150.rs:10:24: 10:33]>, [closure@/checkout/src/test/ui/recursion/issue-83150.rs:10:24: 10:33]>, [closure@/checkout/src/test/ui/recursion/issue-83150.rs:10:24: 10:33]>, [closure@/checkout/src/test/ui/recursion/issue-83150.rs:10:24: 10:33]>, [closure@/checkout/src/test/ui/recursion/issue-83150.rs:10:24: 10:33]>, [closure@/checkout/src/test/ui/recursion/issue-83150.rs:10:24: 10:33]>, [closure@/checkout/src/test/ui/recursion/issue-83150.rs:10:24: 10:33]>, [closure@/checkout/src/test/ui/recursion/issue-83150.rs:10:24: 10:33]>, [closure@/checkout/src/test/ui/recursion/issue-83150.rs:10:24: 10:33]>, [closure@/checkout/src/test/ui/recursion/issue-83150.rs:10:24: 10:33]>, [closure@/checkout/src/test/ui/recursion/issue-83150.rs:10:24: 10:33]>, [closure@/checkout/src/test/ui/recursion/issue-83150.rs:10:24: 10:33]>, [closure@/checkout/src/test/ui/recursion/issue-83150.rs:10:24: 10:33]>, [closure@/checkout/src/test/ui/recursion/issue-83150.rs:10:24: 10:33]>, [closure@/checkout/src/test/ui/recursion/issue-83150.rs:10:24: 10:33]>`
error: aborting due to previous error; 1 warning emitted

For more information about this error, try `rustc --explain E0275`.
------------------------------------------
