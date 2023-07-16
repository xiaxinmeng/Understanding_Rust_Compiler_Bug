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

######################################################################## 100.0%
extracting /checkout/obj/build/cache/2022-06-29/rustfmt-nightly-x86_64-unknown-linux-gnu.tar.xz to /checkout/obj/build/x86_64-unknown-linux-gnu/stage0
skip untracked path cpu-usage.csv during rustfmt invocations
Diff in /checkout/compiler/rustc_target/src/spec/x86_64_apple_ios_sim.rs at line 14:
     Target {
         llvm_target: llvm_target.into(),
         pointer_width: 64,
-        data_layout: "e-m:o-p270:32:32-p271:32:32-p272:64:64-i64:64-f80:128-n8:16:32:64-S128".into(),
+        data_layout: "e-m:o-p270:32:32-p271:32:32-p272:64:64-i64:64-f80:128-n8:16:32:64-S128"
+            .into(),
         arch: "x86_64".into(),
         options: TargetOptions {
             max_atomic_width: Some(64),
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2021" "--unstable-features" "--skip-children" "--check" "/checkout/compiler/rustc_target/src/spec/thumbv7a_uwp_windows_msvc.rs" "/checkout/compiler/rustc_target/src/spec/arm_unknown_linux_musleabihf.rs" "/checkout/compiler/rustc_target/src/spec/wasm32_unknown_emscripten.rs" "/checkout/compiler/rustc_target/src/spec/i586_pc_windows_msvc.rs" "/checkout/compiler/rustc_target/src/spec/mips64el_unknown_linux_gnuabi64.rs" "/checkout/compiler/rustc_target/src/spec/armv7_apple_ios.rs" "/checkout/compiler/rustc_target/src/spec/bpfel_unknown_none.rs" "/checkout/compiler/rustc_target/src/spec/x86_64_apple_ios_sim.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
thread '<unnamed>' panicked at 'tx.send(entry.into_path()) failed with sending on a closed channel', format.rs:166:17
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
thread '<unnamed>' panicked at 'tx.send(entry.into_path()) failed with sending on a closed channel', format.rs:166:17
thread 'main' panicked at 'called `Result::unwrap()` on an `Err` value: Any { .. }', /cargo/registry/src/github.com-1ecc6299db9ec823/ignore-0.4.18/src/walk.rs:1302:31
