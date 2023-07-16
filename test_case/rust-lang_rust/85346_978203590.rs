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
skip untracked path src/doc/rust-by-example/ during rustfmt invocations
skip untracked path src/llvm-project/ during rustfmt invocations
Diff in /checkout/compiler/rustc_parse/src/parser/diagnostics.rs at line 1926:
         };
         let mut err =
             self.struct_span_err(param.span(), "unexpected `const` parameter declaration");
-        err.span_label(
-            param.span(),
-            "expected a `const` expression, not a parameter declaration",
-        );
+        err.span_label(param.span(), "expected a `const` expression, not a parameter declaration");
         if let (Some(generics), Ok(snippet)) =
             (ty_generics, self.sess.source_map().span_to_snippet(param.span()))
         {
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2018" "--unstable-features" "--skip-children" "--check" "/checkout/compiler/rustc_parse/src/parser/attr_wrapper.rs" "/checkout/compiler/rustc_parse/src/parser/expr.rs" "/checkout/compiler/rustc_parse/src/parser/stmt.rs" "/checkout/compiler/rustc_parse/src/parser/diagnostics.rs" "/checkout/compiler/rustc_trait_selection/src/traits/util.rs" "/checkout/compiler/rustc_trait_selection/src/traits/error_reporting/mod.rs" "/checkout/compiler/rustc_trait_selection/src/traits/error_reporting/on_unimplemented.rs" "/checkout/compiler/rustc_parse/src/parser/path.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
