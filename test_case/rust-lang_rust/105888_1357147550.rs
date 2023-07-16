plain
---- [ui] src/test/ui/higher-rank-trait-bounds/normalize-under-binder/issue-71955.rs stdout ----
diff of stderr:

16    |
17 LL |     F2: FnOnce(&<F1 as Parser>::Output) -> bool
+ help: consider changing the type of the closure parameters
+    |
+    |
+ LL |     foo(bar, "string", |_: &_| s.len() == 5);
19 
20 error[E0308]: mismatched types
21   --> $DIR/issue-71955.rs:45:5


35    |
36 LL |     F2: FnOnce(&<F1 as Parser>::Output) -> bool
+ help: consider changing the type of the closure parameters
+    |
+    |
+ LL |     foo(bar, "string", |_| s.len() == 5);
38 
39 error[E0308]: mismatched types
40   --> $DIR/issue-71955.rs:48:5


54    |
55 LL |     F2: FnOnce(&<F1 as Parser>::Output) -> bool
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
+ help: consider changing the type of the closure parameters
+    |
+    |
+ LL |     foo(baz, "string", |_: &_| s.0.len() == 5);
57 
58 error[E0308]: mismatched types
59   --> $DIR/issue-71955.rs:48:5


73    |
74 LL |     F2: FnOnce(&<F1 as Parser>::Output) -> bool
+ help: consider changing the type of the closure parameters
+    |
+    |
+ LL |     foo(baz, "string", |_| s.0.len() == 5);
76 
77 error: aborting due to 4 previous errors
78 

---
To only update this specific test, also pass `--test-args higher-rank-trait-bounds/normalize-under-binder/issue-71955.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/higher-rank-trait-bounds/normalize-under-binder/issue-71955.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/higher-rank-trait-bounds/normalize-under-binder/issue-71955" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/higher-rank-trait-bounds/normalize-under-binder/issue-71955/auxiliary"
stdout: none
--- stderr -------------------------------
  --> /checkout/src/test/ui/higher-rank-trait-bounds/normalize-under-binder/issue-71955.rs:45:5
   |
   |
LL |     foo(bar, "string", |s| s.len() == 5);
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ one type is more general than the other
   |
   = note: expected trait `for<'a, 'b> FnOnce<(&'a &'b str,)>`
              found trait `for<'a> FnOnce<(&'a &str,)>`
note: this closure does not fulfill the lifetime requirements
   |
   |
LL |     foo(bar, "string", |s| s.len() == 5);
note: the lifetime requirement is introduced here
  --> /checkout/src/test/ui/higher-rank-trait-bounds/normalize-under-binder/issue-71955.rs:25:9
   |
   |
LL |     F2: FnOnce(&<F1 as Parser>::Output) -> bool
help: consider changing the type of the closure parameters
   |
   |
LL |     foo(bar, "string", |_: &_| s.len() == 5);

error[E0308]: mismatched types
  --> /checkout/src/test/ui/higher-rank-trait-bounds/normalize-under-binder/issue-71955.rs:45:5
   |
   |
LL |     foo(bar, "string", |s| s.len() == 5);
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ one type is more general than the other
   |
   = note: expected trait `for<'a, 'b> FnOnce<(&'a &'b str,)>`
              found trait `for<'a> FnOnce<(&'a &str,)>`
note: this closure does not fulfill the lifetime requirements
   |
   |
LL |     foo(bar, "string", |s| s.len() == 5);
note: the lifetime requirement is introduced here
  --> /checkout/src/test/ui/higher-rank-trait-bounds/normalize-under-binder/issue-71955.rs:25:44
   |
   |
LL |     F2: FnOnce(&<F1 as Parser>::Output) -> bool
help: consider changing the type of the closure parameters
   |
   |
LL |     foo(bar, "string", |_| s.len() == 5);

error[E0308]: mismatched types
  --> /checkout/src/test/ui/higher-rank-trait-bounds/normalize-under-binder/issue-71955.rs:48:5
   |
   |
LL |     foo(baz, "string", |s| s.0.len() == 5);
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ one type is more general than the other
   |
   = note: expected trait `for<'a, 'b> FnOnce<(&'a Wrapper<'b>,)>`
              found trait `for<'a> FnOnce<(&'a Wrapper<'_>,)>`
note: this closure does not fulfill the lifetime requirements
   |
   |
LL |     foo(baz, "string", |s| s.0.len() == 5);
note: the lifetime requirement is introduced here
  --> /checkout/src/test/ui/higher-rank-trait-bounds/normalize-under-binder/issue-71955.rs:25:9
   |
   |
