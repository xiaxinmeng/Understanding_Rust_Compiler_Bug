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
Diff in /checkout/src/librustdoc/clean/types.rs at line 882:
     /// Returns `true` if the attribute list contains a specific `word`
     fn has_word(self, word: Symbol) -> bool
     where
-        Self: std::marker::Sized
+        Self: std::marker::Sized,
     {
         <Self as NestedAttributesExt>::get_word_attr(self, word).is_some()
Diff in /checkout/src/librustdoc/clean/types.rs at line 894:
 
 
 impl<I> NestedAttributesExt for I
 where
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2018" "--unstable-features" "--skip-children" "--check" "/checkout/src/tools/lint-docs/src/groups.rs" "/checkout/src/librustdoc/clean/utils.rs" "/checkout/src/tools/expand-yaml-anchors/src/main.rs" "/checkout/src/librustdoc/clean/utils/tests.rs" "/checkout/src/librustdoc/clean/mod.rs" "/checkout/src/librustdoc/clean/simplify.rs" "/checkout/src/librustdoc/clean/types.rs" "/checkout/src/tools/error_index_generator/main.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
-    I: IntoIterator<Item = ast::NestedMetaItem>
+    I: IntoIterator<Item = ast::NestedMetaItem>,
 {
     fn get_word_attr(self, word: Symbol) -> Option<ast::NestedMetaItem> {
         self.into_iter().find(|attr| attr.is_word() && attr.has_name(word))
