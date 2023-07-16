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
Diff in /checkout/compiler/rustc_target/src/abi/call/mod.rs at line 631:
             "nvptx64" => nvptx64::compute_abi_info(self),
             "hexagon" => hexagon::compute_abi_info(self),
             "riscv32" | "riscv64" => riscv::compute_abi_info(cx, self),
-            "wasm32" => if use_wasm_bindgen_compat_abi(cx.target_spec()) {
-                wasm32_bindgen_compat::compute_abi_info(self)
-            } else {
-                wasm32::compute_abi_info(cx, self)
-            },
+            "wasm32" => {
+                if use_wasm_bindgen_compat_abi(cx.target_spec()) {
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2018" "--unstable-features" "--skip-children" "--check" "/checkout/compiler/rustc_target/src/abi/call/mod.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
+                    wasm32_bindgen_compat::compute_abi_info(self)
+                } else {
+                    wasm32::compute_abi_info(cx, self)
+            }
+            }
             "asmjs" => wasm32::compute_abi_info(cx, self),
             a => return Err(format!("unrecognized arch \"{}\" in target specification", a)),
failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test --stage 2 src/tools/tidy
Build completed unsuccessfully in 0:00:16
