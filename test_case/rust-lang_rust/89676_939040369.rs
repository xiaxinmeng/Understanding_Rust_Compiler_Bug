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
Diff in /checkout/src/librustdoc/html/format.rs at line 1059:
     ) -> impl fmt::Display + 'a + Captures<'tcx> {
         display_fn(move |f| {
             if !self.generic_params.is_empty() {
-                write!(f, "for&lt;{}&gt; ", comma_sep(self.generic_params.iter().map(|g| g.print(cx))))
+                    f,
+                    f,
+                    "for&lt;{}&gt; ",
+                    comma_sep(self.generic_params.iter().map(|g| g.print(cx)))
             } else {
                 Ok(())
             }
             }
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2018" "--unstable-features" "--skip-children" "--check" "/checkout/src/librustdoc/html/format.rs" "/checkout/src/librustdoc/html/mod.rs" "/checkout/src/librustdoc/theme.rs" "/checkout/src/librustdoc/lib.rs" "/checkout/src/librustdoc/visit_lib.rs" "/checkout/src/librustdoc/html/markdown.rs" "/checkout/src/librustdoc/passes/collect_intra_doc_links.rs" "/checkout/src/librustdoc/html/layout.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
