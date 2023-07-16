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
Diff in /checkout/compiler/rustc_infer/src/infer/error_reporting/need_type_info.rs at line 888:
     /// Replace not yet inferred const params with their def name.
     fn replace_infers(&self, c: &'tcx Const<'tcx>, index: u32, name: Symbol) -> &'tcx Const<'tcx> {
         match c.val {
-            ty::ConstKind::Infer(..) => {
-                self.tcx().mk_const_param(index, name, c.ty)
-            }
+            ty::ConstKind::Infer(..) => self.tcx().mk_const_param(index, name, c.ty),
             _ => c,
     }
     }
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2018" "--unstable-features" "--skip-children" "--check" "/checkout/compiler/rustc_infer/src/infer/higher_ranked/mod.rs" "/checkout/compiler/rustc_infer/src/infer/combine.rs" "/checkout/compiler/rustc_infer/src/infer/nll_relate/mod.rs" "/checkout/compiler/rustc_infer/src/infer/lexical_region_resolve/mod.rs" "/checkout/compiler/rustc_infer/src/infer/lattice.rs" "/checkout/compiler/rustc_infer/src/infer/outlives/verify.rs" "/checkout/compiler/rustc_infer/src/infer/error_reporting/need_type_info.rs" "/checkout/compiler/rustc_infer/src/infer/projection.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
