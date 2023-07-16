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
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2018" "--unstable-features" "--skip-children" "--check" "/checkout/src/tools/build-manifest/src/versions.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
Diff in /checkout/src/tools/build-manifest/src/versions.rs at line 37:
             "rustfmt" | "rustfmt-preview" => PkgType::Rustfmt,
             "llvm-tools" | "llvm-tools-preview" => PkgType::LlvmTools,
             "miri" | "miri-preview" => PkgType::Miri,
-            "rustc-codegen-cranelift"
-            | "rustc-codegen-cranelift-preview" => PkgType::RustcCodegenCranelift,
+            "rustc-codegen-cranelift" | "rustc-codegen-cranelift-preview" => {
+                PkgType::RustcCodegenCranelift
+            }
             other => PkgType::Other(other.into()),
     }
failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test --stage 2 src/tools/tidy
Build completed unsuccessfully in 0:00:24
