plain

---- [ui] tests/ui/parser/issues/issue-111416.rs stdout ----
diff of stderr:

1 error: invalid `struct` delimiters or `fn` call arguments
-   --> $DIR/issue-111416.rs:3:14
3    |
3    |
4 LL |     let my = monad_bind(mx, T: Try);


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/issues/issue-111416/issue-111416.stderr
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/issues/issue-111416/issue-111416.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args parser/issues/issue-111416.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/tests/ui/parser/issues/issue-111416.rs" "-Zthreads=1" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--remap-path-prefix=/checkout/tests/ui=fake-test-src-base" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/issues/issue-111416" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/issues/issue-111416/auxiliary"
stdout: none
--- stderr -------------------------------
error: invalid `struct` delimiters or `fn` call arguments
  --> fake-test-src-base/parser/issues/issue-111416.rs:2:14
   |
LL |     let my = monad_bind(mx, T: Try); //~ ERROR invalid `struct` delimiters or `fn` call arguments
   |
   |
help: if `monad_bind` is a struct, use braces as delimiters
   |
LL |     let my = monad_bind { mx, T: Try }; //~ ERROR invalid `struct` delimiters or `fn` call arguments
   |                         ~            ~
help: if `monad_bind` is a function, use the arguments directly
   |
LL -     let my = monad_bind(mx, T: Try); //~ ERROR invalid `struct` delimiters or `fn` call arguments
LL +     let my = monad_bind(mx, Try); //~ ERROR invalid `struct` delimiters or `fn` call arguments

error: aborting due to previous error
------------------------------------------

