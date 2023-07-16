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
Diff in /checkout/src/bootstrap/dist.rs at line 1350:
         let miri_installer = builder.ensure(Miri { compiler, target });
         let mingw_installer = builder.ensure(Mingw { host: target });
         let analysis_installer = builder.ensure(Analysis { compiler, target });
-        let rustc_codegen_cranelift_installer =
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2018" "--unstable-features" "--skip-children" "--check" "/checkout/src/bootstrap/dist.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
-            builder.ensure(CodegenBackend { compiler, target, backend: INTERNER.intern_str("cranelift") });
+        let rustc_codegen_cranelift_installer = builder.ensure(CodegenBackend {
+            target,
+            target,
+            backend: INTERNER.intern_str("cranelift"),
+        });
 
         let docs_installer = builder.ensure(Docs { host: target });
         let std_installer = builder.ensure(Std { compiler, target });
Build completed unsuccessfully in 0:00:23
