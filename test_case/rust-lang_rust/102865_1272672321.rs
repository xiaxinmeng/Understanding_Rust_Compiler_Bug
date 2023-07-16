plain
configure: rust.debug-assertions := True
configure: rust.overflow-checks := True
configure: llvm.assertions      := True
configure: dist.missing-tools   := True
configure: build.configure-args := ['--enable-sccache', '--disable-manage-submodu ...
configure: writing `config.toml` in current directory
configure: 
configure: run `python /checkout/x.py --help`
Attempting with retry: make prepare
---
skip untracked path src/doc/rust-by-example/ during rustfmt invocations
skip untracked path src/llvm-project/ during rustfmt invocations
Diff in /checkout/compiler/rustc_codegen_ssa/src/back/link.rs at line 1033:
 
     if sess.target.is_like_osx {
         match (strip, crate_type) {
-            (Strip::Debuginfo, _) => strip_symbols_with_external_utility(sess, "strip", &out_filename, Some("-S")),
+            (Strip::Debuginfo, _) => {
+                strip_symbols_with_external_utility(sess, "strip", &out_filename, Some("-S"))
+            }
             // Per the manpage, `-x` is the maximum safe strip level for dynamic libraries. (#93988)
             (Strip::Symbols, CrateType::Dylib | CrateType::Cdylib | CrateType::ProcMacro) => {
                 strip_symbols_with_external_utility(sess, "strip", &out_filename, Some("-x"))
Diff in /checkout/compiler/rustc_codegen_ssa/src/back/link.rs at line 1040:
             }
-            (Strip::Symbols, _) => strip_symbols_with_external_utility(sess, "strip", &out_filename, None),
+            (Strip::Symbols, _) => {
+                strip_symbols_with_external_utility(sess, "strip", &out_filename, None)
+            }
             (Strip::None, _) => {}
     }
Diff in /checkout/compiler/rustc_codegen_ssa/src/back/link.rs at line 1050:
         let stripcmd = "/usr/bin/strip";
         match strip {
         match strip {
             // Always preserve the symbol table (-x).
-            Strip::Debuginfo => strip_symbols_with_external_utility(sess, stripcmd, &out_filename, Some("-x")),
+            Strip::Debuginfo => {
+                strip_symbols_with_external_utility(sess, stripcmd, &out_filename, Some("-x"))
+            }
             // Strip::Symbols is handled via the --strip-all linker option.
-            Strip::Symbols => {},
+            Strip::Symbols => {}
             Strip::None => {}
     }
Diff in /checkout/compiler/rustc_codegen_ssa/src/back/link.rs at line 1068:
     }
 }
 }
 
-fn strip_symbols_with_external_utility<'a>(sess: &'a Session, util: &str, out_filename: &Path, option: Option<&str>) {
+fn strip_symbols_with_external_utility<'a>(
+    sess: &'a Session,
+    util: &str,
+    out_filename: &Path,
+    option: Option<&str>,
     let mut cmd = Command::new(util);
     if let Some(option) = option {
         cmd.arg(option);
         cmd.arg(option);
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2021" "--unstable-features" "--skip-children" "--check" "/checkout/compiler/rustc_codegen_ssa/src/back/symbol_export.rs" "/checkout/compiler/rustc_codegen_ssa/src/mono_item.rs" "/checkout/compiler/rustc_codegen_ssa/src/back/metadata.rs" "/checkout/compiler/rustc_transmute/src/maybe_transmutable/mod.rs" "/checkout/compiler/rustc_codegen_ssa/src/back/mod.rs" "/checkout/compiler/rustc_transmute/src/maybe_transmutable/tests.rs" "/checkout/compiler/rustc_codegen_ssa/src/back/link.rs" "/checkout/compiler/rustc_codegen_ssa/src/debuginfo/type_names.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
