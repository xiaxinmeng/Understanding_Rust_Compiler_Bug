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
Diff in /checkout/src/bootstrap/builder.rs at line 822:
             Color::Always => {
                 cargo.arg("--color=always");
                 for log in &color_logs {
-                  cargo.env(log, "always");
+                    cargo.env(log, "always");
             }
             }
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2018" "--unstable-features" "--skip-children" "--check" "/checkout/src/bootstrap/builder.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
             Color::Never => {
Diff in /checkout/src/bootstrap/builder.rs at line 829:
                 cargo.arg("--color=never");
                 for log in &color_logs {
-                  cargo.env(log, "never");
+                    cargo.env(log, "never");
             }
             }
             Color::Auto => {} // nothing to do
Build completed unsuccessfully in 0:00:19
