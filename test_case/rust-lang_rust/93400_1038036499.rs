plain
.................................................................................................... 12200/12632
.................................................................................................... 12300/12632
.................................................................................................... 12400/12632
.................................................................................................... 12500/12632
..................F..F...F.iiiF..................................................................... 12600/12632
failures:

---- [ui] ui/issues/issue-36299.rs stdout ----
diff of stderr:
diff of stderr:

5    |            ^^ unused parameter
6    |
7    = help: consider removing `'a`, referring to it in a field, or using a marker such as `PhantomData`
+    = help: if you intended `'a` to be a const parameter, use `const 'a: usize` instead
8 
9 error[E0392]: parameter `A` is never used


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-36299/issue-36299.stderr
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-36299/issue-36299.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args issues/issue-36299.rs`

error: 1 errors occurred comparing output.
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues/issue-36299.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-36299" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-36299/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0392]: parameter `'a` is never used
   |
   |
LL | struct Foo<'a, A> {}
   |            ^^ unused parameter
   |
   = help: consider removing `'a`, referring to it in a field, or using a marker such as `PhantomData`
   = help: if you intended `'a` to be a const parameter, use `const 'a: usize` instead

error[E0392]: parameter `A` is never used
   |
   |
LL | struct Foo<'a, A> {}
   |                ^ unused parameter
   |
   = help: consider removing `A`, referring to it in a field, or using a marker such as `PhantomData`
   = help: if you intended `A` to be a const parameter, use `const A: usize` instead
error: aborting due to 2 previous errors

For more information about this error, try `rustc --explain E0392`.

---
diff of stderr:

28    |                  ^^ unused parameter
29    |
30    = help: consider removing `'c`, referring to it in a field, or using a marker such as `PhantomData`
+    = help: if you intended `'c` to be a const parameter, use `const 'c: usize` instead
32 error: aborting due to 3 previous errors
33 



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/regions/region-bounds-on-objects-and-type-parameters/region-bounds-on-objects-and-type-parameters.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args regions/region-bounds-on-objects-and-type-parameters.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/regions/region-bounds-on-objects-and-type-parameters.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/regions/region-bounds-on-objects-and-type-parameters" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/regions/region-bounds-on-objects-and-type-parameters/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0226]: only a single explicit lifetime bound is permitted
   |
   |
LL |     z: Box<dyn Is<'a>+'b+'c>,


error[E0478]: lifetime bound not satisfied
   |
   |
LL |     z: Box<dyn Is<'a>+'b+'c>,
   |
   |
note: lifetime parameter instantiated with the lifetime `'b` as defined here
   |
   |
LL | struct Foo<'a,'b,'c> { //~ ERROR parameter `'c` is never used
   |               ^^
note: but lifetime parameter must outlive the lifetime `'a` as defined here
   |
   |
LL | struct Foo<'a,'b,'c> { //~ ERROR parameter `'c` is never used


error[E0392]: parameter `'c` is never used
   |
   |
LL | struct Foo<'a,'b,'c> { //~ ERROR parameter `'c` is never used
   |                  ^^ unused parameter
   |
   = help: consider removing `'c`, referring to it in a field, or using a marker such as `PhantomData`
   = help: if you intended `'c` to be a const parameter, use `const 'c: usize` instead
error: aborting due to 3 previous errors

Some errors have detailed explanations: E0226, E0392, E0478.
For more information about an error, try `rustc --explain E0226`.
---
diff of stderr:

79    |            ^^^^^ unused parameter
80    |
81    = help: consider removing `'Self`, referring to it in a field, or using a marker such as `PhantomData`
+    = help: if you intended `'Self` to be a const parameter, use `const 'Self: usize` instead
83 error: aborting due to 12 previous errors
84 



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/self/self_type_keyword/self_type_keyword.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args self/self_type_keyword.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/self/self_type_keyword.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/self/self_type_keyword" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/self/self_type_keyword/auxiliary"
------------------------------------------

------------------------------------------
stderr:
---

error: expected identifier, found keyword `Self`
  --> /checkout/src/test/ui/self/self_type_keyword.rs:14:13
   |
LL |         ref Self => (),


error: `mut` must be followed by a named binding
   |
   |
LL |         mut Self => (),
   |         ^^^^^^^^ help: remove the `mut` prefix: `Self`
   |
   = note: `mut` may be followed by `variable` and `variable @ pattern`
error: expected identifier, found keyword `Self`
  --> /checkout/src/test/ui/self/self_type_keyword.rs:19:17
   |
   |
LL |         ref mut Self => (),

error: expected identifier, found keyword `Self`
  --> /checkout/src/test/ui/self/self_type_keyword.rs:23:15
   |
   |
LL |         Foo { Self } => (),

error: expected identifier, found keyword `Self`
  --> /checkout/src/test/ui/self/self_type_keyword.rs:29:26
   |
---

error: lifetimes cannot use keyword names
  --> /checkout/src/test/ui/self/self_type_keyword.rs:6:12
   |
LL | struct Bar<'Self>;

