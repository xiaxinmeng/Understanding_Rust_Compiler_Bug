
running 1 test
F
failures:

---- [ui] ui/proc-macro/musl-proc-macro.rs stdout ----

error: test compilation failed although it shouldn't!
status: exit code: 1
command: "/home/han/rust/build/x86_64-unknown-linux-musl/stage1/bin/rustc" "/home/han/rust/src/test/ui/proc-macro/musl-proc-macro.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-musl" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "--out-dir" "/home/han/rust/build/x86_64-unknown-linux-musl/test/ui/proc-macro/musl-proc-macro" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/home/han/rust/build/x86_64-unknown-linux-musl/native/rust-test-helpers" "-Ctarget-feature=-crt-static" "-A" "unused" "-Ctarget-feature=" "-L" "/home/han/rust/build/x86_64-unknown-linux-musl/test/ui/proc-macro/musl-proc-macro/auxiliary"
stdout:
------------------------------------------

------------------------------------------
stderr:
------------------------------------------
warning: dropping unsupported crate type `proc-macro` for target `x86_64-unknown-linux-musl`

error: the `#[proc_macro_derive]` attribute is only usable with crates of the `proc-macro` crate type
  --> /home/han/rust/src/test/ui/proc-macro/musl-proc-macro.rs:13:1
   |
LL | #[proc_macro_derive(Foo)]
   | ^^^^^^^^^^^^^^^^^^^^^^^^^

error: aborting due to previous error
