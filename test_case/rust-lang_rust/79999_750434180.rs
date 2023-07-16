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
Diff in /checkout/compiler/rustc_mir/src/util/pretty.rs at line 413:
         ty::Int(_) | ty::Uint(_) | ty::Bool | ty::Char | ty::Float(_) => false,
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2018" "--unstable-features" "--skip-children" "--check" "/checkout/compiler/rustc_mir/src/util/pretty.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
         // Unit type
         ty::Tuple(g_args) if g_args.is_empty() => false,
-        ty::Tuple(g_args) => g_args.iter().any(|g_arg| {
-            use_verbose(&g_arg.expect_ty())
-        }),
+        ty::Tuple(g_args) => g_args.iter().any(|g_arg| use_verbose(&g_arg.expect_ty())),
         ty::Array(ty, _) => use_verbose(ty),
         ty::FnDef(..) => false,
failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test --stage 2 src/tools/tidy
Build completed unsuccessfully in 0:00:18
