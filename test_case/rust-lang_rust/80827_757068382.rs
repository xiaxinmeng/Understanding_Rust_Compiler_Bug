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
Diff in /checkout/compiler/rustc_typeck/src/astconv/generics.rs at line 324:
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2018" "--unstable-features" "--skip-children" "--check" "/checkout/compiler/rustc_typeck/src/astconv/generics.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
                         params.next();
                         substs.reserve(params.len());
                         for p in params {
-                          substs.push(ctx.inferred_kind(Some(&substs), p, infer_args));
+                            substs.push(ctx.inferred_kind(Some(&substs), p, infer_args));
                         break;
                     }
failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test --stage 2 src/tools/tidy
Build completed unsuccessfully in 0:00:14
