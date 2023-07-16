plain
........................................................................................ 10648/13793
........................................................................................ 10736/13793
........................................................................................ 10824/13793
.....................................................................................iii 10912/13793
ii...i...F.iiFFF.F.FF.F................................................................. 11000/13793
....................................................................................i.ii 11176/13793
iiii..iiiiiiiii.i....................................................................... 11264/13793
........................................................................................ 11352/13793
........................................................................................ 11440/13793
---

---- [ui] src/test/ui/rfc-2627-raw-dylib/link-ordinal-and-name.rs stdout ----
diff of stderr:

1 error: cannot use `#[link_name]` with `#[link_ordinal]`
-   --> $DIR/link-ordinal-and-name.rs:6:5
3    |
4 LL |     #[link_ordinal(42)]
5    |     ^^^^^^^^^^^^^^^^^^^


6 
7 error: cannot use `#[link_name]` with `#[link_ordinal]`
-   --> $DIR/link-ordinal-and-name.rs:10:5
9    |
10 LL |     #[link_ordinal(5)]
11    |     ^^^^^^^^^^^^^^^^^^

---
To only update this specific test, also pass `--test-args rfc-2627-raw-dylib/link-ordinal-and-name.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/rfc-2627-raw-dylib/link-ordinal-and-name.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc-2627-raw-dylib/link-ordinal-and-name" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc-2627-raw-dylib/link-ordinal-and-name/auxiliary"
stdout: none
--- stderr -------------------------------
error: cannot use `#[link_name]` with `#[link_ordinal]`
   |
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
LL |     #[link_ordinal(42)]
   |     ^^^^^^^^^^^^^^^^^^^
   |     ^^^^^^^^^^^^^^^^^^^

error: cannot use `#[link_name]` with `#[link_ordinal]`
   |
LL |     #[link_ordinal(5)]
   |     ^^^^^^^^^^^^^^^^^^


error: aborting due to 2 previous errors
------------------------------------------


---- [ui] src/test/ui/rfc-2627-raw-dylib/link-ordinal-invalid-format.rs stdout ----
diff of stderr:

1 error: illegal ordinal format in `link_ordinal`
-   --> $DIR/link-ordinal-invalid-format.rs:5:5
3    |
3    |
4 LL |     #[link_ordinal("JustMonika")]


7    = note: an unsuffixed integer value, e.g., `1`, is expected
8 
9 error: illegal ordinal format in `link_ordinal`
-   --> $DIR/link-ordinal-invalid-format.rs:8:5
11    |
11    |
12 LL |     #[link_ordinal("JustMonika")]


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc-2627-raw-dylib/link-ordinal-invalid-format/link-ordinal-invalid-format.stderr
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc-2627-raw-dylib/link-ordinal-invalid-format/link-ordinal-invalid-format.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args rfc-2627-raw-dylib/link-ordinal-invalid-format.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/rfc-2627-raw-dylib/link-ordinal-invalid-format.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc-2627-raw-dylib/link-ordinal-invalid-format" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc-2627-raw-dylib/link-ordinal-invalid-format/auxiliary"
stdout: none
--- stderr -------------------------------
error: illegal ordinal format in `link_ordinal`
   |
   |
LL |     #[link_ordinal("JustMonika")]
   |
   |
   = note: an unsuffixed integer value, e.g., `1`, is expected

error: illegal ordinal format in `link_ordinal`
   |
   |
LL |     #[link_ordinal("JustMonika")]
   |
   |
   = note: an unsuffixed integer value, e.g., `1`, is expected
error: aborting due to 2 previous errors
------------------------------------------



---- [ui] src/test/ui/rfc-2627-raw-dylib/link-ordinal-missing-argument.rs stdout ----
diff of stderr:

1 error: incorrect number of arguments to `#[link_ordinal]`
-   --> $DIR/link-ordinal-missing-argument.rs:5:5
3    |
4 LL |     #[link_ordinal()]
5    |     ^^^^^^^^^^^^^^^^^


7    = note: the attribute requires exactly one argument
8 
9 error: incorrect number of arguments to `#[link_ordinal]`
-   --> $DIR/link-ordinal-missing-argument.rs:8:5
11    |
12 LL |     #[link_ordinal()]
13    |     ^^^^^^^^^^^^^^^^^

---
To only update this specific test, also pass `--test-args rfc-2627-raw-dylib/link-ordinal-missing-argument.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/rfc-2627-raw-dylib/link-ordinal-missing-argument.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc-2627-raw-dylib/link-ordinal-missing-argument" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc-2627-raw-dylib/link-ordinal-missing-argument/auxiliary"
stdout: none
--- stderr -------------------------------
error: incorrect number of arguments to `#[link_ordinal]`
   |
LL |     #[link_ordinal()]
   |     ^^^^^^^^^^^^^^^^^
   |
   |
   = note: the attribute requires exactly one argument

error: incorrect number of arguments to `#[link_ordinal]`
   |
LL |     #[link_ordinal()]
   |     ^^^^^^^^^^^^^^^^^
   |
---

---- [ui] src/test/ui/rfc-2627-raw-dylib/link-ordinal-too-large.rs stdout ----
diff of stderr:

1 error: ordinal value in `link_ordinal` is too large: `72436`
-   --> $DIR/link-ordinal-too-large.rs:5:5
3    |
4 LL |     #[link_ordinal(72436)]
5    |     ^^^^^^^^^^^^^^^^^^^^^^


7    = note: the value may not exceed `u16::MAX`
8 
9 error: ordinal value in `link_ordinal` is too large: `72436`
-   --> $DIR/link-ordinal-too-large.rs:8:5
11    |
12 LL |     #[link_ordinal(72436)]
13    |     ^^^^^^^^^^^^^^^^^^^^^^

