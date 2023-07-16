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
Diff in /checkout/compiler/rustc_target/src/spec/mips64_openwrt_linux_musl.rs at line 1:
 /// A target tuple for OpenWrt MIPS64 targets
-
 use crate::abi::Endian;
 use crate::abi::Endian;
 use crate::spec::{Target, TargetOptions};
 
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2018" "--unstable-features" "--skip-children" "--check" "/checkout/compiler/rustc_target/src/spec/vxworks_base.rs" "/checkout/compiler/rustc_target/src/spec/aarch64_unknown_linux_gnu.rs" "/checkout/compiler/rustc_target/src/spec/mips64_openwrt_linux_musl.rs" "/checkout/compiler/rustc_target/src/asm/aarch64.rs" "/checkout/compiler/rustc_target/src/spec/thumbv8m_base_none_eabi.rs" "/checkout/compiler/rustc_target/src/asm/x86.rs" "/checkout/compiler/rustc_target/src/spec/x86_64_pc_windows_gnu.rs" "/checkout/compiler/rustc_target/src/spec/x86_64_apple_ios_macabi.rs"` failed.
Build completed unsuccessfully in 0:00:11
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
