plain
Prepare all required actions
Getting action download info
Download action repository 'actions/checkout@v3' (SHA:ac593985615ec2ede58e132d2e21d2b1cbd6127c)
Download action repository 'rust-lang/simpleinfra@master' (SHA:ece894d15649f0f9d7884f915dc821f00bd0418b)
Complete job name: PR (x86_64-gnu-llvm-13, false, ubuntu-20.04-xl)
git config --global core.autocrlf false
shell: /usr/bin/bash --noprofile --norc -e -o pipefail {0}
env:
  CI_JOB_NAME: x86_64-gnu-llvm-13
---
   Compiling tempfile v3.3.0
   Compiling serde_json v1.0.85
   Compiling lint-docs v0.1.0 (/checkout/src/tools/lint-docs)
    Finished release [optimized] target(s) in 5.05s
warning: the code example in lint `must_not_suspend` in /checkout/compiler/rustc_lint_defs/src/builtin.rs failed to generate the expected output: did not find lint `must_not_suspend` in output of example, got:
warning: unused variable: `guard`
  --> lint_example.rs:11:9
   |
   |
11 |     let guard = SyncThing {};
   |         ^^^^^ help: if this is intentional, prefix it with an underscore: `_guard`
   = note: `#[warn(unused_variables)]` on by default


warning: struct `SyncThing` is never constructed
---
  |
  = note: `#[warn(dead_code)]` on by default


warning: function `yield_now` is never used
 --> lint_example.rs:8:10
  |
8 | async fn yield_now() {}



warning: function `uhoh` is never used
  --> lint_example.rs:10:14
   |
10 | pub async fn uhoh() {


warning: 4 warnings emitted

---
Testing error-index stage2
doc tests for: /checkout/obj/build/x86_64-unknown-linux-gnu/test/error-index.md


command did not execute successfully: "/checkout/obj/build/bootstrap/debug/rustdoc" "-Wrustdoc::invalid_codeblock_attributes" "-Dwarnings" "-Znormalize-docs" "-Z" "unstable-options" "--test" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/error-index.md" "--test-args" ""

stdout ----

running 1073 tests
---
---- /checkout/obj/build/x86_64-unknown-linux-gnu/test/error-index.md - Rust_Compiler_Error_Index::E0698 (line 14854) stdout ----
error[E0282]: type annotations needed
 --> /checkout/obj/build/x86_64-unknown-linux-gnu/test/error-index.md:14858:5
  |
6 |     bar().await; // error: cannot infer type for `T`
  |     ^^^ cannot infer type of the type parameter `T` declared on the function `bar`
help: consider specifying the generic argument
  |
  |
6 |     bar::<T>().await; // error: cannot infer type for `T`

error: aborting due to previous error

For more information about this error, try `rustc --explain E0282`.
For more information about this error, try `rustc --explain E0282`.
Some expected error codes were not found: ["E0698"]
failures:
    /checkout/obj/build/x86_64-unknown-linux-gnu/test/error-index.md - Rust_Compiler_Error_Index::E0698 (line 14854)

test result: FAILED. 1023 passed; 1 failed; 49 ignored; 0 measured; 0 filtered out; finished in 7.98s
