plain
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
skip untracked path src/llvm-project/ during rustfmt invocations
Diff in /checkout/src/bootstrap/test.rs at line 473:
     }
 
     fn make_run(run: RunConfig<'_>) {
-        run.builder.ensure(Miri { stage: run.builder.top_stage, host: run.build_triple(), target: run.target });
+        run.builder.ensure(Miri {
+            stage: run.builder.top_stage,
+            host: run.build_triple(),
+            target: run.target,
     }
 
     /// Runs `cargo test` for miri.
     /// Runs `cargo test` for miri.
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2021" "--unstable-features" "--skip-children" "--check" "/checkout/src/bootstrap/metadata.rs" "/checkout/src/bootstrap/test.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
