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
Diff in /checkout/compiler/rustc_target/src/spec/x86_64_apple_darwin.rs at line 5:
     base.cpu = "core2".to_string();
     base.max_atomic_width = Some(128); // core2 support cmpxchg16b
     base.eliminate_frame_pointer = false;
-    base.pre_link_args.insert(
-        LinkerFlavor::Gcc,
-        vec!["-arch".to_string(), "x86_64".to_string()],
-    );
+    base.pre_link_args.insert(LinkerFlavor::Gcc, vec!["-arch".to_string(), "x86_64".to_string()]);
     base.link_env_remove.extend(super::apple_base::macos_link_env_remove());
     // don't use probe-stack=inline-asm until rust#83139 and rust#84667 are resolved
     base.stack_probes = StackProbeType::Call;
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2018" "--unstable-features" "--skip-children" "--check" "/checkout/compiler/rustc_target/src/spec/vxworks_base.rs" "/checkout/compiler/rustc_target/src/spec/android_base.rs" "/checkout/compiler/rustc_target/src/spec/arm_linux_androideabi.rs" "/checkout/compiler/rustc_target/src/spec/wasm64_unknown_unknown.rs" "/checkout/compiler/rustc_target/src/spec/x86_64_unknown_openbsd.rs" "/checkout/compiler/rustc_target/src/spec/sparcv9_sun_solaris.rs" "/checkout/compiler/rustc_target/src/spec/x86_64_apple_darwin.rs" "/checkout/compiler/rustc_target/src/spec/aarch64_unknown_linux_gnu_ilp32.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
Build completed unsuccessfully in 0:00:15
