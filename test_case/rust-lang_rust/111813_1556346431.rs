plain

---- [ui] tests/ui/borrowck/borrowck-thread-local-static-borrow-outlives-fn.rs stdout ----
diff of stderr:

4 LL |      assert_static(&FOO);
5    |                    ^^^^ thread-local variables cannot be borrowed beyond the end of the function
6 LL | }
-    | - end of enclosing function is here
+    |  - end of enclosing function is here
9 error: aborting due to previous error
10 



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/borrowck/borrowck-thread-local-static-borrow-outlives-fn/borrowck-thread-local-static-borrow-outlives-fn.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args borrowck/borrowck-thread-local-static-borrow-outlives-fn.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/tests/ui/borrowck/borrowck-thread-local-static-borrow-outlives-fn.rs" "-Zthreads=1" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--remap-path-prefix=/checkout/tests/ui=fake-test-src-base" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/borrowck/borrowck-thread-local-static-borrow-outlives-fn" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/borrowck/borrowck-thread-local-static-borrow-outlives-fn/auxiliary"
stdout: none
error[E0712]: thread-local variable borrowed past end of function
  --> fake-test-src-base/borrowck/borrowck-thread-local-static-borrow-outlives-fn.rs:8:20
   |
   |
LL |      assert_static(&FOO); //~ ERROR [E0712]
   |                    ^^^^ thread-local variables cannot be borrowed beyond the end of the function
LL | }
   |  - end of enclosing function is here
error: aborting due to previous error

For more information about this error, try `rustc --explain E0712`.
------------------------------------------
------------------------------------------


---- [ui] tests/ui/issues/issue-17954.rs stdout ----
diff of stderr:

5    |             ^^^^ thread-local variables cannot be borrowed beyond the end of the function
6 ...
7 LL | }
-    | - end of enclosing function is here
+    |  - end of enclosing function is here
10 error: aborting due to previous error
11 



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-17954/issue-17954.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args issues/issue-17954.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/tests/ui/issues/issue-17954.rs" "-Zthreads=1" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--remap-path-prefix=/checkout/tests/ui=fake-test-src-base" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-17954" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-17954/auxiliary"
stdout: none
error[E0712]: thread-local variable borrowed past end of function
  --> fake-test-src-base/issues/issue-17954.rs:7:13
   |
LL |     let a = &FOO;
LL |     let a = &FOO;
   |             ^^^^ thread-local variables cannot be borrowed beyond the end of the function
...
LL | }
   |  - end of enclosing function is here
error: aborting due to previous error

For more information about this error, try `rustc --explain E0712`.
------------------------------------------
---
diff of stderr:

2   --> $DIR/issue-52049.rs:6:10
3    |
4 LL |     foo(&unpromotable(5u32));
+    |     -----^^^^^^^^^^^^^^^^^^-- temporary value is freed at the end of this statement
6    |     |    |
7    |     |    creates a temporary value which is freed while still in use
7    |     |    creates a temporary value which is freed while still in use
8    |     argument requires that borrow lasts for `'static`
- LL | }
-    | - temporary value is freed at the end of this statement
11 
12 error: aborting due to previous error
---
To only update this specific test, also pass `--test-args issues/issue-52049.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/tests/ui/issues/issue-52049.rs" "-Zthreads=1" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--remap-path-prefix=/checkout/tests/ui=fake-test-src-base" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-52049" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-52049/auxiliary"
stdout: none
error[E0716]: temporary value dropped while borrowed
  --> fake-test-src-base/issues/issue-52049.rs:6:10
   |
   |
LL |     foo(&unpromotable(5u32));
   |     -----^^^^^^^^^^^^^^^^^^-- temporary value is freed at the end of this statement
   |     |    creates a temporary value which is freed while still in use
   |     |    creates a temporary value which is freed while still in use
   |     argument requires that borrow lasts for `'static`
error: aborting due to previous error

For more information about this error, try `rustc --explain E0716`.
------------------------------------------
------------------------------------------


---- [ui] tests/ui/lifetimes/issue-90600-expected-return-static-indirect.rs stdout ----
diff of stderr:

10    |                -------- cast requires that `foo` is borrowed for `'static`
12 LL | }
12 LL | }
-    | - `foo` dropped here while still borrowed
+    |  - `foo` dropped here while still borrowed
15 error: lifetime may not live long enough
16   --> $DIR/issue-90600-expected-return-static-indirect.rs:9:16



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lifetimes/issue-90600-expected-return-static-indirect/issue-90600-expected-return-static-indirect.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args lifetimes/issue-90600-expected-return-static-indirect.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/tests/ui/lifetimes/issue-90600-expected-return-static-indirect.rs" "-Zthreads=1" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--remap-path-prefix=/checkout/tests/ui=fake-test-src-base" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lifetimes/issue-90600-expected-return-static-indirect" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lifetimes/issue-90600-expected-return-static-indirect/auxiliary"
stdout: none
error[E0597]: `foo` does not live long enough
  --> fake-test-src-base/lifetimes/issue-90600-expected-return-static-indirect.rs:7:32
   |
   |
LL | fn inner(mut foo: &[u8]) {
   |          ------- binding `foo` declared here
LL |     let refcell = RefCell::new(&mut foo);
   |                                ^^^^^^^^ borrowed value does not live long enough
LL |     //~^ ERROR `foo` does not live long enough
LL |     let read = &refcell as &RefCell<dyn Read>;
   |                -------- cast requires that `foo` is borrowed for `'static`
LL | }
LL | }
   |  - `foo` dropped here while still borrowed
error: lifetime may not live long enough
  --> fake-test-src-base/lifetimes/issue-90600-expected-return-static-indirect.rs:9:16
   |
   |
LL | fn inner(mut foo: &[u8]) {
   |                   - let's call the lifetime of this reference `'1`
...
LL |     let read = &refcell as &RefCell<dyn Read>;
   |                ^^^^^^^^ cast requires that `'1` must outlive `'static`
error: aborting due to 2 previous errors

For more information about this error, try `rustc --explain E0597`.
------------------------------------------
