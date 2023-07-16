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
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2018" "--unstable-features" "--skip-children" "--check" "/checkout/compiler/rustc_mir/src/transform/inline/cycle.rs"` failed.
Diff in /checkout/compiler/rustc_mir/src/transform/inline/cycle.rs at line 27:
         !tcx.is_constructor(root.def_id()),
         "you should not call `mir_callgraph_reachable` on enum/struct constructor functions"
     );
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
-    #[instrument(level = "debug", skip(tcx, param_env, target, stack, seen, recursion_limiter, caller))]
+    #[instrument(
+        level = "debug",
+        skip(tcx, param_env, target, stack, seen, recursion_limiter, caller)
+    )]
     fn process(
         tcx: TyCtxt<'tcx>,
         param_env: ty::ParamEnv<'tcx>,
Build completed unsuccessfully in 0:00:20
