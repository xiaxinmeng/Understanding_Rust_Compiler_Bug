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
Diff in /checkout/compiler/rustc_passes/src/stability.rs at line 986:
 
 fn enabling_feature_not_allowed(tcx: TyCtxt<'_>, span: Span) {
     struct_span_err!(
-        tcx.sess, span, E0554, "`#![feature]` may not be used on the {} release channel", option_env!("CFG_RELEASE_CHANNEL").unwrap_or("(unknown)")
-    ).emit();
+        tcx.sess,
+        span,
+        E0554,
+        "`#![feature]` may not be used on the {} release channel",
+        option_env!("CFG_RELEASE_CHANNEL").unwrap_or("(unknown)")
+    )
+    .emit();
 
 
 fn unnecessary_stable_feature_lint(tcx: TyCtxt<'_>, span: Span, feature: Symbol, since: Symbol) {
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2018" "--unstable-features" "--skip-children" "--check" "/checkout/compiler/rustc_data_structures/src/functor.rs" "/checkout/compiler/rustc_data_structures/src/thin_vec.rs" "/checkout/compiler/rustc_parse_format/src/lib.rs" "/checkout/compiler/rustc_passes/src/layout_test.rs" "/checkout/compiler/rustc_parse_format/src/tests.rs" "/checkout/compiler/rustc_passes/src/stability.rs" "/checkout/compiler/rustc_passes/src/weak_lang_items.rs" "/checkout/compiler/rustc_data_structures/src/base_n.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
