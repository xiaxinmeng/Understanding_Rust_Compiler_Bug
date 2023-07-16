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
Diff in /checkout/src/bootstrap/compile.rs at line 465:
                 cmd.env("RUSTC_BOOTSTRAP", "1");
                 if !builder.local_rebuild {
                     // a local_rebuild compiler already has stage1 features
-                    cmd.arg("--cfg")
-                        .arg("bootstrap");
+                    cmd.arg("--cfg").arg("bootstrap");
                 builder.run(
                 builder.run(
                     cmd.arg("--target")
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2018" "--unstable-features" "--skip-children" "--check" "/checkout/src/bootstrap/metadata.rs" "/checkout/src/bootstrap/config.rs" "/checkout/src/bootstrap/lib.rs" "/checkout/src/bootstrap/compile.rs" "/checkout/src/bootstrap/cc_detect.rs" "/checkout/src/bootstrap/clean.rs" "/checkout/src/bootstrap/dist.rs" "/checkout/src/bootstrap/builder/tests.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
Build completed unsuccessfully in 0:00:15
