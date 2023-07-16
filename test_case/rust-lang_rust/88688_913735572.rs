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
skip untracked path src/doc/rust-by-example/ during rustfmt invocations
skip untracked path src/llvm-project/ during rustfmt invocations
Diff in /checkout/compiler/rustc_lint/src/builtin.rs at line 529:
 
 impl MissingDoc {
     pub fn new() -> MissingDoc {
-        MissingDoc { doc_hidden_stack: vec![false], private_traits: FxHashSet::default(), is_ignorable: false }
+        MissingDoc {
+            doc_hidden_stack: vec![false],
+            private_traits: FxHashSet::default(),
+            is_ignorable: false,
     }
 
 
     fn doc_hidden(&self) -> bool {
Diff in /checkout/compiler/rustc_lint/src/builtin.rs at line 629:
             }
             hir::ItemKind::Struct(hir::VariantData::Tuple(fields, _), _) => {
                 if fields.len() < 2 {
+                    // No need to check if there is missing documentation on the field.
+                    // No need to check if there is missing documentation on the field.
                     self.is_ignorable = true;
             }
             }
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2018" "--unstable-features" "--skip-children" "--check" "/checkout/compiler/rustc_interface/src/tests.rs" "/checkout/compiler/rustc_lint/src/noop_method_call.rs" "/checkout/compiler/rustc_interface/src/callbacks.rs" "/checkout/compiler/rustc_lint/src/builtin.rs" "/checkout/compiler/rustc_lint/src/non_fmt_panic.rs" "/checkout/compiler/rustc_lint/src/nonstandard_style/tests.rs" "/checkout/compiler/rustc_lint/src/internal.rs" "/checkout/compiler/rustc_lint/src/context.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