LL |     F2: FnOnce(&<F1 as Parser>::Output) -> bool
help: consider changing the type of the closure parameters
   |
   |
LL |     foo(baz, "string", |_: &_| s.0.len() == 5);

error[E0308]: mismatched types
  --> /checkout/src/test/ui/higher-rank-trait-bounds/normalize-under-binder/issue-71955.rs:48:5
   |
   |
LL |     foo(baz, "string", |s| s.0.len() == 5);
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ one type is more general than the other
   |
   = note: expected trait `for<'a, 'b> FnOnce<(&'a Wrapper<'b>,)>`
              found trait `for<'a> FnOnce<(&'a Wrapper<'_>,)>`
note: this closure does not fulfill the lifetime requirements
   |
   |
LL |     foo(baz, "string", |s| s.0.len() == 5);
note: the lifetime requirement is introduced here
  --> /checkout/src/test/ui/higher-rank-trait-bounds/normalize-under-binder/issue-71955.rs:25:44
   |
   |
LL |     F2: FnOnce(&<F1 as Parser>::Output) -> bool
help: consider changing the type of the closure parameters
   |
   |
LL |     foo(baz, "string", |_| s.0.len() == 5);

error: aborting due to 4 previous errors

For more information about this error, try `rustc --explain E0308`.
For more information about this error, try `rustc --explain E0308`.
------------------------------------------


---- [ui] src/test/ui/lifetimes/issue-79187-2.rs stdout ----
diff of stderr:

43    |
44 LL | fn take_foo(_: impl Foo) {}
+ help: consider changing the type of the closure parameters
+    |
+    |
+ LL |     take_foo(|_: &_| a);
46 
47 error[E0308]: mismatched types
48   --> $DIR/issue-79187-2.rs:11:5

---
To only update this specific test, also pass `--test-args lifetimes/issue-79187-2.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/lifetimes/issue-79187-2.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lifetimes/issue-79187-2" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lifetimes/issue-79187-2/auxiliary"
stdout: none
--- stderr -------------------------------
error: lifetime may not live long enough
   |
   |
LL |     take_foo(|a: &i32| a);
   |                  -   - ^ returning this value requires that `'1` must outlive `'2`
   |                  |   |
   |                  |   return type of closure is &'2 i32
   |                  let's call the lifetime of this reference `'1`
error: lifetime may not live long enough
  --> /checkout/src/test/ui/lifetimes/issue-79187-2.rs:14:34
   |
   |
LL |     take_foo(|a: &i32| -> &i32 { a });
   |                  -        -      ^ returning this value requires that `'1` must outlive `'2`
   |                  |        |
   |                  |        let's call the lifetime of this reference `'2`
   |                  let's call the lifetime of this reference `'1`

error: implementation of `FnOnce` is not general enough
   |
   |
LL |     take_foo(|a| a);
   |     ^^^^^^^^^^^^^^^ implementation of `FnOnce` is not general enough
   |
   = note: closure with signature `fn(&'2 i32) -> &i32` must implement `FnOnce<(&'1 i32,)>`, for any lifetime `'1`...
   = note: ...but it actually implements `FnOnce<(&'2 i32,)>`, for some specific lifetime `'2`
error[E0308]: mismatched types
  --> /checkout/src/test/ui/lifetimes/issue-79187-2.rs:8:5
   |
   |
LL |     take_foo(|a| a);
   |     ^^^^^^^^^^^^^^^ one type is more general than the other
   |
   = note: expected trait `for<'a> Fn<(&'a i32,)>`
              found trait `Fn<(&i32,)>`
note: this closure does not fulfill the lifetime requirements
   |
   |
LL |     take_foo(|a| a);
note: the lifetime requirement is introduced here
  --> /checkout/src/test/ui/lifetimes/issue-79187-2.rs:5:21
   |
   |
LL | fn take_foo(_: impl Foo) {}
help: consider changing the type of the closure parameters
   |
   |
LL |     take_foo(|_: &_| a);

error[E0308]: mismatched types
  --> /checkout/src/test/ui/lifetimes/issue-79187-2.rs:11:5
   |
   |
LL |     take_foo(|a: &i32| a);
   |     ^^^^^^^^^^^^^^^^^^^^^ one type is more general than the other
   = note: expected reference `&i32`
              found reference `&i32`
note: the lifetime requirement is introduced here
  --> /checkout/src/test/ui/lifetimes/issue-79187-2.rs:5:21
  --> /checkout/src/test/ui/lifetimes/issue-79187-2.rs:5:21
   |
LL | fn take_foo(_: impl Foo) {}

