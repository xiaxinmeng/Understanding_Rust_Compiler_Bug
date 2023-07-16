plain
configure: rust.debug-assertions := True
configure: rust.overflow-checks := True
configure: llvm.assertions      := True
configure: dist.missing-tools   := True
configure: build.configure-args := ['--enable-sccache', '--disable-manage-submodu ...
configure: writing `config.toml` in current directory
configure: 
configure: run `python /checkout/x.py --help`
Attempting with retry: make prepare
---
Diff in /checkout/compiler/rustc_codegen_llvm/src/attributes.rs at line 224:
     attrs
 }
 
-fn create_alloc_family_attr<'ll>(llcx: &'ll llvm::Context) -> &'ll llvm::Attribute{
+fn create_alloc_family_attr<'ll>(llcx: &'ll llvm::Context) -> &'ll llvm::Attribute {
     llvm::CreateAttrStringValue(llcx, "alloc-family", "__rust_alloc")
 
 
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2021" "--unstable-features" "--skip-children" "--check" "/checkout/compiler/rustc_codegen_llvm/src/attributes.rs" "/checkout/compiler/rustc_graphviz/src/lib.rs" "/checkout/compiler/rustc_codegen_llvm/src/va_arg.rs" "/checkout/compiler/rustc_codegen_llvm/src/builder.rs" "/checkout/compiler/rustc_codegen_llvm/src/llvm_util.rs" "/checkout/compiler/rustc_codegen_llvm/src/coverageinfo/mod.rs" "/checkout/compiler/rustc_codegen_llvm/src/value.rs" "/checkout/compiler/rustc_graphviz/src/tests.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
thread 'main' panicked at 'called `Result::unwrap()` on an `Err` value: Any { .. }', format.rs:174:19
Build completed unsuccessfully in 0:00:16
