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
Diff in /checkout/compiler/rustc_target/src/spec/aarch64_apple_ios.rs at line 2:
 use crate::spec::{FramePointer, Target, TargetOptions};
 pub fn target() -> Target {
-
-
     // Clang automatically chooses a more specific target based on
     // IPHONEOS_DEPLOYMENT_TARGET.
     // This is required for the target to pick the right
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2018" "--unstable-features" "--skip-children" "--check" "/checkout/compiler/rustc_target/src/spec/aarch64_apple_ios.rs" "/checkout/compiler/rustc_target/src/spec/armv5te_unknown_linux_gnueabi.rs" "/checkout/compiler/rustc_target/src/spec/sparc64_unknown_openbsd.rs" "/checkout/compiler/rustc_target/src/spec/mipsisa32r6el_unknown_linux_gnu.rs" "/checkout/compiler/rustc_target/src/spec/i686_unknown_netbsd.rs" "/checkout/compiler/rustc_target/src/spec/mipsel_unknown_linux_uclibc.rs" "/checkout/compiler/rustc_target/src/spec/windows_msvc_base.rs" "/checkout/compiler/rustc_target/src/spec/armv4t_unknown_linux_gnueabi.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
