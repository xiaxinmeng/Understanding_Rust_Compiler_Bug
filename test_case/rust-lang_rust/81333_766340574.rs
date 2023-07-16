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
Diff in /checkout/compiler/rustc_codegen_ssa/src/mir/constant.rs at line 30:
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2018" "--unstable-features" "--skip-children" "--check" "/checkout/compiler/rustc_codegen_ssa/src/mir/constant.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
                 .tcx()
                 .const_eval_resolve(ty::ParamEnv::reveal_all(), def, substs, promoted, None)
                 .map_err(|err| {
-                    self.cx
-                        .tcx()
-                        .sess
-                        .span_err(constant.span, "erroneous constant encountered");
+                    self.cx.tcx().sess.span_err(constant.span, "erroneous constant encountered");
                 }),
                 }),
             ty::ConstKind::Value(value) => Ok(value),
Build completed unsuccessfully in 0:00:18
