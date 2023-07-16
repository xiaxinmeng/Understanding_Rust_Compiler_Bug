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
Diff in /checkout/compiler/rustc_mir/src/transform/inline.rs at line 454:
             }
         }
 
-        if tcx.sess.opts.debugging_opts.mir_opt_level > 1 && attr::InlineAttr::Always == codegen_fn_attrs.inline {
+        if tcx.sess.opts.debugging_opts.mir_opt_level > 1
+            && attr::InlineAttr::Always == codegen_fn_attrs.inline
+        {
             debug!("INLINING {:?} because inline(always) [cost={}]", callsite, cost);
         } else {
         } else {
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2018" "--unstable-features" "--skip-children" "--check" "/checkout/compiler/rustc_mir/src/transform/inline/cycle.rs" "/checkout/compiler/rustc_mir/src/transform/check_consts/resolver.rs" "/checkout/compiler/rustc_mir/src/transform/inline.rs" "/checkout/compiler/rustc_mir/src/transform/check_consts/post_drop_elaboration.rs" "/checkout/compiler/rustc_mir/src/transform/mod.rs" "/checkout/compiler/rustc_mir/src/transform/check_consts/validation.rs" "/checkout/compiler/rustc_mir/src/transform/remove_noop_landing_pads.rs" "/checkout/compiler/rustc_mir/src/transform/required_consts.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
Build completed unsuccessfully in 0:00:15
