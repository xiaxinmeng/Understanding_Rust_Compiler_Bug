plain
running 15044 tests
..........................................ii............................................    88/15044
...................................................................................iiiii   176/15044
iiiiiiiiii.....................i..................i.....................................   264/15044
..........................................................................F.F...........   352/15044
........................................................................................   528/15044
........................................................................................   616/15044
........................................................................................   704/15044
........................................................................................   792/15044
---
1 error[E0580]: `main` function has wrong type
-   --> $DIR/issue-111879-1.rs:10:1
+   --> $DIR/issue-111879-1.rs:12:1
3    |
4 LL | fn main(_: for<'a> fn(Foo<fn(&'a ())>::Assoc)) {}


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/associated-inherent-types/issue-111879-1/issue-111879-1.stderr
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/associated-inherent-types/issue-111879-1/issue-111879-1.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args associated-inherent-types/issue-111879-1.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/tests/ui/associated-inherent-types/issue-111879-1.rs" "-Zthreads=1" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--remap-path-prefix=/checkout/tests/ui=fake-test-src-base" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/associated-inherent-types/issue-111879-1" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/associated-inherent-types/issue-111879-1/auxiliary"
stdout: none
error[E0580]: `main` function has wrong type
  --> fake-test-src-base/associated-inherent-types/issue-111879-1.rs:12:1
   |
   |
LL | fn main(_: for<'a> fn(Foo<fn(&'a ())>::Assoc)) {} //~ ERROR `main` function has wrong type
   |
   = note: expected fn pointer `fn()`
   = note: expected fn pointer `fn()`
              found fn pointer `fn(for<'a> fn(Foo<fn(&'a ())>::Assoc))`
error: aborting due to previous error

For more information about this error, try `rustc --explain E0580`.
------------------------------------------
------------------------------------------


---- [ui] tests/ui/associated-inherent-types/issue-111879-0.rs stdout ----
diff of stderr:

1 error: overflow evaluating associated type `Carrier<'b>::Focus<i32>`
-   --> $DIR/issue-111879-0.rs:9:25
3    |
3    |
4 LL |     pub type Focus<T> = &'a mut User;


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/associated-inherent-types/issue-111879-0/issue-111879-0.stderr
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/associated-inherent-types/issue-111879-0/issue-111879-0.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args associated-inherent-types/issue-111879-0.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/tests/ui/associated-inherent-types/issue-111879-0.rs" "-Zthreads=1" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--remap-path-prefix=/checkout/tests/ui=fake-test-src-base" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/associated-inherent-types/issue-111879-0" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/associated-inherent-types/issue-111879-0/auxiliary"
stdout: none
--- stderr -------------------------------
error: overflow evaluating associated type `Carrier<'b>::Focus<i32>`
  --> fake-test-src-base/associated-inherent-types/issue-111879-0.rs:11:25
   |
LL |     pub type Focus<T> = &'a mut User; //~ ERROR overflow evaluating associated type

error: aborting due to previous error
------------------------------------------

