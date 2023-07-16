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
Diff in /checkout/src/librustdoc/clean/mod.rs at line 1700:
         let kind = match self.ctor_kind {
             CtorKind::Const => Variant::CLike,
             CtorKind::Fn => Variant::Tuple(
-                self.fields
-                    .iter()
-                    .map(|f| cx.tcx.type_of(f.did).clean(cx))
-                    .collect(),
+                self.fields.iter().map(|f| cx.tcx.type_of(f.did).clean(cx)).collect(),
             ),
             CtorKind::Fictive => Variant::Struct(VariantStruct {
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2018" "--unstable-features" "--skip-children" "--check" "/checkout/src/librustdoc/clean/cfg/tests.rs" "/checkout/src/librustdoc/clean/types.rs" "/checkout/src/librustdoc/clean/simplify.rs" "/checkout/src/librustdoc/clean/inline.rs" "/checkout/src/librustdoc/clean/utils/tests.rs" "/checkout/src/librustdoc/clean/mod.rs" "/checkout/src/librustdoc/clean/blanket_impl.rs" "/checkout/src/librustdoc/passes/collect_trait_impls.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
                 struct_type: CtorKind::Fictive,
