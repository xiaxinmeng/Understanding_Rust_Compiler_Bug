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
###########################################################               82.2%
######################################################################## 100.0%
extracting /checkout/obj/build/cache/2022-06-29/rustfmt-nightly-x86_64-unknown-linux-gnu.tar.xz to /checkout/obj/build/x86_64-unknown-linux-gnu/stage0
skip untracked path cpu-usage.csv during rustfmt invocations
Diff in /checkout/src/bootstrap/compile.rs at line 1119:
         // panic (see #90224). In practice, that's probably ok: nothing currently uses the stage 2
         // sysroot.
         if run.builder.top_stage >= 2 && !run.builder.config.full_bootstrap {
-            run.builder.ensure(Rustc { compiler: target_compiler, target: target_compiler.host, crates: Default::default() });
+            run.builder.ensure(Rustc {
+                compiler: target_compiler,
+                target: target_compiler.host,
+                crates: Default::default(),
             return;
         }
         }
         run.builder.ensure(Assemble { target_compiler });
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2021" "--unstable-features" "--skip-children" "--check" "/checkout/src/bootstrap/tool.rs" "/checkout/src/librustdoc/html/highlight/tests.rs" "/checkout/src/librustdoc/html/static_files.rs" "/checkout/src/bootstrap/dist.rs" "/checkout/src/librustdoc/html/sources.rs" "/checkout/src/bootstrap/compile.rs" "/checkout/src/librustdoc/html/mod.rs" "/checkout/src/librustdoc/html/length_limit/tests.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
thread 'main' panicked at 'called `Result::unwrap()` on an `Err` value: Any { .. }', format.rs:175:19
Build completed unsuccessfully in 0:00:16
