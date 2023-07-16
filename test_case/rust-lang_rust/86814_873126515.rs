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
skip untracked path cpu-usage.csv during rustfmt invocations
skip untracked path src/doc/book/ during rustfmt invocations
skip untracked path src/doc/rust-by-example/ during rustfmt invocations
skip untracked path src/llvm-project/ during rustfmt invocations
Diff in /checkout/compiler/rustc_parse/src/parser/attr.rs at line 66:
                 just_parsed_doc_comment = true;
                 // Always make an outer attribute - this allows us to recover from a misplaced
                 // inner attribute.
-                Some(attr::mk_doc_comment(comment_kind, ast::AttrStyle::Outer, data, self.prev_token.span))
+                Some(attr::mk_doc_comment(
+                    comment_kind,
+                    ast::AttrStyle::Outer,
+                    data,
+                    self.prev_token.span,
             } else {
                 None
             };
             };
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2018" "--unstable-features" "--skip-children" "--check" "/checkout/compiler/rustc_codegen_ssa/src/glue.rs" "/checkout/compiler/rustc_parse/src/parser/diagnostics.rs" "/checkout/compiler/rustc_codegen_ssa/src/meth.rs" "/checkout/compiler/rustc_parse/src/parser/attr.rs" "/checkout/compiler/rustc_parse/src/parser/path.rs" "/checkout/compiler/rustc_parse/src/parser/pat.rs" "/checkout/compiler/rustc_parse/src/parser/nonterminal.rs" "/checkout/compiler/rustc_codegen_ssa/src/traits/mod.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
Build completed unsuccessfully in 0:00:15
