plain
skip untracked path cpu-usage.csv during rustfmt invocations
skip untracked path src/doc/book/ during rustfmt invocations
skip untracked path src/doc/rust-by-example/ during rustfmt invocations
skip untracked path src/llvm-project/ during rustfmt invocations
Diff in /checkout/compiler/rustc_codegen_llvm/src/intrinsic.rs at line 350:
                 // For zero-sized types, the location pointed to by the result may be
                 // uninitialized. Do not "use" the result in this case; instead just clobber
                 // the memory.
-                let (constraint, inputs) : (&str, &[_]) = if result.layout.is_zst() {
+                let (constraint, inputs): (&str, &[_]) = if result.layout.is_zst() {
                     ("~{memory}", &[])
                 } else {
                     ("r,~{memory}", &result_val_span)
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2021" "--unstable-features" "--skip-children" "--check" "/checkout/compiler/rustc_codegen_llvm/src/debuginfo/utils.rs" "/checkout/compiler/rustc_codegen_llvm/src/debuginfo/metadata.rs" "/checkout/compiler/rustc_codegen_llvm/src/debuginfo/gdb.rs" "/checkout/compiler/rustc_codegen_llvm/src/debuginfo/namespace.rs" "/checkout/compiler/rustc_codegen_llvm/src/debuginfo/metadata/type_map.rs" "/checkout/compiler/rustc_codegen_llvm/src/debuginfo/metadata/enums/cpp_like.rs" "/checkout/compiler/rustc_codegen_llvm/src/debuginfo/metadata/enums/native.rs" "/checkout/compiler/rustc_codegen_llvm/src/intrinsic.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
