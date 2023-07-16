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
Diff in /checkout/src/librustdoc/clean/utils.rs at line 348:
         .collect()
 }
 
-fn print_const_with_custom_print_scalar(cx: &DocContext<'_>, ct: mir::ConstantKind<'tcx>) -> String {
+fn print_const_with_custom_print_scalar(
+    cx: &DocContext<'_>,
+    ct: mir::ConstantKind<'tcx>,
+) -> String {
     // Use a slightly different format for integer types which always shows the actual value.
     // For all other types, fallback to the original `pretty_print_const`.
     match ct.ty().kind() {
Diff in /checkout/src/librustdoc/clean/utils.rs at line 355:
         ty::Uint(ui) => {
             let int = ct.try_to_scalar_int().unwrap();
             let int = ty::ConstInt::new(int, false, ct.ty().is_ptr_sized_integral());
-            format!("{}{}", format_integer_with_underscore_sep(&format!("{:?}", int)), ui.name_str())
+                "{}{}",
+                "{}{}",
+                format_integer_with_underscore_sep(&format!("{:?}", int)),
+                ui.name_str()
         }
         }
         ty::Int(i) => {
             let ty = cx.tcx.lift(ct.ty()).unwrap();
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2018" "--unstable-features" "--skip-children" "--check" "/checkout/src/rustdoc-json-types/tests.rs" "/checkout/src/librustdoc/clean/cfg.rs" "/checkout/src/rustdoc-json-types/lib.rs" "/checkout/src/librustdoc/clean/utils.rs" "/checkout/src/librustdoc/clean/blanket_impl.rs" "/checkout/compiler/rustc_ast/src/tokenstream.rs" "/checkout/compiler/rustc_ast/src/attr/mod.rs" "/checkout/src/librustdoc/clean/cfg/tests.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
Build completed unsuccessfully in 0:00:18
