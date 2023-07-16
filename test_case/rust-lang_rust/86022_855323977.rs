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
skip untracked path src/doc/rust-by-example/ during rustfmt invocations
skip untracked path src/llvm-project/ during rustfmt invocations
Diff in /checkout/src/bootstrap/builder.rs at line 1624:
     {
         // ignore errors; we're exiting anyway
         println!("note: failed while building {}", step.name);
-        print!("help: to replicate this failure, run `./x.py {} {} --stage {}",
+        print!(
+            "help: to replicate this failure, run `./x.py {} {} --stage {}",
             step.cmd,
             step.path.display(),
             step.stage,
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2018" "--unstable-features" "--skip-children" "--check" "/checkout/compiler/rustc_infer/src/infer/equate.rs" "/checkout/src/bootstrap/native.rs" "/checkout/src/bootstrap/compile.rs" "/checkout/src/bootstrap/lib.rs" "/checkout/src/bootstrap/util.rs" "/checkout/src/bootstrap/tool.rs" "/checkout/src/bootstrap/builder.rs" "/checkout/compiler/rustc_infer/src/infer/mod.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
note: failed while building bootstrap::test::Tidy
help: to replicate this failure, run `./x.py test src/tools/tidy --stage 2`
Build completed unsuccessfully in 0:00:15
