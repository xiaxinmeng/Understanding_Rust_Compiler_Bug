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
Diff in /checkout/compiler/rustc_target/src/abi/call/aarch64.rs at line 95:
     arg.make_indirect();
 }
 
-pub fn compute_abi_info<'a, Ty, C>(
-    cx: &C,
-    fn_abi: &mut FnAbi<'a, Ty>,
-    param_policy: ParamExtension,
-) where
+pub fn compute_abi_info<'a, Ty, C>(cx: &C, fn_abi: &mut FnAbi<'a, Ty>, param_policy: ParamExtension)
+where
     Ty: TyAbiInterface<'a, C> + Copy,
     C: HasDataLayout,
 {
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2021" "--unstable-features" "--skip-children" "--check" "/checkout/compiler/rustc_target/src/abi/call/x86_64.rs" "/checkout/compiler/rustc_target/src/abi/call/avr.rs" "/checkout/compiler/rustc_target/src/abi/call/mips64.rs" "/checkout/compiler/rustc_target/src/abi/call/aarch64.rs" "/checkout/compiler/rustc_target/src/abi/call/amdgpu.rs" "/checkout/compiler/rustc_target/src/asm/mod.rs" "/checkout/compiler/rustc_target/src/asm/msp430.rs" "/checkout/compiler/rustc_target/src/abi/call/mips.rs"` failed.
Build completed unsuccessfully in 0:00:12
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
