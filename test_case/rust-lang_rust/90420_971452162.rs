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
skip untracked path src/llvm-project/ during rustfmt invocations
Diff in /checkout/compiler/rustc_passes/src/check_attr.rs at line 962:
                         }
 
                         sym::primitive => {
-                            if !self.tcx.features().rustdoc_internals
-                            {
+                            if !self.tcx.features().rustdoc_internals {
                                 self.tcx.struct_span_lint_hir(
                                     hir_id,
                                     hir_id,
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2018" "--unstable-features" "--skip-children" "--check" "/checkout/compiler/rustc_passes/src/lang_items.rs" "/checkout/compiler/rustc_passes/src/naked_functions.rs" "/checkout/compiler/rustc_borrowck/src/facts.rs" "/checkout/compiler/rustc_borrowck/src/universal_regions.rs" "/checkout/compiler/rustc_passes/src/intrinsicck.rs" "/checkout/compiler/rustc_borrowck/src/location.rs" "/checkout/compiler/rustc_passes/src/hir_id_validator.rs" "/checkout/compiler/rustc_passes/src/check_attr.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
