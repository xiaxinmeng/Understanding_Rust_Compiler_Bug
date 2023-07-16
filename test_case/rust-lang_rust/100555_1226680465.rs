plain

---- [ui] src/test/ui/coherence/issue-100191.rs stdout ----
diff of stderr:

1 error[E0210]: type parameter `T` must be used as the type parameter for some local type (e.g., `MyStruct<T>`)
-   --> $DIR/issue-100191.rs:18:6
3    |
3    |
4 LL | impl<T> From<<A<T> as Z>::Assoc> for T {}
-    |      ^ type parameter `T` must be used as the type parameter for some local type
+    |                                      ^ type parameter `T` must be used as the type parameter for some local type
6    |
7    = note: implementing a foreign trait is only possible if at least one of the types for which it is implemented is local
8    = note: only traits defined in the current crate can be implemented for a type parameter

The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/coherence/issue-100191/issue-100191.stderr
To update references, rerun the tests and pass the `--bless` flag
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args coherence/issue-100191.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/coherence/issue-100191.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/coherence/issue-100191" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/coherence/issue-100191/auxiliary"
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
stdout: none
--- stderr -------------------------------
error[E0210]: type parameter `T` must be used as the type parameter for some local type (e.g., `MyStruct<T>`)
   |
   |
LL | impl<T> From<<A<T> as Z>::Assoc> for T {}
   |                                      ^ type parameter `T` must be used as the type parameter for some local type
   |
   = note: implementing a foreign trait is only possible if at least one of the types for which it is implemented is local
   = note: only traits defined in the current crate can be implemented for a type parameter
error: aborting due to previous error

For more information about this error, try `rustc --explain E0210`.
------------------------------------------
------------------------------------------


---- [ui] src/test/ui/error-codes/e0119/issue-28981.rs stdout ----
diff of stderr:

1 error[E0210]: type parameter `Foo` must be used as the type parameter for some local type (e.g., `MyStruct<Foo>`)
-   --> $DIR/issue-28981.rs:5:6
3    |
3    |
4 LL | impl<Foo> Deref for Foo { }
-    |      ^^^ type parameter `Foo` must be used as the type parameter for some local type
+    |                     ^^^ type parameter `Foo` must be used as the type parameter for some local type
6    |
7    = note: implementing a foreign trait is only possible if at least one of the types for which it is implemented is local
8    = note: only traits defined in the current crate can be implemented for a type parameter

The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/error-codes/e0119/issue-28981/issue-28981.stderr
To update references, rerun the tests and pass the `--bless` flag
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args error-codes/e0119/issue-28981.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/error-codes/e0119/issue-28981.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/error-codes/e0119/issue-28981" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/error-codes/e0119/issue-28981/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0210]: type parameter `Foo` must be used as the type parameter for some local type (e.g., `MyStruct<Foo>`)
   |
   |
LL | impl<Foo> Deref for Foo { } //~ ERROR must be used
   |                     ^^^ type parameter `Foo` must be used as the type parameter for some local type
   |
   = note: implementing a foreign trait is only possible if at least one of the types for which it is implemented is local
   = note: only traits defined in the current crate can be implemented for a type parameter
error: aborting due to previous error

For more information about this error, try `rustc --explain E0210`.
------------------------------------------
------------------------------------------


---- [ui] src/test/ui/issues/issue-41974.rs stdout ----
diff of stderr:

