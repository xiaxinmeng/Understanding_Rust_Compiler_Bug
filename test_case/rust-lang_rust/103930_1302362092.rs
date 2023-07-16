plain
Successfully built 8ae63bd84783
Successfully tagged rust-ci:latest
Built container sha256:8ae63bd84783a2f7577021efd8513a12e4b6c9cb874276127f38c9da56f412c9
Uploading finished image to https://ci-caches.rust-lang.org/docker/a27a2e8501e6bda8d9ec9572725b52c65accf3f919588efe2ef5cb584fdeae418747b3be4fa090dfcc3ff7ae8714c60234c3f32ed53a403a0831a7e22eb564d1
upload failed: - to s3://rust-lang-ci-sccache2/docker/a27a2e8501e6bda8d9ec9572725b52c65accf3f919588efe2ef5cb584fdeae418747b3be4fa090dfcc3ff7ae8714c60234c3f32ed53a403a0831a7e22eb564d1 Unable to locate credentials
[CI_JOB_NAME=x86_64-gnu-llvm-13]
---
........................................................................................ 10736/13769
........................................................................................ 10824/13769
.................................................................iiiii...i....i.i....... 10912/13769
........................................................................................ 11000/13769
.................................................F....i..F.............................. 11088/13769
........................................................................................ 11264/13769
........................................................................................ 11352/13769
........................................................................................ 11440/13769
........................................................................................ 11528/13769
---
1 error[E0106]: missing lifetime specifier
-   --> $DIR/rfc1623-2.rs:8:42
+   --> $DIR/rfc1623-3.rs:8:42
3    |
4 LL | static NON_ELIDABLE_FN: &fn(&u8, &u8) -> &u8 =
5    |                             ---  ---     ^ expected named lifetime parameter
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
12    |                          +++++++     ++      ++         ++
13 
14 error[E0106]: missing lifetime specifier
-   --> $DIR/rfc1623-2.rs:10:39
-   --> $DIR/rfc1623-2.rs:10:39
+   --> $DIR/rfc1623-3.rs:10:39
16    |
17 LL |     &(non_elidable as fn(&u8, &u8) -> &u8);
18    |                          ---  ---     ^ expected named lifetime parameter
24    |                       +++++++     ++      ++         ++
25 
25 
26 error[E0605]: non-primitive cast: `for<'a, 'b> fn(&'a u8, &'b u8) -> &'a u8 {non_elidable}` as `for<'a, 'b> fn(&'a u8, &'b u8) -> &u8`
-   --> $DIR/rfc1623-2.rs:10:6
+   --> $DIR/rfc1623-3.rs:10:6
28    |
29 LL |     &(non_elidable as fn(&u8, &u8) -> &u8);
30    |      ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ invalid cast

The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfcs/rfc1623-3/rfc1623-3.stderr
To update references, rerun the tests and pass the `--bless` flag
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args rfcs/rfc1623-3.rs`
error: 1 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/rfcs/rfc1623-3.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfcs/rfc1623-3" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfcs/rfc1623-3/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0106]: missing lifetime specifier
   |
   |
LL | static NON_ELIDABLE_FN: &fn(&u8, &u8) -> &u8 =
   |                             ---  ---     ^ expected named lifetime parameter
   |
   = help: this function's return type contains a borrowed value, but the signature does not say whether it is borrowed from argument 1 or argument 2
   = note: for more information on higher-ranked polymorphism, visit https://doc.rust-lang.org/nomicon/hrtb.html
help: consider making the type lifetime-generic with a new `'a` lifetime
   |
LL | static NON_ELIDABLE_FN: &for<'a> fn(&'a u8, &'a u8) -> &'a u8 =
   |                          +++++++     ++      ++         ++
error[E0106]: missing lifetime specifier
  --> /checkout/src/test/ui/rfcs/rfc1623-3.rs:10:39
   |
   |
LL |     &(non_elidable as fn(&u8, &u8) -> &u8);
   |                          ---  ---     ^ expected named lifetime parameter
   |
   = help: this function's return type contains a borrowed value, but the signature does not say whether it is borrowed from argument 1 or argument 2
