plain
configure: rust.channel         := nightly
configure: rust.debug-assertions := True
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
Diff in /checkout/compiler/rustc_codegen_llvm/src/lib.rs at line 211:
         match req {
             PrintRequest::RelocationModels => {
                 println!("Available relocation models:");
-                for name in
-                    &["static", "pic", "pie", "dynamic-no-pic", "ropi", "rwpi", "ropi-rwpi", "default"]
+                for name in &[
+                    "static",
+                    "pic",
+                    "pic",
+                    "pie",
+                    "dynamic-no-pic",
+                    "ropi",
+                    "rwpi",
+                    "ropi-rwpi",
+                    "default",
+                ] {
                     println!("    {}", name);
                 println!();
                 println!();
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2018" "--unstable-features" "--skip-children" "--check" "/checkout/compiler/rustc_codegen_ssa/src/glue.rs" "/checkout/compiler/rustc_codegen_llvm/src/lib.rs" "/checkout/compiler/rustc_codegen_llvm/src/declare.rs" "/checkout/compiler/rustc_codegen_ssa/src/mir/block.rs" "/checkout/compiler/rustc_codegen_llvm/src/intrinsic.rs" "/checkout/compiler/rustc_codegen_llvm/src/value.rs" "/checkout/compiler/rustc_codegen_ssa/src/mir/statement.rs" "/checkout/compiler/rustc_codegen_llvm/src/type_.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
