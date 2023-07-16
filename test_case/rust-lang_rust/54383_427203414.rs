
---- [codegen] codegen/lto-removes-invokes.rs stdout ----

error: compilation failed!
status: signal: 6
command: "/users/mahewson/rust1/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/users/mahewson/rust1/src/test/codegen/lto-removes-invokes.rs" "--target=x86_64-unknown-linux-gnu" "--out-dir" "/users/mahewson/rust1/build/x86_64-unknown-linux-gnu/test/codegen/lto-removes-invokes" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/users/mahewson/rust1/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-C" "lto" "-C" "panic=abort" "-O" "-L" "/users/mahewson/rust1/build/x86_64-unknown-linux-gnu/test/codegen/lto-removes-invokes/auxiliary" "--emit=llvm-ir"
stdout:
------------------------------------------

------------------------------------------
stderr:
------------------------------------------
warning: function is marked #[no_mangle], but not exported
  --> /users/mahewson/rust1/src/test/codegen/lto-removes-invokes.rs:20:1
   |
20 |   fn foo() {
   |   ^
   |   |
   |  _help: try making it public: `pub`
   | |
21 | |     let _a = Box::new(3);
22 | |     bar();
23 | | // CHECK-LABEL: foo
24 | | // CHECK: call {{.*}} void @bar
25 | | }
   | |_^
   |
   = note: #[warn(private_no_mangle_fns)] on by default

warning: function is marked #[no_mangle], but not exported
  --> /users/mahewson/rust1/src/test/codegen/lto-removes-invokes.rs:29:1
   |
29 |   fn bar() {
   |   ^
   |   |
   |  _help: try making it public: `pub`
   | |
30 | |     println!("hello!");
31 | | }
   | |_^

rustc: /users/mahewson/rust1/src/llvm/lib/IR/Value.cpp:256: void llvm::Value::setNameImpl(const llvm::Twine&): Assertion `!getType()->isVoidTy() && "Cannot assign a name to void values!"' failed.

------------------------------------------