help: consider making the type lifetime-generic with a new `'a` lifetime
   |
LL |     &(non_elidable as for<'a> fn(&'a u8, &'a u8) -> &'a u8);
   |                       +++++++     ++      ++         ++

error[E0605]: non-primitive cast: `for<'a, 'b> fn(&'a u8, &'b u8) -> &'a u8 {non_elidable}` as `for<'a, 'b> fn(&'a u8, &'b u8) -> &u8`
   |
   |
LL |     &(non_elidable as fn(&u8, &u8) -> &u8);
   |      ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ invalid cast
error: aborting due to 3 previous errors

Some errors have detailed explanations: E0106, E0605.
For more information about an error, try `rustc --explain E0106`.
---
3    |
4 LL |     f: &id,
5    |        ^^^ one type is more general than the other

8               found trait `Fn<(&Foo<'_>,)>`
10 error[E0308]: mismatched types
-   --> $DIR/rfc1623.rs:28:8
+   --> $DIR/rfc1623-2.rs:28:8
12    |
12    |
13 LL |     f: &id,
14    |        ^^^ one type is more general than the other

17               found trait `Fn<(&Foo<'_>,)>`
18 
19 error: implementation of `FnOnce` is not general enough
-   --> $DIR/rfc1623.rs:28:8
+   --> $DIR/rfc1623-2.rs:28:8
22 LL |     f: &id,
22 LL |     f: &id,
23    |        ^^^ implementation of `FnOnce` is not general enough

26    = note: ...but it actually implements `FnOnce<(&'2 Foo<'_>,)>`, for some specific lifetime `'2`
27 
28 error: implementation of `FnOnce` is not general enough
-   --> $DIR/rfc1623.rs:28:8
+   --> $DIR/rfc1623-2.rs:28:8
31 LL |     f: &id,
31 LL |     f: &id,
32    |        ^^^ implementation of `FnOnce` is not general enough

The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfcs/rfc1623-2/rfc1623-2.stderr
To update references, rerun the tests and pass the `--bless` flag
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args rfcs/rfc1623-2.rs`
error: 1 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/rfcs/rfc1623-2.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfcs/rfc1623-2" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfcs/rfc1623-2/auxiliary"
stdout: none
--- stderr -------------------------------
  --> /checkout/src/test/ui/rfcs/rfc1623-2.rs:28:8
   |
LL |     f: &id,
   |        ^^^ one type is more general than the other
   |        ^^^ one type is more general than the other
   |
   = note: expected trait `for<'a, 'b> Fn<(&'a Foo<'b>,)>`
              found trait `Fn<(&Foo<'_>,)>`
error[E0308]: mismatched types
  --> /checkout/src/test/ui/rfcs/rfc1623-2.rs:28:8
   |
LL |     f: &id,
LL |     f: &id,
   |        ^^^ one type is more general than the other
   |
   = note: expected trait `for<'a, 'b> Fn<(&'a Foo<'b>,)>`
              found trait `Fn<(&Foo<'_>,)>`

error: implementation of `FnOnce` is not general enough
   |
LL |     f: &id,
LL |     f: &id,
   |        ^^^ implementation of `FnOnce` is not general enough
   |
   = note: `fn(&'2 Foo<'_>) -> &'2 Foo<'_> {id::<&'2 Foo<'_>>}` must implement `FnOnce<(&'1 Foo<'b>,)>`, for any lifetime `'1`...
   = note: ...but it actually implements `FnOnce<(&'2 Foo<'_>,)>`, for some specific lifetime `'2`

error: implementation of `FnOnce` is not general enough
   |
LL |     f: &id,
LL |     f: &id,
   |        ^^^ implementation of `FnOnce` is not general enough
   |
   = note: `fn(&Foo<'2>) -> &Foo<'2> {id::<&Foo<'2>>}` must implement `FnOnce<(&'a Foo<'1>,)>`, for any lifetime `'1`...
   = note: ...but it actually implements `FnOnce<(&Foo<'2>,)>`, for some specific lifetime `'2`
error: aborting due to 4 previous errors

For more information about this error, try `rustc --explain E0308`.
------------------------------------------
