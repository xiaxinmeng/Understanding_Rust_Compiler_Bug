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
Diff in /checkout/src/tools/compiletest/src/util.rs at line 121:
 pub const MEMTAG_SUPPORTED_TARGETS: &[&str] =
     &["aarch64-linux-android", "aarch64-unknown-linux-gnu"];
 
-pub const SHADOWCALLSTACK_SUPPORTED_TARGETS: &[&str] =
-    &["aarch64-linux-android"];
+pub const SHADOWCALLSTACK_SUPPORTED_TARGETS: &[&str] = &["aarch64-linux-android"];
 
 const BIG_ENDIAN: &[&str] = &[
     "aarch64_be",
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2021" "--unstable-features" "--skip-children" "--check" "/checkout/src/tools/compiletest/src/read2.rs" "/checkout/src/tools/build-manifest/src/main.rs" "/checkout/src/tools/compiletest/src/runtest/debugger.rs" "/checkout/src/tools/compiletest/src/runtest/tests.rs" "/checkout/src/tools/compiletest/src/header/tests.rs" "/checkout/src/tools/build-manifest/src/versions.rs" "/checkout/src/tools/build-manifest/src/checksum.rs" "/checkout/src/tools/compiletest/src/util.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