---
To only update this specific test, also pass `--test-args rfc-2627-raw-dylib/link-ordinal-too-large.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/rfc-2627-raw-dylib/link-ordinal-too-large.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc-2627-raw-dylib/link-ordinal-too-large" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc-2627-raw-dylib/link-ordinal-too-large/auxiliary"
stdout: none
--- stderr -------------------------------
error: ordinal value in `link_ordinal` is too large: `72436`
   |
LL |     #[link_ordinal(72436)]
   |     ^^^^^^^^^^^^^^^^^^^^^^
   |
   |
   = note: the value may not exceed `u16::MAX`

error: ordinal value in `link_ordinal` is too large: `72436`
   |
LL |     #[link_ordinal(72436)]
   |     ^^^^^^^^^^^^^^^^^^^^^^
   |
---
To only update this specific test, also pass `--test-args rfc-2627-raw-dylib/link-ordinal-not-foreign-fn.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/rfc-2627-raw-dylib/link-ordinal-not-foreign-fn.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc-2627-raw-dylib/link-ordinal-not-foreign-fn" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc-2627-raw-dylib/link-ordinal-not-foreign-fn/auxiliary"
stdout: none
--- stderr -------------------------------
error: attribute should be applied to a foreign function or static
   |
LL | #[link_ordinal(123)]
   | ^^^^^^^^^^^^^^^^^^^^

---

---- [ui] src/test/ui/rfc-2627-raw-dylib/link-ordinal-too-many-arguments.rs stdout ----
diff of stderr:

1 error: incorrect number of arguments to `#[link_ordinal]`
-   --> $DIR/link-ordinal-too-many-arguments.rs:5:5
3    |
4 LL |     #[link_ordinal(3, 4)]
5    |     ^^^^^^^^^^^^^^^^^^^^^


7    = note: the attribute requires exactly one argument
8 
9 error: incorrect number of arguments to `#[link_ordinal]`
-   --> $DIR/link-ordinal-too-many-arguments.rs:8:5
11    |
12 LL |     #[link_ordinal(3, 4)]
13    |     ^^^^^^^^^^^^^^^^^^^^^

---
To only update this specific test, also pass `--test-args rfc-2627-raw-dylib/link-ordinal-too-many-arguments.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/rfc-2627-raw-dylib/link-ordinal-too-many-arguments.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc-2627-raw-dylib/link-ordinal-too-many-arguments" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc-2627-raw-dylib/link-ordinal-too-many-arguments/auxiliary"
stdout: none
--- stderr -------------------------------
error: incorrect number of arguments to `#[link_ordinal]`
   |
LL |     #[link_ordinal(3, 4)]
   |     ^^^^^^^^^^^^^^^^^^^^^
   |
   |
   = note: the attribute requires exactly one argument

error: incorrect number of arguments to `#[link_ordinal]`
   |
LL |     #[link_ordinal(3, 4)]
   |     ^^^^^^^^^^^^^^^^^^^^^
   |
---

---- [ui] src/test/ui/rfc-2627-raw-dylib/link-ordinal-unsupported-link-kind.rs stdout ----
diff of stderr:

1 error: `#[link_ordinal]` is only supported if link kind is `raw-dylib`
-   --> $DIR/link-ordinal-unsupported-link-kind.rs:5:5
3    |
4 LL |     #[link_ordinal(3)]
5    |     ^^^^^^^^^^^^^^^^^^


6 
7 error: `#[link_ordinal]` is only supported if link kind is `raw-dylib`
-   --> $DIR/link-ordinal-unsupported-link-kind.rs:12:5
9    |
10 LL |     #[link_ordinal(3)]
11    |     ^^^^^^^^^^^^^^^^^^

---
To only update this specific test, also pass `--test-args rfc-2627-raw-dylib/link-ordinal-unsupported-link-kind.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/rfc-2627-raw-dylib/link-ordinal-unsupported-link-kind.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc-2627-raw-dylib/link-ordinal-unsupported-link-kind" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc-2627-raw-dylib/link-ordinal-unsupported-link-kind/auxiliary"
stdout: none
--- stderr -------------------------------
error: `#[link_ordinal]` is only supported if link kind is `raw-dylib`
   |
LL |     #[link_ordinal(3)]
   |     ^^^^^^^^^^^^^^^^^^


error: `#[link_ordinal]` is only supported if link kind is `raw-dylib`
   |
LL |     #[link_ordinal(3)]
   |     ^^^^^^^^^^^^^^^^^^


error: aborting due to 2 previous errors
------------------------------------------


---- [ui] src/test/ui/rfc-2627-raw-dylib/raw-dylib-windows-only.rs stdout ----
diff of stderr:

1 error[E0455]: link kind `raw-dylib` is only supported on Windows targets
-   --> $DIR/raw-dylib-windows-only.rs:4:29
3    |
3    |
4 LL | #[link(name = "foo", kind = "raw-dylib")]


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc-2627-raw-dylib/raw-dylib-windows-only/raw-dylib-windows-only.stderr
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc-2627-raw-dylib/raw-dylib-windows-only/raw-dylib-windows-only.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args rfc-2627-raw-dylib/raw-dylib-windows-only.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/rfc-2627-raw-dylib/raw-dylib-windows-only.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc-2627-raw-dylib/raw-dylib-windows-only" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc-2627-raw-dylib/raw-dylib-windows-only/auxiliary" "--crate-type" "lib"
stdout: none
--- stderr -------------------------------
error[E0455]: link kind `raw-dylib` is only supported on Windows targets
   |
   |
LL | #[link(name = "foo", kind = "raw-dylib")]

error: aborting due to previous error

For more information about this error, try `rustc --explain E0455`.
