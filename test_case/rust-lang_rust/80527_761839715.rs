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
Diff in /checkout/src/bootstrap/builder.rs at line 1281:
 
             // cfg(bootstrap), can be removed on the next beta bump
             if compiler.stage == 0 {
-              rustdocflags.arg("-Winvalid_codeblock_attributes");
+                rustdocflags.arg("-Winvalid_codeblock_attributes");
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2018" "--unstable-features" "--skip-children" "--check" "/checkout/src/bootstrap/builder.rs"` failed.
             } else {
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
-              rustdocflags.arg("-Wrustdoc::invalid_codeblock_attributes");
+                rustdocflags.arg("-Wrustdoc::invalid_codeblock_attributes");
         }
 
failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test --stage 2 src/tools/tidy
Build completed unsuccessfully in 0:00:21
