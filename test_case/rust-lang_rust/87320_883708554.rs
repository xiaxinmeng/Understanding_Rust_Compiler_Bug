plain
Successfully built 90a52368bdb3
Successfully tagged rust-ci:latest
Built container sha256:90a52368bdb3bd39c5dc3538830b53cf1c974c8443c6e6be4789460a1847e26d
Uploading finished image to https://ci-caches.rust-lang.org/docker/b4c226ec1d684c71a4d91fb0cd1a640c97d897266a00433471b3dedeefdb8d321b1df2c8c36c10bd4d6087df0c2b9b507f88d10da95406d6ce945bef9db881e3
upload failed: - to s3://rust-lang-ci-sccache2/docker/b4c226ec1d684c71a4d91fb0cd1a640c97d897266a00433471b3dedeefdb8d321b1df2c8c36c10bd4d6087df0c2b9b507f88d10da95406d6ce945bef9db881e3 Unable to locate credentials
 * branch              master     -> FETCH_HEAD
[CI_JOB_NAME=mingw-check]
[CI_JOB_NAME=mingw-check]
---
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
Diff in /checkout/compiler/rustc_session/src/config.rs at line 1164:
             "FROM=TO",
         opt::opt_s(
-          "",
-          "debug-compilation-dir",
-          "debug-compilation-dir",
-          "Remap source names relative to the current current working \
+            "debug-compilation-dir",
+            "debug-compilation-dir",
+            "Remap source names relative to the current current working \
           directory to be relative to the given argument instead in all output \
           (compiler messages and output files)",
-          "PATH_PREFIX",
-  ]);
+            "PATH_PREFIX",
+        ),
+    ]);
+    ]);
     opts
 }
 
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2018" "--unstable-features" "--skip-children" "--check" "/checkout/compiler/rustc_hir_pretty/src/lib.rs" "/checkout/compiler/rustc_session/src/session.rs" "/checkout/compiler/rustc_session/src/config.rs" "/checkout/compiler/rustc_session/src/code_stats.rs" "/checkout/compiler/rustc_session/src/filesearch.rs" "/checkout/compiler/rustc_session/src/cgu_reuse_tracker.rs" "/checkout/compiler/rustc_session/src/search_paths.rs" "/checkout/compiler/rustc_session/src/utils.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
