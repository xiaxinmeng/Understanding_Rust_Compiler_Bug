plain
..................................................................................................ii 3700/12160
.................................................................................................... 3800/12160
................i............................................i...................................... 3900/12160
.................................................................................................... 4000/12160
......FF......F.................FF.................................................F................ 4100/12160
.................................................................................................... 4300/12160
.................................................................................................... 4400/12160
.................................................................................................... 4500/12160
.......................................................................ii........................... 4600/12160
---
failures:

---- [ui] ui/generic-associated-types/collections.rs stdout ----

error: test compilation failed although it shouldn't!
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/generic-associated-types/collections.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/generic-associated-types/collections/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/generic-associated-types/collections/auxiliary"
------------------------------------------

------------------------------------------
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
---
   |
LL | trait Collection<T> {
   |                  - required by this bound in `Collection`
...
LL |     type Sibling<U>: Collection<U> =
   |                  -   ^^^^^^^^^^^^^ doesn't have a size known at compile-time
   |                  this type parameter needs to be `std::marker::Sized`
   |
   |
help: consider relaxing the implicit `Sized` restriction
   |
LL | trait Collection<T: ?Sized> {

error[E0277]: the size for values of type `T` cannot be known at compilation time
  --> /checkout/src/test/ui/generic-associated-types/collections.rs:25:21
   |
   |
LL | trait Collection<T> {
   |                  - required by this bound in `Collection`
...
LL |     type Member<T>: Collection<T, Family = Self>;
   |                 -   ^^^^^^^^^^^^^^^^^^^^^^^^^^^^ doesn't have a size known at compile-time
   |                 this type parameter needs to be `std::marker::Sized`
   |
   |
help: consider relaxing the implicit `Sized` restriction
   |
LL | trait Collection<T: ?Sized> {

error[E0277]: the size for values of type `T` cannot be known at compilation time
  --> /checkout/src/test/ui/generic-associated-types/collections.rs:31:22
   |
   |
LL |     type Member<T> = Vec<T>;
   |                 -    ^^^^^^ doesn't have a size known at compile-time
   |                 this type parameter needs to be `std::marker::Sized`
   | 
  ::: /checkout/library/alloc/src/vec/mod.rs:396:16
   |
   |
LL | pub struct Vec<T, #[unstable(feature = "allocator_api", issue = "32838")] A: Allocator = Global> {
   |                - required by this bound in `Vec`
error: aborting due to 3 previous errors

For more information about this error, try `rustc --explain E0277`.

---
-   --> $DIR/collections-project-default.rs:59:5
+ error[E0277]: the size for values of type `U` cannot be known at compilation time
+   --> $DIR/collections-project-default.rs:14:22
3    |
- LL | fn floatify_sibling<C>(ints: &C) -> <C as Collection<i32>>::Sibling<f32>
-    |                                     ------------------------------------ expected `<C as Collection<i32>>::Sibling<f32>` because of return type
+ LL | trait Collection<T> {
+    |                  - required by this bound in `Collection`
- LL |     res
- LL |     res
-    |     ^^^ expected Collection::Sibling, found CollectionFamily::Member
+ LL |     type Sibling<U>: Collection<U> =
+    |                  -   ^^^^^^^^^^^^^ doesn't have a size known at compile-time
+    |                  this type parameter needs to be `std::marker::Sized`
9    |
9    |
-    = note: expected associated type `<C as Collection<i32>>::Sibling<f32>`
-               found associated type `<<C as Collection<i32>>::Family as CollectionFamily>::Member<f32>`
+ help: consider relaxing the implicit `Sized` restriction
+    |
+ LL | trait Collection<T: ?Sized> {
12 
- error: aborting due to previous error
- error: aborting due to previous error
+ error[E0277]: the size for values of type `T` cannot be known at compilation time
+    |
+    |
+ LL | trait Collection<T> {
+    |                  - required by this bound in `Collection`
+ ...
+ LL |     type Member<T>: Collection<T, Family = Self>;
+    |                 -   ^^^^^^^^^^^^^^^^^^^^^^^^^^^^ doesn't have a size known at compile-time
+    |                 this type parameter needs to be `std::marker::Sized`
+    |
+    |
+ help: consider relaxing the implicit `Sized` restriction
+    |
+ LL | trait Collection<T: ?Sized> {
14 
- For more information about this error, try `rustc --explain E0308`.
- For more information about this error, try `rustc --explain E0308`.
+ error[E0277]: the size for values of type `T` cannot be known at compilation time
+    |
+    |
+ LL |     type Member<T> = Vec<T>;
+    |                 -    ^^^^^^ doesn't have a size known at compile-time
+    |                 this type parameter needs to be `std::marker::Sized`
+    | 
+   ::: $SRC_DIR/alloc/src/vec/mod.rs:LL:COL
+    |
+    |
+ LL | pub struct Vec<T, #[unstable(feature = "allocator_api", issue = "32838")] A: Allocator = Global> {
+    |                - required by this bound in `Vec`
+ error: aborting due to 3 previous errors
+ 
+ For more information about this error, try `rustc --explain E0277`.
16 
---
To only update this specific test, also pass `--test-args generic-associated-types/collections-project-default.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/generic-associated-types/collections-project-default.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/generic-associated-types/collections-project-default" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/generic-associated-types/collections-project-default/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0277]: the size for values of type `U` cannot be known at compilation time
  --> /checkout/src/test/ui/generic-associated-types/collections-project-default.rs:14:22
   |
LL | trait Collection<T> {
   |                  - required by this bound in `Collection`
...
LL |     type Sibling<U>: Collection<U> =
   |                  -   ^^^^^^^^^^^^^ doesn't have a size known at compile-time
   |                  this type parameter needs to be `std::marker::Sized`
   |
   |
help: consider relaxing the implicit `Sized` restriction
   |
LL | trait Collection<T: ?Sized> {

error[E0277]: the size for values of type `T` cannot be known at compilation time
  --> /checkout/src/test/ui/generic-associated-types/collections-project-default.rs:25:21
   |
   |
LL | trait Collection<T> {
   |                  - required by this bound in `Collection`
...
LL |     type Member<T>: Collection<T, Family = Self>;
   |                 -   ^^^^^^^^^^^^^^^^^^^^^^^^^^^^ doesn't have a size known at compile-time
   |                 this type parameter needs to be `std::marker::Sized`
   |
   |
help: consider relaxing the implicit `Sized` restriction
   |
LL | trait Collection<T: ?Sized> {

error[E0277]: the size for values of type `T` cannot be known at compilation time
  --> /checkout/src/test/ui/generic-associated-types/collections-project-default.rs:31:22
   |
   |
LL |     type Member<T> = Vec<T>;
   |                 -    ^^^^^^ doesn't have a size known at compile-time
   |                 this type parameter needs to be `std::marker::Sized`
   | 
  ::: /checkout/library/alloc/src/vec/mod.rs:396:16
   |
   |
LL | pub struct Vec<T, #[unstable(feature = "allocator_api", issue = "32838")] A: Allocator = Global> {
   |                - required by this bound in `Vec`
error: aborting due to 3 previous errors

For more information about this error, try `rustc --explain E0277`.

---
16    = note: see issue #44265 <https://github.com/rust-lang/rust/issues/44265> for more information
17    = help: add `#![feature(generic_associated_types)]` to the crate attributes to enable
18 
- error: aborting due to 2 previous errors
+ error[E0277]: the size for values of type `T` cannot be known at compilation time
+   --> $DIR/gat-dont-ice-on-absent-feature-2.rs:12:5
+    |
+ LL |     type Item<T>;
+    |     ------------- required by this bound in `MyTrait::Item`
+ ...
+ LL |     type Item<T> = T;
+    |     ^^^^^^^^^^-^^^^^^
+    |     |         this type parameter needs to be `std::marker::Sized`
+    |     doesn't have a size known at compile-time
20 
- For more information about this error, try `rustc --explain E0658`.
---
To only update this specific test, also pass `--test-args generic-associated-types/gat-dont-ice-on-absent-feature-2.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/generic-associated-types/gat-dont-ice-on-absent-feature-2.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/generic-associated-types/gat-dont-ice-on-absent-feature-2" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/generic-associated-types/gat-dont-ice-on-absent-feature-2/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0658]: generic associated types are unstable
   |
   |
LL |     type Item<T>;
   |
   = note: see issue #44265 <https://github.com/rust-lang/rust/issues/44265> for more information
   = help: add `#![feature(generic_associated_types)]` to the crate attributes to enable


error[E0658]: generic associated types are unstable
   |
   |
LL |     type Item<T> = T;
   |
   = note: see issue #44265 <https://github.com/rust-lang/rust/issues/44265> for more information
   = help: add `#![feature(generic_associated_types)]` to the crate attributes to enable


error[E0277]: the size for values of type `T` cannot be known at compilation time
  --> /checkout/src/test/ui/generic-associated-types/gat-dont-ice-on-absent-feature-2.rs:12:5
   |
LL |     type Item<T>;
   |     ------------- required by this bound in `MyTrait::Item`
...
LL |     type Item<T> = T;
   |     ^^^^^^^^^^-^^^^^^
   |     |         this type parameter needs to be `std::marker::Sized`
   |     doesn't have a size known at compile-time

error: aborting due to 3 previous errors
---

---- [ui] ui/generic-associated-types/generic-associated-types-where.rs stdout ----
diff of stderr:

- error[E0277]: `T` doesn't implement `std::fmt::Display`
-   --> $DIR/generic-associated-types-where.rs:20:5
+ error[E0277]: the size for values of type `T` cannot be known at compilation time
3    |
3    |
4 LL |     type Assoc2<T> = Vec<T>;
-    |     ^^^^^^^^^^^^^^^^^^^^^^^^ `T` cannot be formatted with the default formatter
+    |                 -    ^^^^^^ doesn't have a size known at compile-time
+    |                 this type parameter needs to be `std::marker::Sized`
+    | 
+   ::: $SRC_DIR/alloc/src/vec/mod.rs:LL:COL
6    |
6    |
-    = note: in format strings you may be able to use `{:?}` (or {:#?} for pretty-print) instead
- help: consider restricting type parameter `T`
-    |
- LL |     type Assoc2<T: std::fmt::Display> = Vec<T>;
-    |                  ^^^^^^^^^^^^^^^^^^^
+ LL | pub struct Vec<T, #[unstable(feature = "allocator_api", issue = "32838")] A: Allocator = Global> {
+    |                - required by this bound in `Vec`
- error[E0276]: impl has stricter requirements than trait
-   --> $DIR/generic-associated-types-where.rs:22:5
-   --> $DIR/generic-associated-types-where.rs:22:5
+ error[E0277]: the size for values of type `T` cannot be known at compilation time
15    |
15    |
- LL |     type Assoc3<T>;
-    |     --------------- definition of `Assoc3` from trait
- ...
19 LL |     type Assoc3<T> where T: Iterator = Vec<T>;
-    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ impl has extra requirement `T: Iterator`
+    |                 -                      ^^^^^^ doesn't have a size known at compile-time
+    |                 this type parameter needs to be `std::marker::Sized`
+    | 
+   ::: $SRC_DIR/alloc/src/vec/mod.rs:LL:COL
+    |
+    |
+ LL | pub struct Vec<T, #[unstable(feature = "allocator_api", issue = "32838")] A: Allocator = Global> {
+    |                - required by this bound in `Vec`
22 error: aborting due to 2 previous errors
23 

- Some errors have detailed explanations: E0276, E0277.
---
To only update this specific test, also pass `--test-args generic-associated-types/generic-associated-types-where.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/generic-associated-types/generic-associated-types-where.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/generic-associated-types/generic-associated-types-where" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/generic-associated-types/generic-associated-types-where/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0277]: the size for values of type `T` cannot be known at compilation time
  --> /checkout/src/test/ui/generic-associated-types/generic-associated-types-where.rs:20:22
   |
LL |     type Assoc2<T> = Vec<T>;
   |                 -    ^^^^^^ doesn't have a size known at compile-time
   |                 this type parameter needs to be `std::marker::Sized`
   | 
  ::: /checkout/library/alloc/src/vec/mod.rs:396:16
   |
   |
LL | pub struct Vec<T, #[unstable(feature = "allocator_api", issue = "32838")] A: Allocator = Global> {
   |                - required by this bound in `Vec`
error[E0277]: the size for values of type `T` cannot be known at compilation time
  --> /checkout/src/test/ui/generic-associated-types/generic-associated-types-where.rs:22:40
   |
   |
LL |     type Assoc3<T> where T: Iterator = Vec<T>;
   |                 -                      ^^^^^^ doesn't have a size known at compile-time
   |                 this type parameter needs to be `std::marker::Sized`
   | 
  ::: /checkout/library/alloc/src/vec/mod.rs:396:16
   |
   |
LL | pub struct Vec<T, #[unstable(feature = "allocator_api", issue = "32838")] A: Allocator = Global> {
   |                - required by this bound in `Vec`
error: aborting due to 2 previous errors

For more information about this error, try `rustc --explain E0277`.


------------------------------------------


---- [ui] ui/generic-associated-types/issue-47206-where-clause.rs stdout ----
diff of stderr:

- error[E0276]: impl has stricter requirements than trait
-   --> $DIR/issue-47206-where-clause.rs:12:5
+ error[E0277]: the size for values of type `T` cannot be known at compilation time
3    |
3    |
- LL |     type Assoc3<T>;
-    |     --------------- definition of `Assoc3` from trait
- ...
7 LL |     type Assoc3<T> where T: Iterator = Vec<T>;
-    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ impl has extra requirement `T: Iterator`
+    |                 -                      ^^^^^^ doesn't have a size known at compile-time
+    |                 this type parameter needs to be `std::marker::Sized`
+    | 
+   ::: $SRC_DIR/alloc/src/vec/mod.rs:LL:COL
+    |
+    |
+ LL | pub struct Vec<T, #[unstable(feature = "allocator_api", issue = "32838")] A: Allocator = Global> {
+    |                - required by this bound in `Vec`
10 error: aborting due to previous error
11 

- For more information about this error, try `rustc --explain E0276`.
---
To only update this specific test, also pass `--test-args generic-associated-types/issue-47206-where-clause.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/generic-associated-types/issue-47206-where-clause.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/generic-associated-types/issue-47206-where-clause" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/generic-associated-types/issue-47206-where-clause/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0277]: the size for values of type `T` cannot be known at compilation time
  --> /checkout/src/test/ui/generic-associated-types/issue-47206-where-clause.rs:12:40
   |
LL |     type Assoc3<T> where T: Iterator = Vec<T>;
   |                 -                      ^^^^^^ doesn't have a size known at compile-time
   |                 this type parameter needs to be `std::marker::Sized`
   | 
  ::: /checkout/library/alloc/src/vec/mod.rs:396:16
   |
   |
LL | pub struct Vec<T, #[unstable(feature = "allocator_api", issue = "32838")] A: Allocator = Global> {
   |                - required by this bound in `Vec`
error: aborting due to previous error

For more information about this error, try `rustc --explain E0277`.


------------------------------------------


---- [ui] ui/generic-associated-types/parameter_number_and_kind_impl.rs stdout ----
diff of stderr:

- error[E0195]: lifetime parameters or bounds on type `A` do not match the trait declaration
-   --> $DIR/parameter_number_and_kind_impl.rs:15:11
+ error[E0277]: the size for values of type `T` cannot be known at compilation time
3    |
3    |
- LL |     type A<'a>;
-    |           ---- lifetimes in impl do not match this type in trait
- ...
- LL |     type A = u32;
-    |           ^ lifetimes do not match type in trait
- 
- error[E0049]: type `B` has 1 type parameter but its trait declaration has 0 type parameters
-   --> $DIR/parameter_number_and_kind_impl.rs:17:12
-    |
- LL |     type B<'a, 'b>;
-    |            |
-    |            expected 0 type parameters
- ...
- ...
18 LL |     type B<'a, T> = Vec<T>;
-    |            ^^  ^
-    |            found 1 type parameter
- 
- 
- error[E0195]: lifetime parameters or bounds on type `C` do not match the trait declaration
-   --> $DIR/parameter_number_and_kind_impl.rs:19:11
+    |                -    ^^^^^^ doesn't have a size known at compile-time
+    |                this type parameter needs to be `std::marker::Sized`
+    | 
+   ::: $SRC_DIR/alloc/src/vec/mod.rs:LL:COL
25    |
25    |
- LL |     type C;
-    |           - lifetimes in impl do not match this type in trait
- ...
- LL |     type C<'a> = u32;
-    |           ^^^^ lifetimes do not match type in trait
+ LL | pub struct Vec<T, #[unstable(feature = "allocator_api", issue = "32838")] A: Allocator = Global> {
+    |                - required by this bound in `Vec`
31 
- error[E0049]: type `A` has 1 type parameter but its trait declaration has 0 type parameters
-   --> $DIR/parameter_number_and_kind_impl.rs:26:12
-    |
- LL |     type A<'a>;
-    |            -- expected 0 type parameters
- ...
- LL |     type A<T> = u32;
-    |            ^ found 1 type parameter
+ error: aborting due to previous error
40 
- error[E0195]: lifetime parameters or bounds on type `B` do not match the trait declaration
-   --> $DIR/parameter_number_and_kind_impl.rs:28:11
-    |
- LL |     type B<'a, 'b>;
-    |           -------- lifetimes in impl do not match this type in trait
- ...
- LL |     type B<'a> = u32;
-    |           ^^^^ lifetimes do not match type in trait
- 
- error[E0049]: type `C` has 1 type parameter but its trait declaration has 0 type parameters
-   --> $DIR/parameter_number_and_kind_impl.rs:30:12
- LL |     type C;
-    |           - expected 0 type parameters
- ...
- ...
- LL |     type C<T> = T;
-    |            ^ found 1 type parameter
- error: aborting due to 6 previous errors
- 
- Some errors have detailed explanations: E0049, E0195.
- For more information about an error, try `rustc --explain E0049`.
---
To only update this specific test, also pass `--test-args generic-associated-types/parameter_number_and_kind_impl.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/generic-associated-types/parameter_number_and_kind_impl.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/generic-associated-types/parameter_number_and_kind_impl" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/generic-associated-types/parameter_number_and_kind_impl/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0277]: the size for values of type `T` cannot be known at compilation time
  --> /checkout/src/test/ui/generic-associated-types/parameter_number_and_kind_impl.rs:17:21
   |
LL |     type B<'a, T> = Vec<T>;
   |                -    ^^^^^^ doesn't have a size known at compile-time
   |                this type parameter needs to be `std::marker::Sized`
   | 
  ::: /checkout/library/alloc/src/vec/mod.rs:396:16
   |
   |
LL | pub struct Vec<T, #[unstable(feature = "allocator_api", issue = "32838")] A: Allocator = Global> {
   |                - required by this bound in `Vec`
error: aborting due to previous error

For more information about this error, try `rustc --explain E0277`.

---
test result: FAILED. 12052 passed; 6 failed; 102 ignored; 0 measured; 0 filtered out; finished in 102.97s



command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--suite" "ui" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-10/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python3" "--lldb-python" "/usr/bin/python3" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "10.0.0" "--llvm-components" "aarch64 aarch64asmparser aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets amdgpu amdgpuasmparser amdgpucodegen amdgpudesc amdgpudisassembler amdgpuinfo amdgpuutils analysis arm armasmparser armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter avr avrasmparser avrcodegen avrdesc avrdisassembler avrinfo binaryformat bitreader bitstreamreader bitwriter bpf bpfasmparser bpfcodegen bpfdesc bpfdisassembler bpfinfo cfguard codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfogsym debuginfomsf debuginfopdb demangle dlltooldriver dwarflinker engine executionengine frontendopenmp fuzzmutate globalisel hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interpreter ipo irreader jitlink lanai lanaiasmparser lanaicodegen lanaidesc lanaidisassembler lanaiinfo libdriver lineeditor linker lto mc mca mcdisassembler mcjit mcparser mips mipsasmparser mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmparser msp430codegen msp430desc msp430disassembler msp430info native nativecodegen nvptx nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcerror orcjit passes perfjitevents powerpc powerpcasmparser powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata remarks riscv riscvasmparser riscvcodegen riscvdesc riscvdisassembler riscvinfo riscvutils runtimedyld scalaropts selectiondag sparc sparcasmparser sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target textapi transformutils vectorize webassembly webassemblyasmparser webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo windowsmanifest x86 x86asmparser x86codegen x86desc x86disassembler x86info x86utils xcore xcorecodegen xcoredesc xcoredisassembler xcoreinfo xray" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--channel" "nightly" "--color" "always"


Build completed unsuccessfully in 0:10:03
