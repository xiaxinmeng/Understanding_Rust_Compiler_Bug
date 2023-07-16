plain
configure: rust.debug-assertions := True
configure: rust.overflow-checks := True
configure: llvm.assertions      := True
configure: dist.missing-tools   := True
configure: build.configure-args := ['--enable-sccache', '--disable-manage-submodu ...
configure: writing `config.toml` in current directory
configure: 
configure: run `python /checkout/x.py --help`
configure: 
---
skip untracked path cpu-usage.csv during rustfmt invocations
skip untracked path src/doc/book/ during rustfmt invocations
skip untracked path src/doc/rust-by-example/ during rustfmt invocations
skip untracked path src/llvm-project/ during rustfmt invocations
Diff in /checkout/compiler/rustc_const_eval/src/interpret/operand.rs at line 403:
         // codegen does as good as we can (see `extract_field` in `rustc_codegen_ssa/src/mir/operand.rs`).
         let field_val: Immediate<_> = match (*base, base.layout.abi) {
             // the field contains no information
-            _ if field_layout.is_zst() => {
-                Scalar::ZST.into()
-            }
+            _ if field_layout.is_zst() => Scalar::ZST.into(),
             // the field covers the entire type
             _ if field_layout.size == base.layout.size => {
                 assert!(match (base.layout.abi, field_layout.abi) {
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2021" "--unstable-features" "--skip-children" "--check" "/checkout/compiler/rustc_const_eval/src/interpret/validity.rs" "/checkout/compiler/rustc_const_eval/src/const_eval/mod.rs" "/checkout/compiler/rustc_const_eval/src/interpret/memory.rs" "/checkout/compiler/rustc_const_eval/src/const_eval/eval_queries.rs" "/checkout/compiler/rustc_const_eval/src/interpret/visitor.rs" "/checkout/compiler/rustc_const_eval/src/interpret/operand.rs" "/checkout/compiler/rustc_const_eval/src/const_eval/valtrees.rs" "/checkout/compiler/rustc_const_eval/src/const_eval/machine.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
