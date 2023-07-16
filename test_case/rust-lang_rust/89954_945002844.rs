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
Diff in /checkout/src/librustdoc/clean/mod.rs at line 805:
             match literal.kind {
                 ast::LitKind::Int(a, _) => {
                     let gen = func.generics.params.remove(0);
-                    if let GenericParamDef { name, kind: GenericParamDefKind::Const { ty, .. } } = gen {
+                    if let GenericParamDef { name, kind: GenericParamDefKind::Const { ty, .. } } =
+                    {
+                    {
                         func.decl.inputs.values.insert(a as _, Argument { name, type_: ty });
                     } else {
                         panic!("unexpected non const in position {}", pos);
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2018" "--unstable-features" "--skip-children" "--check" "/checkout/src/librustdoc/clean/inline.rs" "/checkout/src/librustdoc/passes/check_doc_test_visibility.rs" "/checkout/src/librustdoc/clean/mod.rs" "/checkout/src/librustdoc/passes/strip_priv_imports.rs" "/checkout/src/librustdoc/passes/mod.rs" "/checkout/src/librustdoc/clean/cfg/tests.rs" "/checkout/src/tools/rust-demangler/src/main.rs" "/checkout/src/librustdoc/passes/strip_hidden.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