1 error[E0210]: type parameter `T` must be used as the type parameter for some local type (e.g., `MyStruct<T>`)
-   --> $DIR/issue-41974.rs:7:6
3    |
3    |
4 LL | impl<T> Drop for T where T: A {
-    |      ^ type parameter `T` must be used as the type parameter for some local type
+    |                  ^ type parameter `T` must be used as the type parameter for some local type
6    |
7    = note: implementing a foreign trait is only possible if at least one of the types for which it is implemented is local
8    = note: only traits defined in the current crate can be implemented for a type parameter

The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-41974/issue-41974.stderr
To update references, rerun the tests and pass the `--bless` flag
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args issues/issue-41974.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues/issue-41974.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-41974" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-41974/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0210]: type parameter `T` must be used as the type parameter for some local type (e.g., `MyStruct<T>`)
   |
   |
LL | impl<T> Drop for T where T: A {
   |                  ^ type parameter `T` must be used as the type parameter for some local type
   |
   = note: implementing a foreign trait is only possible if at least one of the types for which it is implemented is local
   = note: only traits defined in the current crate can be implemented for a type parameter

error[E0120]: the `Drop` trait may only be implemented for local structs, enums, and unions
   |
   |
LL | impl<T> Drop for T where T: A {
   |                  ^ must be a struct, enum, or union in the current crate
error: aborting due to 2 previous errors

Some errors have detailed explanations: E0120, E0210.
For more information about an error, try `rustc --explain E0120`.
For more information about an error, try `rustc --explain E0120`.
------------------------------------------


---- [ui] src/test/ui/orphan-check-diagnostics.rs stdout ----
diff of stderr:

1 error[E0210]: type parameter `T` must be used as the type parameter for some local type (e.g., `MyStruct<T>`)
-   --> $DIR/orphan-check-diagnostics.rs:11:6
+   --> $DIR/orphan-check-diagnostics.rs:11:25
3    |
4 LL | impl<T> RemoteTrait for T where T: LocalTrait {}
-    |      ^ type parameter `T` must be used as the type parameter for some local type
+    |                         ^ type parameter `T` must be used as the type parameter for some local type
6    |
7    = note: implementing a foreign trait is only possible if at least one of the types for which it is implemented is local
8    = note: only traits defined in the current crate can be implemented for a type parameter

The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/orphan-check-diagnostics/orphan-check-diagnostics.stderr
To update references, rerun the tests and pass the `--bless` flag
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args orphan-check-diagnostics.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/orphan-check-diagnostics.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/orphan-check-diagnostics" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/orphan-check-diagnostics/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0210]: type parameter `T` must be used as the type parameter for some local type (e.g., `MyStruct<T>`)
   |
   |
LL | impl<T> RemoteTrait for T where T: LocalTrait {}
   |                         ^ type parameter `T` must be used as the type parameter for some local type
   |
   = note: implementing a foreign trait is only possible if at least one of the types for which it is implemented is local
   = note: only traits defined in the current crate can be implemented for a type parameter
error: aborting due to previous error

For more information about this error, try `rustc --explain E0210`.
------------------------------------------
------------------------------------------


---- [ui] src/test/ui/specialization/issue-43037.rs stdout ----
diff of stderr:

1 error[E0210]: type parameter `T` must be used as the type parameter for some local type (e.g., `MyStruct<T>`)
-   --> $DIR/issue-43037.rs:17:6
3    |
3    |
4 LL | impl<T> From<<A<T> as Z>::Assoc> for T {}
-    |      ^ type parameter `T` must be used as the type parameter for some local type
+    |                                      ^ type parameter `T` must be used as the type parameter for some local type
6    |
7    = note: implementing a foreign trait is only possible if at least one of the types for which it is implemented is local
8    = note: only traits defined in the current crate can be implemented for a type parameter

The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/specialization/issue-43037/issue-43037.stderr
To update references, rerun the tests and pass the `--bless` flag
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args specialization/issue-43037.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/specialization/issue-43037.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/specialization/issue-43037" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/specialization/issue-43037/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0210]: type parameter `T` must be used as the type parameter for some local type (e.g., `MyStruct<T>`)
   |
   |
LL | impl<T> From<<A<T> as Z>::Assoc> for T {}
   |                                      ^ type parameter `T` must be used as the type parameter for some local type
   |
   = note: implementing a foreign trait is only possible if at least one of the types for which it is implemented is local
   = note: only traits defined in the current crate can be implemented for a type parameter
error: aborting due to previous error

For more information about this error, try `rustc --explain E0210`.
------------------------------------------
------------------------------------------


---- [ui] src/test/ui/type-alias-impl-trait/incoherent-assoc-imp-trait.rs stdout ----
diff of stderr:

- error[E0210]: type parameter `F` must be used as the type parameter for some local type (e.g., `MyStruct<F>`)
-   --> $DIR/incoherent-assoc-imp-trait.rs:10:6
+ error[E0210]: type parameter `F` as argument to a fundamental type must be used as the type parameter for some local type (e.g., `MyStruct<F>`)
3    |
3    |
4 LL | impl<F> FnOnce<()> for &F {
-    |      ^ type parameter `F` must be used as the type parameter for some local type
+    |                        ^^ type parameter `F` as argument to a fundamental type must be used as the type parameter for some local type
6    |
7    = note: implementing a foreign trait is only possible if at least one of the types for which it is implemented is local
8    = note: only traits defined in the current crate can be implemented for a type parameter

The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/type-alias-impl-trait/incoherent-assoc-imp-trait/incoherent-assoc-imp-trait.stderr
To update references, rerun the tests and pass the `--bless` flag
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args type-alias-impl-trait/incoherent-assoc-imp-trait.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/type-alias-impl-trait/incoherent-assoc-imp-trait.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/type-alias-impl-trait/incoherent-assoc-imp-trait" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/type-alias-impl-trait/incoherent-assoc-imp-trait/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0210]: type parameter `F` as argument to a fundamental type must be used as the type parameter for some local type (e.g., `MyStruct<F>`)
   |
   |
LL | impl<F> FnOnce<()> for &F {
   |                        ^^ type parameter `F` as argument to a fundamental type must be used as the type parameter for some local type
   |
   = note: implementing a foreign trait is only possible if at least one of the types for which it is implemented is local
   = note: only traits defined in the current crate can be implemented for a type parameter
error: aborting due to previous error

For more information about this error, try `rustc --explain E0210`.
------------------------------------------
