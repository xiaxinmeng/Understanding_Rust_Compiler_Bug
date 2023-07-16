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
Diff in /checkout/compiler/rustc_ast/src/util/comments/tests.rs at line 45:
 #[test]
 fn test_doc_blocks() {
     create_default_session_globals_then(|| {
-        let stripped = beautify_doc_string(
-            Symbol::intern(" # Returns\n     *\n     "),
-        );
+        let stripped =
+        let stripped =
+            beautify_doc_string(Symbol::intern(" # Returns\n     *\n     "), CommentKind::Block);
         assert_eq!(stripped.as_str(), " # Returns\n\n");
 
         let stripped = beautify_doc_string(
Diff in /checkout/compiler/rustc_ast/src/util/comments/tests.rs at line 57:
         );
         assert_eq!(stripped.as_str(), " # Returns\n\n");
 
-        let stripped = beautify_doc_string(
-            Symbol::intern("\n *     a\n "),
-        );
-        );
+        let stripped = beautify_doc_string(Symbol::intern("\n *     a\n "), CommentKind::Block);
         assert_eq!(stripped.as_str(), "     a\n");
 }
 }
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2018" "--unstable-features" "--skip-children" "--check" "/checkout/compiler/rustc_ast/src/util/comments/tests.rs" "/checkout/compiler/rustc_ast/src/util/unicode.rs" "/checkout/compiler/rustc_ast/src/attr/mod.rs" "/checkout/compiler/rustc_ast/src/ast/tests.rs" "/checkout/compiler/rustc_ast/src/visit.rs" "/checkout/compiler/rustc_ast/src/entry.rs" "/checkout/compiler/rustc_target/src/spec/riscv32gc_unknown_linux_gnu.rs" "/checkout/compiler/rustc_ast/src/util/comments.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
