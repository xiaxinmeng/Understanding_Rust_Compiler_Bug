plain
---- [ui] src/test/ui/consts/const-size_of-cycle.rs stdout ----
diff of stderr:

16    |                 ^^^^^^^^^^^^^^^^^^^^^^^^^^
17    = note: ...which requires computing layout of `Foo`...
18    = note: ...which requires computing layout of `[u8; _]`...
-    = note: ...which requires normalizing `[u8; _]`...
+    = note: ...which requires normalizing `std::mem::size_of::<Foo>()`...
20    = note: ...which again requires evaluating type-level constant, completing the cycle
21 note: cycle used when checking that `Foo` is well-formed


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-size_of-cycle/const-size_of-cycle.stderr
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-size_of-cycle/const-size_of-cycle.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args consts/const-size_of-cycle.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/const-size_of-cycle.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-size_of-cycle" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-size_of-cycle/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0391]: cycle detected when evaluating type-level constant
   |
   |
LL |     bytes: [u8; std::mem::size_of::<Foo>()]
   |
   |
note: ...which requires const-evaluating + checking `Foo::bytes::{constant#0}`...
   |
   |
LL |     bytes: [u8; std::mem::size_of::<Foo>()]
   |                 ^^^^^^^^^^^^^^^^^^^^^^^^^^
note: ...which requires const-evaluating + checking `Foo::bytes::{constant#0}`...
   |
   |
LL |     bytes: [u8; std::mem::size_of::<Foo>()]
   |                 ^^^^^^^^^^^^^^^^^^^^^^^^^^
   = note: ...which requires computing layout of `Foo`...
   = note: ...which requires computing layout of `[u8; _]`...
   = note: ...which requires normalizing `std::mem::size_of::<Foo>()`...
   = note: ...which again requires evaluating type-level constant, completing the cycle
note: cycle used when checking that `Foo` is well-formed
   |
LL | struct Foo {
   | ^^^^^^^^^^

---
---- [ui] src/test/ui/consts/issue-44415.rs stdout ----
diff of stderr:

16    |                 ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
17    = note: ...which requires computing layout of `Foo`...
18    = note: ...which requires computing layout of `[u8; _]`...
-    = note: ...which requires normalizing `[u8; _]`...
+    = note: ...which requires normalizing `unsafe { intrinsics::size_of::<Foo>() }`...
20    = note: ...which again requires evaluating type-level constant, completing the cycle
21 note: cycle used when checking that `Foo` is well-formed


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/issue-44415/issue-44415.stderr
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/issue-44415/issue-44415.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args consts/issue-44415.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/issue-44415.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/issue-44415" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/issue-44415/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0391]: cycle detected when evaluating type-level constant
   |
   |
LL |     bytes: [u8; unsafe { intrinsics::size_of::<Foo>() }],
   |
   |
note: ...which requires const-evaluating + checking `Foo::bytes::{constant#0}`...
   |
   |
LL |     bytes: [u8; unsafe { intrinsics::size_of::<Foo>() }],
   |                 ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
note: ...which requires const-evaluating + checking `Foo::bytes::{constant#0}`...
   |
   |
LL |     bytes: [u8; unsafe { intrinsics::size_of::<Foo>() }],
   |                 ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
   = note: ...which requires computing layout of `Foo`...
   = note: ...which requires computing layout of `[u8; _]`...
   = note: ...which requires normalizing `unsafe { intrinsics::size_of::<Foo>() }`...
   = note: ...which again requires evaluating type-level constant, completing the cycle
note: cycle used when checking that `Foo` is well-formed
   |
LL | struct Foo {
   | ^^^^^^^^^^

