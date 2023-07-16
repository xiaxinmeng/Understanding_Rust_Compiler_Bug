plain
   Compiling cranelift-frontend v0.69.0 (https://github.com/bytecodealliance/wasmtime/?branch=main#986b5768)
   Compiling cranelift-object v0.69.0 (https://github.com/bytecodealliance/wasmtime/?branch=main#986b5768)
   Compiling cranelift-jit v0.69.0 (https://github.com/bytecodealliance/wasmtime/?branch=main#986b5768)
   Compiling rustc_codegen_cranelift v0.1.0 (/checkout/compiler/rustc_codegen_cranelift)
warning: field is never read: `update_symbols`
   |
33 |     update_symbols: bool,
   |     ^^^^^^^^^^^^^^^^^^^^
   |
---
Suite("src/test/assembly") not skipped for "bootstrap::test::Assembly" -- not in ["src/tools/tidy"]
Check compiletest suite=assembly mode=assembly (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 29 tests
iiiiiiiiiiiiiiiiiiiiiiiiiiiii
Suite("src/test/incremental") not skipped for "bootstrap::test::Incremental" -- not in ["src/tools/tidy"]

 finished in 0.076 seconds
Check compiletest suite=incremental mode=incremental (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
---
 finished in 11.488 seconds
Check compiletest suite=debuginfo mode=debuginfo (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 116 tests
iiiiiiiiii.i.i..i..i..ii....i.i....ii..........iiii.........i.....i...i.......ii.i.ii.....iiii.....i 100/116
test result: ok. 78 passed; 0 failed; 38 ignored; 0 measured; 0 filtered out; finished in 2.59s

 finished in 2.666 seconds
Suite("src/test/ui-fulldeps") not skipped for "bootstrap::test::UiFullDeps" -- not in ["src/tools/tidy"]
---

---- builder::tests::defaults::build_cross_compile stdout ----
thread 'main' panicked at 'assertion failed: `(left == right)`

Diff < left / right > :
     Rustc {
         target: TargetSelection {
             triple: "A",
             file: None,
---

---- builder::tests::defaults::build_default stdout ----
thread 'main' panicked at 'assertion failed: `(left == right)`

Diff < left / right > :
     Rustc {
         target: TargetSelection {
             triple: "A",
             file: None,
---
 ]

', src/bootstrap/builder/tests.rs:59:9

---- builder::tests::defaults::build_stage_0 stdout ----
thread 'main' panicked at 'assertion failed: builder.cache.all::<compile::Rustc>().is_empty()', src/bootstrap/builder/tests.rs:84:9

failures:
    builder::tests::defaults::build_cross_compile
    builder::tests::defaults::build_default
