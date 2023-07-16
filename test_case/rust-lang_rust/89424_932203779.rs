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
Diff in /checkout/compiler/rustc_target/src/spec/riscv64gc_unknown_hermit.rs at line 1:
-use crate::spec::{CodeModel, RelocModel, TlsModel};
 use crate::spec::Target;
+use crate::spec::{CodeModel, RelocModel, TlsModel};
 pub fn target() -> Target {
 pub fn target() -> Target {
     let mut base = super::hermit_base::opts();
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2018" "--unstable-features" "--skip-children" "--check" "/checkout/compiler/rustc_target/src/abi/call/avr.rs" "/checkout/compiler/rustc_target/src/spec/sparc64_unknown_linux_gnu.rs" "/checkout/compiler/rustc_target/src/abi/call/nvptx.rs" "/checkout/compiler/rustc_target/src/spec/thumbv7a_uwp_windows_msvc.rs" "/checkout/compiler/rustc_target/src/abi/call/mips.rs" "/checkout/compiler/rustc_target/src/abi/call/m68k.rs" "/checkout/compiler/rustc_target/src/spec/riscv32imc_unknown_none_elf.rs" "/checkout/compiler/rustc_target/src/spec/riscv64gc_unknown_hermit.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