error: cannot find macro `Self` in this scope
  --> /checkout/src/test/ui/self/self_type_keyword.rs:21:9
   |
   |
LL |         Self!() => (),

error[E0531]: cannot find unit struct, unit variant or constant `Self` in this scope
  --> /checkout/src/test/ui/self/self_type_keyword.rs:16:13
   |
   |
LL |         mut Self => (),
   |
note: unit struct `foo::Self` exists but is inaccessible
  --> /checkout/src/test/ui/self/self_type_keyword.rs:2:3
   |
   |
LL |   struct Self;
   |   ^^^^^^^^^^^^ not accessible

error[E0392]: parameter `'Self` is never used
   |
   |
LL | struct Bar<'Self>;
   |            ^^^^^ unused parameter
   |
   = help: consider removing `'Self`, referring to it in a field, or using a marker such as `PhantomData`
   = help: if you intended `'Self` to be a const parameter, use `const 'Self: usize` instead
error: aborting due to 12 previous errors

Some errors have detailed explanations: E0392, E0531.
For more information about an error, try `rustc --explain E0392`.
---
diff of stderr:

5    |          ^^ unused parameter
6    |
7    = help: consider removing `'a`, referring to it in a field, or using a marker such as `PhantomData`
+    = help: if you intended `'a` to be a const parameter, use `const 'a: usize` instead
8 
9 error[E0392]: parameter `'a` is never used

13    |          ^^ unused parameter
14    |
14    |
15    = help: consider removing `'a`, referring to it in a field, or using a marker such as `PhantomData`
+    = help: if you intended `'a` to be a const parameter, use `const 'a: usize` instead
17 error: aborting due to 2 previous errors
18 



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/variance/variance-regions-unused-indirect/variance-regions-unused-indirect.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args variance/variance-regions-unused-indirect.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/variance/variance-regions-unused-indirect.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/variance/variance-regions-unused-indirect" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/variance/variance-regions-unused-indirect/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0392]: parameter `'a` is never used
  --> /checkout/src/test/ui/variance/variance-regions-unused-indirect.rs:3:10
   |
LL | enum Foo<'a> { //~ ERROR parameter `'a` is never used
   |          ^^ unused parameter
   |
   = help: consider removing `'a`, referring to it in a field, or using a marker such as `PhantomData`
   = help: if you intended `'a` to be a const parameter, use `const 'a: usize` instead

error[E0392]: parameter `'a` is never used
  --> /checkout/src/test/ui/variance/variance-regions-unused-indirect.rs:7:10
   |
LL | enum Bar<'a> { //~ ERROR parameter `'a` is never used
   |          ^^ unused parameter
   |
   = help: consider removing `'a`, referring to it in a field, or using a marker such as `PhantomData`
   = help: if you intended `'a` to be a const parameter, use `const 'a: usize` instead
error: aborting due to 2 previous errors

For more information about this error, try `rustc --explain E0392`.

---
diff of stderr:

5    |                  ^^ unused parameter
6    |
7    = help: consider removing `'a`, referring to it in a field, or using a marker such as `PhantomData`
+    = help: if you intended `'a` to be a const parameter, use `const 'a: usize` instead
8 
9 error[E0392]: parameter `'d` is never used

13    |                   ^^ unused parameter
14    |
14    |
15    = help: consider removing `'d`, referring to it in a field, or using a marker such as `PhantomData`
+    = help: if you intended `'d` to be a const parameter, use `const 'd: usize` instead
17 error: aborting due to 2 previous errors
18 



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/variance/variance-regions-unused-direct/variance-regions-unused-direct.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args variance/variance-regions-unused-direct.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/variance/variance-regions-unused-direct.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/variance/variance-regions-unused-direct" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/variance/variance-regions-unused-direct/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0392]: parameter `'a` is never used
  --> /checkout/src/test/ui/variance/variance-regions-unused-direct.rs:5:18
   |
LL | struct Bivariant<'a>; //~ ERROR parameter `'a` is never used
   |                  ^^ unused parameter
   |
   = help: consider removing `'a`, referring to it in a field, or using a marker such as `PhantomData`
   = help: if you intended `'a` to be a const parameter, use `const 'a: usize` instead

error[E0392]: parameter `'d` is never used
  --> /checkout/src/test/ui/variance/variance-regions-unused-direct.rs:7:19
   |
