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
Diff in /checkout/compiler/rustc_target/src/spec/sparc64_unknown_netbsd.rs at line 12:
         pointer_width: 64,
         data_layout: "E-m:e-i64:64-n32:64-S128".to_string(),
         arch: "sparc64".to_string(),
-        options: TargetOptions {
-            endian: Endian::Big,
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2018" "--unstable-features" "--skip-children" "--check" "/checkout/compiler/rustc_target/src/spec/sparc64_unknown_netbsd.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
-            mcount: "__mcount".to_string(),
-            ..base
-        },
+        options: TargetOptions { endian: Endian::Big, mcount: "__mcount".to_string(), ..base },
 }
 
failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test --stage 2 src/tools/tidy
Build completed unsuccessfully in 0:00:18
