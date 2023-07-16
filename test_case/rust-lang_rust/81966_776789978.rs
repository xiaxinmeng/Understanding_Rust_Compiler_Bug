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
Diff in /checkout/compiler/rustc_target/src/spec/apple_sdk_base.rs at line 30:
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2018" "--unstable-features" "--skip-children" "--check" "/checkout/compiler/rustc_target/src/spec/apple_sdk_base.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
 
 fn link_env_remove(arch: Arch) -> Vec<String> {
     match arch {
-        Armv7 | Armv7s | Arm64 | I386 | X86_64 | Arm64_sim => vec!["MACOSX_DEPLOYMENT_TARGET".to_string()],
+        Armv7 | Armv7s | Arm64 | I386 | X86_64 | Arm64_sim => {
+            vec!["MACOSX_DEPLOYMENT_TARGET".to_string()]
+        }
         X86_64_macabi | Arm64_macabi => vec!["IPHONEOS_DEPLOYMENT_TARGET".to_string()],
 }
failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test --stage 2 src/tools/tidy
Build completed unsuccessfully in 0:00:22