error[E0308]: mismatched types
  --> /checkout/src/test/ui/lifetimes/issue-79187-2.rs:14:5
   |
   |
LL |     take_foo(|a: &i32| -> &i32 { a });
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ one type is more general than the other
   = note: expected reference `&i32`
              found reference `&i32`
note: the lifetime requirement is introduced here
  --> /checkout/src/test/ui/lifetimes/issue-79187-2.rs:5:21
  --> /checkout/src/test/ui/lifetimes/issue-79187-2.rs:5:21
   |
LL | fn take_foo(_: impl Foo) {}

error: aborting due to 6 previous errors

For more information about this error, try `rustc --explain E0308`.
For more information about this error, try `rustc --explain E0308`.
------------------------------------------


---- [ui] src/test/ui/lifetimes/issue-79187.rs stdout ----
diff of stderr:

16    |
17 LL | fn thing(x: impl FnOnce(&u32)) {}
+ help: consider changing the type of the closure parameters
+    |
+    |
+ LL |     let f = |_: &_| ();
19 
19 
20 error: implementation of `FnOnce` is not general enough


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lifetimes/issue-79187/issue-79187.stderr
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lifetimes/issue-79187/issue-79187.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args lifetimes/issue-79187.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/lifetimes/issue-79187.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lifetimes/issue-79187" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lifetimes/issue-79187/auxiliary"
stdout: none
--- stderr -------------------------------
  --> /checkout/src/test/ui/lifetimes/issue-79187.rs:5:5
   |
LL |     thing(f);
   |     ^^^^^^^^ one type is more general than the other
   |     ^^^^^^^^ one type is more general than the other
   |
   = note: expected trait `for<'a> FnOnce<(&'a u32,)>`
              found trait `FnOnce<(&u32,)>`
note: this closure does not fulfill the lifetime requirements
   |
   |
LL |     let f = |_| ();
note: the lifetime requirement is introduced here
  --> /checkout/src/test/ui/lifetimes/issue-79187.rs:1:18
   |
   |
LL | fn thing(x: impl FnOnce(&u32)) {}
help: consider changing the type of the closure parameters
   |
   |
LL |     let f = |_: &_| ();


error: implementation of `FnOnce` is not general enough
   |
LL |     thing(f);
LL |     thing(f);
   |     ^^^^^^^^ implementation of `FnOnce` is not general enough
   |
   = note: closure with signature `fn(&'2 u32)` must implement `FnOnce<(&'1 u32,)>`, for any lifetime `'1`...
   = note: ...but it actually implements `FnOnce<(&'2 u32,)>`, for some specific lifetime `'2`
error: aborting due to 2 previous errors

For more information about this error, try `rustc --explain E0308`.
------------------------------------------
------------------------------------------


---- [ui] src/test/ui/mismatched_types/closure-mismatch.rs stdout ----
diff of stderr:

25    |
26 LL | fn baz<T: Foo>(_: T) {}
+ help: consider changing the type of the closure parameters
+    |
+    |
+ LL |     baz(|_: &_| ());
28 
29 error: aborting due to 2 previous errors
30 

---
To only update this specific test, also pass `--test-args mismatched_types/closure-mismatch.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/mismatched_types/closure-mismatch.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/mismatched_types/closure-mismatch" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/mismatched_types/closure-mismatch/auxiliary"
stdout: none
--- stderr -------------------------------
error: implementation of `FnOnce` is not general enough
   |
   |
LL |     baz(|_| ());
   |     ^^^^^^^^^^^ implementation of `FnOnce` is not general enough
   |
   = note: closure with signature `fn(&'2 ())` must implement `FnOnce<(&'1 (),)>`, for any lifetime `'1`...
   = note: ...but it actually implements `FnOnce<(&'2 (),)>`, for some specific lifetime `'2`
error[E0308]: mismatched types
  --> /checkout/src/test/ui/mismatched_types/closure-mismatch.rs:8:5
   |
   |
LL |     baz(|_| ());
   |     ^^^^^^^^^^^ one type is more general than the other
   |
   = note: expected trait `for<'a> Fn<(&'a (),)>`
              found trait `Fn<(&(),)>`
note: this closure does not fulfill the lifetime requirements
   |
   |
LL |     baz(|_| ());
note: the lifetime requirement is introduced here
  --> /checkout/src/test/ui/mismatched_types/closure-mismatch.rs:5:11
   |
   |
LL | fn baz<T: Foo>(_: T) {}
help: consider changing the type of the closure parameters
   |
   |
LL |     baz(|_: &_| ());

error: aborting due to 2 previous errors

For more information about this error, try `rustc --explain E0308`.
