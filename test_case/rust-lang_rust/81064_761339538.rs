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
Diff in /checkout/src/bootstrap/check.rs at line 84:
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2018" "--unstable-features" "--skip-children" "--check" "/checkout/src/bootstrap/check.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
         );
         std_cargo(builder, target, compiler.stage, &mut cargo);
 
-        builder.info(&format!("Checking stage{} std artifacts ({} -> {})", builder.top_stage, &compiler.host, target));
+        builder.info(&format!(
+            "Checking stage{} std artifacts ({} -> {})",
+            builder.top_stage, &compiler.host, target
+        ));
         run_cargo(
             cargo,
Diff in /checkout/src/bootstrap/check.rs at line 220:
Diff in /checkout/src/bootstrap/check.rs at line 220:
             cargo.arg("-p").arg(krate.name);
 
 
-        builder.info(&format!("Checking stage{} compiler artifacts ({} -> {})", builder.top_stage, &compiler.host, target));
+        builder.info(&format!(
+            "Checking stage{} compiler artifacts ({} -> {})",
+            builder.top_stage, &compiler.host, target
+        ));
         run_cargo(
             cargo,
failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test --stage 2 src/tools/tidy
Build completed unsuccessfully in 0:00:24
