plain
---- [ui] src/test/ui/type/issue-103271.rs stdout ----
diff of stderr:

7    = note: the following trait bounds were not satisfied:
8            `&[_]: ExactSizeIterator`
9            which is required by `&mut &[_]: ExactSizeIterator`
- help: the following trait is implemented but not in scope; perhaps add a `use` for it:
-    |
- LL | use object::read::read_ref::ReadRef;
-    |
-    |
15 help: the function `len` is implemented on `[_]`
16    |
17 LL |     let length = <[_]>::len;

The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/type/issue-103271/issue-103271.stderr
To update references, rerun the tests and pass the `--bless` flag
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args type/issue-103271.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/type/issue-103271.rs" "-Zthreads=1" "--target=wasm32-unknown-unknown" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/type/issue-103271" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/wasm32-unknown-unknown/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/type/issue-103271/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0599]: the function or associated item `len` exists for reference `&[_]`, but its trait bounds were not satisfied
   |
   |
LL |     let length = <&[_]>::len;
   |                          ^^^ function or associated item cannot be called on `&[_]` due to unsatisfied trait bounds
   = note: the following trait bounds were not satisfied:
   = note: the following trait bounds were not satisfied:
           `&[_]: ExactSizeIterator`
           which is required by `&mut &[_]: ExactSizeIterator`
help: the function `len` is implemented on `[_]`
   |
LL |     let length = <[_]>::len;

error: aborting due to previous error

For more information about this error, try `rustc --explain E0599`.
