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
Diff in /checkout/compiler/rustc_target/src/abi/call/sparc64.rs at line 1:
 // FIXME: This needs an audit for correctness and completeness.
 
-use crate::abi::call::{ArgAbi, ArgAttribute, ArgAttributes, ArgExtension, CastTarget, FnAbi, Reg, RegKind, Uniform};
+use crate::abi::call::{
+    ArgAbi, ArgAttribute, ArgAttributes, ArgExtension, CastTarget, FnAbi, Reg, RegKind, Uniform,
+};
 use crate::abi::{self, HasDataLayout, Size, TyAbiInterface};
 
 fn is_homogeneous_aggregate<'a, Ty, C>(cx: &C, arg: &mut ArgAbi<'a, Ty>) -> Option<Uniform>
Diff in /checkout/compiler/rustc_target/src/abi/call/sparc64.rs at line 88:
                     }
 
-
-
                     if scalar.value == abi::F32 {
                         arg_attribute = ArgAttribute::InReg;
                         prefix[prefix_index] = Some(Reg::f32());
Diff in /checkout/compiler/rustc_target/src/abi/call/sparc64.rs at line 112:
 
             arg.cast_to(CastTarget {
                 prefix,
-                rest: Uniform {
-                    unit: Reg::i64(),
-                },
-                },
+                rest: Uniform { unit: Reg::i64(), total: rest_size },
                 attrs: ArgAttributes {
                     regular: arg_attribute,
                     arg_ext: ArgExtension::None,
Diff in /checkout/compiler/rustc_target/src/abi/call/mod.rs at line 258:
         let mut size = self.rest.total;
         for i in 0..self.prefix.iter().count() {
             match self.prefix[i] {
-                Some(v) => size += Size{ raw: v.size.bytes() },
-                None => {},
+                Some(v) => size += Size { raw: v.size.bytes() },
             }
         }
         return size;
Diff in /checkout/compiler/rustc_target/src/abi/call/mips64.rs at line 1:
Diff in /checkout/compiler/rustc_target/src/abi/call/mips64.rs at line 1:
-use crate::abi::call::{ArgAbi, ArgAttribute, ArgAttributes, ArgExtension, CastTarget, FnAbi, PassMode, Reg, Uniform};
+use crate::abi::call::{
+    ArgAbi, ArgAttribute, ArgAttributes, ArgExtension, CastTarget, FnAbi, PassMode, Reg, Uniform,
+};
 use crate::abi::{self, HasDataLayout, Size, TyAbiInterface};
 
 fn extend_integer_width_mips<Ty>(arg: &mut ArgAbi<'_, Ty>, bits: u64) {
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2018" "--unstable-features" "--skip-children" "--check" "/checkout/compiler/rustc_target/src/spec/nvptx64_nvidia_cuda.rs" "/checkout/compiler/rustc_target/src/abi/call/nvptx64.rs" "/checkout/compiler/rustc_target/src/spec/x86_64_linux_android.rs" "/checkout/compiler/rustc_target/src/abi/call/mod.rs" "/checkout/compiler/rustc_target/src/spec/i686_unknown_uefi.rs" "/checkout/compiler/rustc_target/src/abi/call/msp430.rs" "/checkout/compiler/rustc_target/src/spec/x86_64_pc_solaris.rs" "/checkout/compiler/rustc_target/src/abi/call/avr.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
