plain
....................i................................................................... 1496/13429
........................................................................................ 1584/13429
........................................................................................ 1672/13429
..........................................................i......ii..................... 1760/13429
........................FF....................F........................F................ 1848/13429
...................................................FFF...FFFF....F.F...FF..F.F.......... 1936/13429
........................................................................................ 2112/13429
........................................................................................ 2200/13429
........................................................................................ 2288/13429
........................................................................................ 2376/13429
---

---- [ui] src/test/ui/coherence/coherence-all-remote.rs stdout ----
diff of stderr:

1 error[E0210]: type parameter `T` must be used as the type parameter for some local type (e.g., `MyStruct<T>`)
-   --> $DIR/coherence-all-remote.rs:6:6
+   --> $DIR/coherence-all-remote.rs:6:17
3    |
4 LL | impl<T> Remote1<T> for isize { }
-    |      ^ type parameter `T` must be used as the type parameter for some local type
+    |                 ^ type parameter `T` must be used as the type parameter for some local type
6    |
7    = note: implementing a foreign trait is only possible if at least one of the types for which it is implemented is local
8    = note: only traits defined in the current crate can be implemented for a type parameter

The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/coherence/coherence-all-remote/coherence-all-remote.stderr
To update references, rerun the tests and pass the `--bless` flag
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args coherence/coherence-all-remote.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/coherence/coherence-all-remote.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/coherence/coherence-all-remote" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/coherence/coherence-all-remote/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0210]: type parameter `T` must be used as the type parameter for some local type (e.g., `MyStruct<T>`)
   |
   |
LL | impl<T> Remote1<T> for isize { }
   |                 ^ type parameter `T` must be used as the type parameter for some local type
   |
   = note: implementing a foreign trait is only possible if at least one of the types for which it is implemented is local
   = note: only traits defined in the current crate can be implemented for a type parameter
error: aborting due to previous error

For more information about this error, try `rustc --explain E0210`.
------------------------------------------
------------------------------------------


---- [ui] src/test/ui/coherence/coherence-bigint-param.rs stdout ----
diff of stderr:

1 error[E0210]: type parameter `T` must be covered by another type when it appears before the first local type (`BigInt`)
-   --> $DIR/coherence-bigint-param.rs:8:6
+   --> $DIR/coherence-bigint-param.rs:8:29
3    |
4 LL | impl<T> Remote1<BigInt> for T { }
-    |      ^ type parameter `T` must be covered by another type when it appears before the first local type (`BigInt`)
+    |                             ^ type parameter `T` must be covered by another type when it appears before the first local type (`BigInt`)
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
6    |
7    = note: implementing a foreign trait is only possible if at least one of the types for which it is implemented is local, and no uncovered type parameters or associated types appear before that first local type
8    = note: in this case, 'before' refers to the following order: `impl<..> ForeignTrait<T1, ..., Tn> for T0`, where `T0` is the first and `Tn` is the last

The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/coherence/coherence-bigint-param/coherence-bigint-param.stderr
To update references, rerun the tests and pass the `--bless` flag
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args coherence/coherence-bigint-param.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/coherence/coherence-bigint-param.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/coherence/coherence-bigint-param" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/coherence/coherence-bigint-param/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0210]: type parameter `T` must be covered by another type when it appears before the first local type (`BigInt`)
   |
   |
LL | impl<T> Remote1<BigInt> for T { }
   |                             ^ type parameter `T` must be covered by another type when it appears before the first local type (`BigInt`)
   |
   = note: implementing a foreign trait is only possible if at least one of the types for which it is implemented is local, and no uncovered type parameters or associated types appear before that first local type
   = note: in this case, 'before' refers to the following order: `impl<..> ForeignTrait<T1, ..., Tn> for T0`, where `T0` is the first and `Tn` is the last
error: aborting due to previous error

For more information about this error, try `rustc --explain E0210`.
------------------------------------------
------------------------------------------


---- [ui] src/test/ui/coherence/coherence-cross-crate-conflict.rs stdout ----
diff of stderr:

