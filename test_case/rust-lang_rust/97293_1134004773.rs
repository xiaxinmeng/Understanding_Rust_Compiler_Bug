plain

---- [ui] src/test/ui/type/ascription/issue-47666.rs stdout ----
diff of stderr:

- error: expected type, found `<[_]>::into_vec(box [0, 1])`
+ error: expected type, found `<[_]>::into_vec(#[rustc_box] ::alloc::boxed::Box::new([0, 1]))`
3    |
3    |
4 LL |     let _ = Option:Some(vec![0, 1]);

The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/type/ascription/issue-47666/issue-47666.stderr
To update references, rerun the tests and pass the `--bless` flag
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args type/ascription/issue-47666.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/type/ascription/issue-47666.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/type/ascription/issue-47666" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/type/ascription/issue-47666/auxiliary"
stdout: none
--- stderr -------------------------------
error: expected type, found `<[_]>::into_vec(#[rustc_box] ::alloc::boxed::Box::new([0, 1]))`
   |
   |
LL |     let _ = Option:Some(vec![0, 1]); //~ ERROR expected type, found
   |                   -     ^^^^^^^^^^
   |                   |     expected type
   |                   |     in this macro invocation
   |                   |     this macro call doesn't expand to a type
   |                   |     this macro call doesn't expand to a type
   |                   help: maybe write a path separator here: `::`
   |
   = note: `#![feature(type_ascription)]` lets you annotate an expression with a type: `<expr>: <type>`
   = note: this error originates in the macro `$crate::__rust_force_expr` (in Nightly builds, run with -Z macro-backtrace for more info)
error: aborting due to previous error
------------------------------------------


