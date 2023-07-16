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
 
 pub fn target() -> Target {
     let mut target = wasm32_unknown_emscripten::target();
-    target
-        .post_link_args
-        .entry(LinkerFlavor::Em)
-        .or_default()
-        .extend(vec!["-sWASM=0".into(), "--memory-init-file".into(), "0".into()]);
+    target.post_link_args.entry(LinkerFlavor::Em).or_default().extend(vec![
+        "-sWASM=0".into(),
+        "--memory-init-file".into(),
+        "0".into(),
     target
 }
 
 
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2021" "--unstable-features" "--skip-children" "--check" "/checkout/compiler/rustc_target/src/spec/thumbv4t_none_eabi.rs" "/checkout/compiler/rustc_target/src/spec/asmjs_unknown_emscripten.rs" "/checkout/compiler/rustc_target/src/spec/aarch64_unknown_hermit.rs" "/checkout/compiler/rustc_target/src/spec/i686_unknown_netbsd.rs" "/checkout/compiler/rustc_target/src/spec/mips_unknown_linux_gnu.rs" "/checkout/compiler/rustc_target/src/spec/mipsisa32r6el_unknown_linux_gnu.rs" "/checkout/compiler/rustc_target/src/spec/mips64el_unknown_linux_muslabi64.rs" "/checkout/compiler/rustc_target/src/spec/thumbv7em_none_eabi.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