1 error[E0210]: type parameter `A` must be used as the type parameter for some local type (e.g., `MyStruct<A>`)
-   --> $DIR/coherence-cross-crate-conflict.rs:9:6
+   --> $DIR/coherence-cross-crate-conflict.rs:9:17
3    |
4 LL | impl<A> Foo for A {
-    |      ^ type parameter `A` must be used as the type parameter for some local type
+    |                 ^ type parameter `A` must be used as the type parameter for some local type
6    |
7    = note: implementing a foreign trait is only possible if at least one of the types for which it is implemented is local
8    = note: only traits defined in the current crate can be implemented for a type parameter

The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/coherence/coherence-cross-crate-conflict/coherence-cross-crate-conflict.stderr
To update references, rerun the tests and pass the `--bless` flag
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args coherence/coherence-cross-crate-conflict.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/coherence/coherence-cross-crate-conflict.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/coherence/coherence-cross-crate-conflict" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/coherence/coherence-cross-crate-conflict/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0210]: type parameter `A` must be used as the type parameter for some local type (e.g., `MyStruct<A>`)
   |
   |
LL | impl<A> Foo for A { //~ ERROR E0210
   |                 ^ type parameter `A` must be used as the type parameter for some local type
   |
   = note: implementing a foreign trait is only possible if at least one of the types for which it is implemented is local
   = note: only traits defined in the current crate can be implemented for a type parameter
error: aborting due to previous error

For more information about this error, try `rustc --explain E0210`.
------------------------------------------
------------------------------------------


---- [ui] src/test/ui/coherence/coherence-lone-type-parameter.rs stdout ----
diff of stderr:

1 error[E0210]: type parameter `T` must be used as the type parameter for some local type (e.g., `MyStruct<T>`)
-   --> $DIR/coherence-lone-type-parameter.rs:6:6
+   --> $DIR/coherence-lone-type-parameter.rs:6:20
3    |
4 LL | impl<T> Remote for T { }
-    |      ^ type parameter `T` must be used as the type parameter for some local type
+    |                    ^ type parameter `T` must be used as the type parameter for some local type
6    |
7    = note: implementing a foreign trait is only possible if at least one of the types for which it is implemented is local
8    = note: only traits defined in the current crate can be implemented for a type parameter

The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/coherence/coherence-lone-type-parameter/coherence-lone-type-parameter.stderr
To update references, rerun the tests and pass the `--bless` flag
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args coherence/coherence-lone-type-parameter.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/coherence/coherence-lone-type-parameter.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/coherence/coherence-lone-type-parameter" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/coherence/coherence-lone-type-parameter/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0210]: type parameter `T` must be used as the type parameter for some local type (e.g., `MyStruct<T>`)
   |
   |
LL | impl<T> Remote for T { }
   |                    ^ type parameter `T` must be used as the type parameter for some local type
   |
   = note: implementing a foreign trait is only possible if at least one of the types for which it is implemented is local
   = note: only traits defined in the current crate can be implemented for a type parameter
error: aborting due to previous error

For more information about this error, try `rustc --explain E0210`.
------------------------------------------
------------------------------------------


---- [ui] src/test/ui/coherence/impl[t]-foreign-for-fundamental[t].rs stdout ----
diff of stderr:

1 error[E0210]: type parameter `T` must be used as the type parameter for some local type (e.g., `MyStruct<T>`)
-   --> $DIR/impl[t]-foreign-for-fundamental[t].rs:10:6
+   --> $DIR/impl[t]-foreign-for-fundamental[t].rs:10:20
3    |
4 LL | impl<T> Remote for Box<T> {
-    |      ^ type parameter `T` must be used as the type parameter for some local type
+    |                    ^^^^^^ type parameter `T` must be used as the type parameter for some local type
6    |
7    = note: implementing a foreign trait is only possible if at least one of the types for which it is implemented is local
8    = note: only traits defined in the current crate can be implemented for a type parameter

The actual stderr differed from the expected stderr.
The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/coherence/impl[t]-foreign-for-fundamental[t]/impl[t]-foreign-for-fundamental[t].stderr
To only update this specific test, also pass `--test-args coherence/impl[t]-foreign-for-fundamental[t].rs`

error: 1 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/coherence/impl[t]-foreign-for-fundamental[t].rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/coherence/impl[t]-foreign-for-fundamental[t]" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--crate-name=test" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/coherence/impl[t]-foreign-for-fundamental[t]/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0210]: type parameter `T` must be used as the type parameter for some local type (e.g., `MyStruct<T>`)
  --> /checkout/src/test/ui/coherence/impl[t]-foreign-for-fundamental[t].rs:10:20
   |
LL | impl<T> Remote for Box<T> {
   |                    ^^^^^^ type parameter `T` must be used as the type parameter for some local type
   |
   = note: implementing a foreign trait is only possible if at least one of the types for which it is implemented is local
   = note: only traits defined in the current crate can be implemented for a type parameter
error: aborting due to previous error

For more information about this error, try `rustc --explain E0210`.
------------------------------------------
------------------------------------------


---- [ui] src/test/ui/coherence/impl[t]-foreign[foreign]-for-fundamental[t].rs stdout ----
diff of stderr:

1 error[E0210]: type parameter `T` must be used as the type parameter for some local type (e.g., `MyStruct<T>`)
-   --> $DIR/impl[t]-foreign[foreign]-for-fundamental[t].rs:10:6
+   --> $DIR/impl[t]-foreign[foreign]-for-fundamental[t].rs:10:26
3    |
4 LL | impl<T> Remote1<u32> for Box<T> {
-    |      ^ type parameter `T` must be used as the type parameter for some local type
+    |                          ^^^^^^ type parameter `T` must be used as the type parameter for some local type
6    |
7    = note: implementing a foreign trait is only possible if at least one of the types for which it is implemented is local
8    = note: only traits defined in the current crate can be implemented for a type parameter
9 
9 
10 error[E0210]: type parameter `T` must be used as the type parameter for some local type (e.g., `MyStruct<T>`)
-   --> $DIR/impl[t]-foreign[foreign]-for-fundamental[t].rs:14:10
+   --> $DIR/impl[t]-foreign[foreign]-for-fundamental[t].rs:14:30
12    |
13 LL | impl<'a, T> Remote1<u32> for &'a T {
-    |          ^ type parameter `T` must be used as the type parameter for some local type
+    |                              ^^^^^ type parameter `T` must be used as the type parameter for some local type
15    |
16    = note: implementing a foreign trait is only possible if at least one of the types for which it is implemented is local
17    = note: only traits defined in the current crate can be implemented for a type parameter

The actual stderr differed from the expected stderr.
The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/coherence/impl[t]-foreign[foreign]-for-fundamental[t]/impl[t]-foreign[foreign]-for-fundamental[t].stderr
To only update this specific test, also pass `--test-args coherence/impl[t]-foreign[foreign]-for-fundamental[t].rs`

error: 1 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/coherence/impl[t]-foreign[foreign]-for-fundamental[t].rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/coherence/impl[t]-foreign[foreign]-for-fundamental[t]" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--crate-name=test" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/coherence/impl[t]-foreign[foreign]-for-fundamental[t]/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0210]: type parameter `T` must be used as the type parameter for some local type (e.g., `MyStruct<T>`)
  --> /checkout/src/test/ui/coherence/impl[t]-foreign[foreign]-for-fundamental[t].rs:10:26
   |
LL | impl<T> Remote1<u32> for Box<T> {
   |                          ^^^^^^ type parameter `T` must be used as the type parameter for some local type
   |
   = note: implementing a foreign trait is only possible if at least one of the types for which it is implemented is local
   = note: only traits defined in the current crate can be implemented for a type parameter

error[E0210]: type parameter `T` must be used as the type parameter for some local type (e.g., `MyStruct<T>`)
  --> /checkout/src/test/ui/coherence/impl[t]-foreign[foreign]-for-fundamental[t].rs:14:30
   |
LL | impl<'a, T> Remote1<u32> for &'a T {
   |                              ^^^^^ type parameter `T` must be used as the type parameter for some local type
   |
   = note: implementing a foreign trait is only possible if at least one of the types for which it is implemented is local
   = note: only traits defined in the current crate can be implemented for a type parameter
error: aborting due to 2 previous errors

For more information about this error, try `rustc --explain E0210`.
------------------------------------------
------------------------------------------


---- [ui] src/test/ui/coherence/impl[t]-foreign[foreign]-for-t.rs stdout ----
diff of stderr:

1 error[E0210]: type parameter `T` must be used as the type parameter for some local type (e.g., `MyStruct<T>`)
-   --> $DIR/impl[t]-foreign[foreign]-for-t.rs:10:6
3    |
3    |
4 LL | impl<T> Remote1<u32> for T {
-    |      ^ type parameter `T` must be used as the type parameter for some local type
+    |                          ^ type parameter `T` must be used as the type parameter for some local type
6    |
7    = note: implementing a foreign trait is only possible if at least one of the types for which it is implemented is local
8    = note: only traits defined in the current crate can be implemented for a type parameter

The actual stderr differed from the expected stderr.
The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/coherence/impl[t]-foreign[foreign]-for-t/impl[t]-foreign[foreign]-for-t.stderr
To only update this specific test, also pass `--test-args coherence/impl[t]-foreign[foreign]-for-t.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/coherence/impl[t]-foreign[foreign]-for-t.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/coherence/impl[t]-foreign[foreign]-for-t" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--crate-name=test" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/coherence/impl[t]-foreign[foreign]-for-t/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0210]: type parameter `T` must be used as the type parameter for some local type (e.g., `MyStruct<T>`)
   |
   |
LL | impl<T> Remote1<u32> for T {
   |                          ^ type parameter `T` must be used as the type parameter for some local type
   |
   = note: implementing a foreign trait is only possible if at least one of the types for which it is implemented is local
   = note: only traits defined in the current crate can be implemented for a type parameter
error: aborting due to previous error

For more information about this error, try `rustc --explain E0210`.
------------------------------------------
------------------------------------------


---- [ui] src/test/ui/coherence/impl[t]-foreign[fundamental[t]_local]-for-foreign.rs stdout ----
diff of stderr:

1 error[E0210]: type parameter `T` must be covered by another type when it appears before the first local type (`Local`)
-   --> $DIR/impl[t]-foreign[fundamental[t]_local]-for-foreign.rs:10:6
3    |
3    |
4 LL | impl<T> Remote2<Box<T>, Local> for u32 {
-    |      ^ type parameter `T` must be covered by another type when it appears before the first local type (`Local`)
+    |                 ^^^^^^ type parameter `T` must be covered by another type when it appears before the first local type (`Local`)
6    |
7    = note: implementing a foreign trait is only possible if at least one of the types for which it is implemented is local, and no uncovered type parameters or associated types appear before that first local type
8    = note: in this case, 'before' refers to the following order: `impl<..> ForeignTrait<T1, ..., Tn> for T0`, where `T0` is the first and `Tn` is the last
9 
9 
10 error[E0210]: type parameter `T` must be covered by another type when it appears before the first local type (`Local`)
-   --> $DIR/impl[t]-foreign[fundamental[t]_local]-for-foreign.rs:14:10
12    |
12    |
13 LL | impl<'a, T> Remote2<&'a T, Local> for u32 {
-    |          ^ type parameter `T` must be covered by another type when it appears before the first local type (`Local`)
+    |                     ^^^^^ type parameter `T` must be covered by another type when it appears before the first local type (`Local`)
15    |
16    = note: implementing a foreign trait is only possible if at least one of the types for which it is implemented is local, and no uncovered type parameters or associated types appear before that first local type
17    = note: in this case, 'before' refers to the following order: `impl<..> ForeignTrait<T1, ..., Tn> for T0`, where `T0` is the first and `Tn` is the last

The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/coherence/impl[t]-foreign[fundamental[t]_local]-for-foreign/impl[t]-foreign[fundamental[t]_local]-for-foreign.stderr
To update references, rerun the tests and pass the `--bless` flag
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args coherence/impl[t]-foreign[fundamental[t]_local]-for-foreign.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/coherence/impl[t]-foreign[fundamental[t]_local]-for-foreign.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/coherence/impl[t]-foreign[fundamental[t]_local]-for-foreign" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--crate-name=test" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/coherence/impl[t]-foreign[fundamental[t]_local]-for-foreign/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0210]: type parameter `T` must be covered by another type when it appears before the first local type (`Local`)
   |
   |
LL | impl<T> Remote2<Box<T>, Local> for u32 {
   |                 ^^^^^^ type parameter `T` must be covered by another type when it appears before the first local type (`Local`)
   |
   = note: implementing a foreign trait is only possible if at least one of the types for which it is implemented is local, and no uncovered type parameters or associated types appear before that first local type
   = note: in this case, 'before' refers to the following order: `impl<..> ForeignTrait<T1, ..., Tn> for T0`, where `T0` is the first and `Tn` is the last

error[E0210]: type parameter `T` must be covered by another type when it appears before the first local type (`Local`)
   |
   |
LL | impl<'a, T> Remote2<&'a T, Local> for u32 {
   |                     ^^^^^ type parameter `T` must be covered by another type when it appears before the first local type (`Local`)
   |
   = note: implementing a foreign trait is only possible if at least one of the types for which it is implemented is local, and no uncovered type parameters or associated types appear before that first local type
   = note: in this case, 'before' refers to the following order: `impl<..> ForeignTrait<T1, ..., Tn> for T0`, where `T0` is the first and `Tn` is the last
error: aborting due to 2 previous errors

For more information about this error, try `rustc --explain E0210`.
------------------------------------------
------------------------------------------


---- [ui] src/test/ui/coherence/impl[t]-foreign[fundamental[t]]-for-foreign.rs stdout ----
diff of stderr:

1 error[E0210]: type parameter `T` must be used as the type parameter for some local type (e.g., `MyStruct<T>`)
-   --> $DIR/impl[t]-foreign[fundamental[t]]-for-foreign.rs:10:6
3    |
3    |
4 LL | impl<T> Remote1<Box<T>> for u32 {
-    |      ^ type parameter `T` must be used as the type parameter for some local type
+    |                 ^^^^^^ type parameter `T` must be used as the type parameter for some local type
6    |
7    = note: implementing a foreign trait is only possible if at least one of the types for which it is implemented is local
8    = note: only traits defined in the current crate can be implemented for a type parameter
9 
9 
10 error[E0210]: type parameter `T` must be used as the type parameter for some local type (e.g., `MyStruct<T>`)
-   --> $DIR/impl[t]-foreign[fundamental[t]]-for-foreign.rs:14:10
12    |
12    |
13 LL | impl<'a, T> Remote1<&'a T> for u32 {
-    |          ^ type parameter `T` must be used as the type parameter for some local type
+    |                     ^^^^^ type parameter `T` must be used as the type parameter for some local type
15    |
16    = note: implementing a foreign trait is only possible if at least one of the types for which it is implemented is local
17    = note: only traits defined in the current crate can be implemented for a type parameter

The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/coherence/impl[t]-foreign[fundamental[t]]-for-foreign/impl[t]-foreign[fundamental[t]]-for-foreign.stderr
To update references, rerun the tests and pass the `--bless` flag
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args coherence/impl[t]-foreign[fundamental[t]]-for-foreign.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/coherence/impl[t]-foreign[fundamental[t]]-for-foreign.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/coherence/impl[t]-foreign[fundamental[t]]-for-foreign" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--crate-name=test" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/coherence/impl[t]-foreign[fundamental[t]]-for-foreign/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0210]: type parameter `T` must be used as the type parameter for some local type (e.g., `MyStruct<T>`)
   |
   |
LL | impl<T> Remote1<Box<T>> for u32 {
   |                 ^^^^^^ type parameter `T` must be used as the type parameter for some local type
   |
   = note: implementing a foreign trait is only possible if at least one of the types for which it is implemented is local
   = note: only traits defined in the current crate can be implemented for a type parameter

error[E0210]: type parameter `T` must be used as the type parameter for some local type (e.g., `MyStruct<T>`)
   |
   |
LL | impl<'a, T> Remote1<&'a T> for u32 {
   |                     ^^^^^ type parameter `T` must be used as the type parameter for some local type
   |
   = note: implementing a foreign trait is only possible if at least one of the types for which it is implemented is local
   = note: only traits defined in the current crate can be implemented for a type parameter
error: aborting due to 2 previous errors

For more information about this error, try `rustc --explain E0210`.
------------------------------------------
------------------------------------------


---- [ui] src/test/ui/coherence/impl[t]-foreign[fundamental[t]]-for-fundamental[t].rs stdout ----
diff of stderr:

1 error[E0210]: type parameter `T` must be used as the type parameter for some local type (e.g., `MyStruct<T>`)
-   --> $DIR/impl[t]-foreign[fundamental[t]]-for-fundamental[t].rs:10:10
+   --> $DIR/impl[t]-foreign[fundamental[t]]-for-fundamental[t].rs:10:33
3    |
4 LL | impl<'a, T> Remote1<Box<T>> for &'a T {
-    |          ^ type parameter `T` must be used as the type parameter for some local type
+    |                                 ^^^^^ type parameter `T` must be used as the type parameter for some local type
6    |
7    = note: implementing a foreign trait is only possible if at least one of the types for which it is implemented is local
8    = note: only traits defined in the current crate can be implemented for a type parameter
9 
9 
10 error[E0210]: type parameter `T` must be used as the type parameter for some local type (e.g., `MyStruct<T>`)
-   --> $DIR/impl[t]-foreign[fundamental[t]]-for-fundamental[t].rs:13:10
+   --> $DIR/impl[t]-foreign[fundamental[t]]-for-fundamental[t].rs:13:32
12    |
13 LL | impl<'a, T> Remote1<&'a T> for Box<T> {
-    |          ^ type parameter `T` must be used as the type parameter for some local type
+    |                                ^^^^^^ type parameter `T` must be used as the type parameter for some local type
15    |
16    = note: implementing a foreign trait is only possible if at least one of the types for which it is implemented is local
17    = note: only traits defined in the current crate can be implemented for a type parameter

The actual stderr differed from the expected stderr.
The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/coherence/impl[t]-foreign[fundamental[t]]-for-fundamental[t]/impl[t]-foreign[fundamental[t]]-for-fundamental[t].stderr
To only update this specific test, also pass `--test-args coherence/impl[t]-foreign[fundamental[t]]-for-fundamental[t].rs`

error: 1 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/coherence/impl[t]-foreign[fundamental[t]]-for-fundamental[t].rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/coherence/impl[t]-foreign[fundamental[t]]-for-fundamental[t]" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--crate-name=test" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/coherence/impl[t]-foreign[fundamental[t]]-for-fundamental[t]/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0210]: type parameter `T` must be used as the type parameter for some local type (e.g., `MyStruct<T>`)
  --> /checkout/src/test/ui/coherence/impl[t]-foreign[fundamental[t]]-for-fundamental[t].rs:10:33
   |
LL | impl<'a, T> Remote1<Box<T>> for &'a T {
   |                                 ^^^^^ type parameter `T` must be used as the type parameter for some local type
   |
   = note: implementing a foreign trait is only possible if at least one of the types for which it is implemented is local
   = note: only traits defined in the current crate can be implemented for a type parameter

error[E0210]: type parameter `T` must be used as the type parameter for some local type (e.g., `MyStruct<T>`)
  --> /checkout/src/test/ui/coherence/impl[t]-foreign[fundamental[t]]-for-fundamental[t].rs:13:32
   |
LL | impl<'a, T> Remote1<&'a T> for Box<T> {
   |                                ^^^^^^ type parameter `T` must be used as the type parameter for some local type
   |
   = note: implementing a foreign trait is only possible if at least one of the types for which it is implemented is local
   = note: only traits defined in the current crate can be implemented for a type parameter
error: aborting due to 2 previous errors

For more information about this error, try `rustc --explain E0210`.
------------------------------------------
------------------------------------------


---- [ui] src/test/ui/coherence/impl[t]-foreign[fundamental[t]]-for-t.rs stdout ----
diff of stderr:

1 error[E0210]: type parameter `T` must be used as the type parameter for some local type (e.g., `MyStruct<T>`)
-   --> $DIR/impl[t]-foreign[fundamental[t]]-for-t.rs:10:6
3    |
3    |
4 LL | impl<T> Remote1<Box<T>> for T {
-    |      ^ type parameter `T` must be used as the type parameter for some local type
+    |                             ^ type parameter `T` must be used as the type parameter for some local type
6    |
7    = note: implementing a foreign trait is only possible if at least one of the types for which it is implemented is local
8    = note: only traits defined in the current crate can be implemented for a type parameter
9 
