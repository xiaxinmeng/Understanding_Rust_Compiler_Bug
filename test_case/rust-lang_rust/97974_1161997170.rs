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
Diff in /checkout/compiler/rustc_const_eval/src/interpret/intrinsics.rs at line 10:
     interpret::{ConstValue, GlobalId, InterpResult, Scalar},
 };
+use rustc_middle::ty;
 use rustc_middle::ty::layout::LayoutOf as _;
 use rustc_middle::ty::layout::LayoutOf as _;
 use rustc_middle::ty::subst::SubstsRef;
-use rustc_middle::ty;
 use rustc_middle::ty::{Ty, TyCtxt};
 use rustc_span::symbol::{sym, Symbol};
 use rustc_target::abi::{Abi, Align, InitKind, Primitive, Size};
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2021" "--unstable-features" "--skip-children" "--check" "/checkout/compiler/rustc_const_eval/src/interpret/traits.rs" "/checkout/compiler/rustc_const_eval/src/interpret/terminator.rs" "/checkout/compiler/rustc_const_eval/src/interpret/intern.rs" "/checkout/compiler/rustc_span/src/symbol/tests.rs" "/checkout/compiler/rustc_const_eval/src/interpret/step.rs" "/checkout/compiler/rustc_const_eval/src/interpret/operand.rs" "/checkout/compiler/rustc_const_eval/src/interpret/mod.rs" "/checkout/compiler/rustc_const_eval/src/interpret/intrinsics.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