LL | struct Struct<'a, 'd> { //~ ERROR parameter `'d` is never used
   |                   ^^ unused parameter
   |
   = help: consider removing `'d`, referring to it in a field, or using a marker such as `PhantomData`
   = help: if you intended `'d` to be a const parameter, use `const 'd: usize` instead
error: aborting due to 2 previous errors

For more information about this error, try `rustc --explain E0392`.

---
diff of stderr:

5    |                   ^^ unused parameter
6    |
7    = help: consider removing `'a`, referring to it in a field, or using a marker such as `PhantomData`
+    = help: if you intended `'a` to be a const parameter, use `const 'a: usize` instead
8 
9 error[E0392]: parameter `'a` is never used

13    |               ^^ unused parameter
14    |
14    |
15    = help: consider removing `'a`, referring to it in a field, or using a marker such as `PhantomData`
+    = help: if you intended `'a` to be a const parameter, use `const 'a: usize` instead
17 error: aborting due to 2 previous errors
18 



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/variance/variance-unused-region-param/variance-unused-region-param.stderr
To only update this specific test, also pass `--test-args variance/variance-unused-region-param.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/variance/variance-unused-region-param.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/variance/variance-unused-region-param" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/variance/variance-unused-region-param/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0392]: parameter `'a` is never used
  --> /checkout/src/test/ui/variance/variance-unused-region-param.rs:3:19
   |
LL | struct SomeStruct<'a> { x: u32 } //~ ERROR parameter `'a` is never used
   |                   ^^ unused parameter
   |
   = help: consider removing `'a`, referring to it in a field, or using a marker such as `PhantomData`
   = help: if you intended `'a` to be a const parameter, use `const 'a: usize` instead

error[E0392]: parameter `'a` is never used
  --> /checkout/src/test/ui/variance/variance-unused-region-param.rs:4:15
   |
LL | enum SomeEnum<'a> { Nothing } //~ ERROR parameter `'a` is never used
   |               ^^ unused parameter
   |
   = help: consider removing `'a`, referring to it in a field, or using a marker such as `PhantomData`
   = help: if you intended `'a` to be a const parameter, use `const 'a: usize` instead
error: aborting due to 2 previous errors

For more information about this error, try `rustc --explain E0392`.


------------------------------------------


---- [ui] ui/variance/variance-unused-type-param.rs stdout ----
diff of stderr:

34    = help: consider removing `T`, referring to it in a field, or using a marker such as `PhantomData`
36 error[E0392]: parameter `T` is never used
-   --> $DIR/variance-unused-type-param.rs:19:24
+   --> $DIR/variance-unused-type-param.rs:22:24
38    |
38    |
39 LL | struct WithWhereBounds<T> where T: Sized {}
40    |                        ^ unused parameter

42    = help: consider removing `T`, referring to it in a field, or using a marker such as `PhantomData`
44 error[E0392]: parameter `T` is never used
-   --> $DIR/variance-unused-type-param.rs:19:27
+   --> $DIR/variance-unused-type-param.rs:25:27
46    |
46    |
47 LL | struct WithOutlivesBounds<T: 'static> {}
48    |                           ^ unused parameter

The actual stderr differed from the expected stderr.
The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/variance/variance-unused-type-param/variance-unused-type-param.stderr
To only update this specific test, also pass `--test-args variance/variance-unused-type-param.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/variance/variance-unused-type-param.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/variance/variance-unused-type-param" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/variance/variance-unused-type-param/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0392]: parameter `A` is never used
  --> /checkout/src/test/ui/variance/variance-unused-type-param.rs:6:19
   |
LL | struct SomeStruct<A> { x: u32 }
   |                   ^ unused parameter
   |
   = help: consider removing `A`, referring to it in a field, or using a marker such as `PhantomData`
   = help: if you intended `A` to be a const parameter, use `const A: usize` instead

error[E0392]: parameter `A` is never used
  --> /checkout/src/test/ui/variance/variance-unused-type-param.rs:9:15
   |
LL | enum SomeEnum<A> { Nothing }
   |               ^ unused parameter
   |
   = help: consider removing `A`, referring to it in a field, or using a marker such as `PhantomData`
   = help: if you intended `A` to be a const parameter, use `const A: usize` instead
error[E0392]: parameter `T` is never used
  --> /checkout/src/test/ui/variance/variance-unused-type-param.rs:13:15
   |
   |
LL | enum ListCell<T> {
   |               ^ unused parameter
   |
   = help: consider removing `T`, referring to it in a field, or using a marker such as `PhantomData`
   = help: if you intended `T` to be a const parameter, use `const T: usize` instead
error[E0392]: parameter `T` is never used
  --> /checkout/src/test/ui/variance/variance-unused-type-param.rs:19:19
   |
   |
LL | struct WithBounds<T: Sized> {}
   |                   ^ unused parameter
   |
   = help: consider removing `T`, referring to it in a field, or using a marker such as `PhantomData`
error[E0392]: parameter `T` is never used
  --> /checkout/src/test/ui/variance/variance-unused-type-param.rs:22:24
   |
   |
LL | struct WithWhereBounds<T> where T: Sized {}
   |                        ^ unused parameter
   |
   = help: consider removing `T`, referring to it in a field, or using a marker such as `PhantomData`
error[E0392]: parameter `T` is never used
  --> /checkout/src/test/ui/variance/variance-unused-type-param.rs:25:27
   |
   |
LL | struct WithOutlivesBounds<T: 'static> {}
   |                           ^ unused parameter
   |
   = help: consider removing `T`, referring to it in a field, or using a marker such as `PhantomData`
error: aborting due to 6 previous errors

For more information about this error, try `rustc --explain E0392`.

