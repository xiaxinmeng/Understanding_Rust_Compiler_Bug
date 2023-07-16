plain
Successfully built 5d5b5389d31a
Successfully tagged rust-ci:latest
Built container sha256:5d5b5389d31a26aed74e65e7c165fc906a1d88cd203efe996a5b63cbc682fd47
Uploading finished image to https://ci-caches.rust-lang.org/docker/b8cdc76b9b2adf6aaac1172d56fab8a31470587d5348a0014d9e27d5bcbbe8577c6dad29501e9359634fe46a53fc6079832ca8161c7036f56b2737f782f779b6
upload failed: - to s3://rust-lang-ci-sccache2/docker/b8cdc76b9b2adf6aaac1172d56fab8a31470587d5348a0014d9e27d5bcbbe8577c6dad29501e9359634fe46a53fc6079832ca8161c7036f56b2737f782f779b6 Unable to locate credentials
[CI_JOB_NAME=mingw-check-tidy]
[CI_JOB_NAME=mingw-check-tidy]
---
configure: rust.debug-assertions := True
configure: rust.overflow-checks := True
configure: llvm.assertions      := True
configure: dist.missing-tools   := True
configure: build.configure-args := ['--enable-sccache', '--disable-manage-submodu ...
configure: writing `config.toml` in current directory
configure: 
configure: run `python /checkout/x.py --help`
Attempting with retry: make prepare
---
skip untracked path cpu-usage.csv during rustfmt invocations
skip untracked path src/doc/book/ during rustfmt invocations
skip untracked path src/doc/rust-by-example/ during rustfmt invocations
skip untracked path src/llvm-project/ during rustfmt invocations
Diff in /checkout/src/bootstrap/check.rs at line 363:
             "Checking stage{} {} artifacts ({} -> {})",
             compiler.stage, "rust-analyzer", &compiler.host.triple, target.triple
         ));
-        run_cargo(builder, cargo, args(builder), &stamp(builder, compiler, target), vec![], true, false);
+        run_cargo(
+            cargo,
+            args(builder),
+            args(builder),
+            &stamp(builder, compiler, target),
+            vec![],
+            false,
+        );
 
 
         /// Cargo's output path in a given stage, compiled by a particular
         /// compiler for the specified target.
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2021" "--unstable-features" "--skip-children" "--check" "/checkout/compiler/rustc_expand/src/tokenstream/tests.rs" "/checkout/compiler/rustc/build.rs" "/checkout/compiler/rustc_expand/src/module.rs" "/checkout/compiler/rustc_expand/src/proc_macro_server.rs" "/checkout/compiler/rustc_expand/src/base.rs" "/checkout/src/bootstrap/check.rs" "/checkout/compiler/rustc_expand/src/parse/tests.rs" "/checkout/compiler/rustc_expand/src/config.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
