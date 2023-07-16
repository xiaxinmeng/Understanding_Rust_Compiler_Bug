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
Diff in /checkout/compiler/rustc_target/src/spec/wasm32_unknown_emscripten.rs at line 16:
     let mut post_link_args = LinkArgs::new();
     post_link_args.insert(
         LinkerFlavor::Em,
-        vec![
-            "-s".into(),
-            "ABORTING_MALLOC=0".into(),
-            "-Wl,--fatal-warnings".into(),
-        ],
+        vec!["-s".into(), "ABORTING_MALLOC=0".into(), "-Wl,--fatal-warnings".into()],
 
     let opts = TargetOptions {
     let opts = TargetOptions {
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2021" "--unstable-features" "--skip-children" "--check" "/checkout/compiler/rustc_target/src/spec/thumbv7em_none_eabihf.rs" "/checkout/compiler/rustc_target/src/spec/thumbv7neon_linux_androideabi.rs" "/checkout/compiler/rustc_target/src/spec/i386_apple_ios.rs" "/checkout/compiler/rustc_target/src/spec/armv7_unknown_linux_gnueabihf.rs" "/checkout/compiler/rustc_target/src/spec/wasm32_unknown_emscripten.rs" "/checkout/compiler/rustc_target/src/spec/i686_unknown_freebsd.rs" "/checkout/compiler/rustc_target/src/spec/x86_64_apple_tvos.rs" "/checkout/compiler/rustc_target/src/spec/aarch64_unknown_linux_gnu_ilp32.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
