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
Diff in /checkout/src/bootstrap/tool.rs at line 217:
             if tool == "tidy" {
                 tool = "rust-tidy";
-            let cargo_out =
-            let cargo_out =
-                builder.cargo_out(compiler, self.mode, target).join(exe(tool, target));
+            let cargo_out = builder.cargo_out(compiler, self.mode, target).join(exe(tool, target));
             let bin = builder.tools_dir(compiler).join(exe(tool, target));
             builder.copy(&cargo_out, &bin);
             Some(bin)
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2018" "--unstable-features" "--skip-children" "--check" "/checkout/src/bootstrap/build.rs" "/checkout/src/bootstrap/lib.rs" "/checkout/src/bootstrap/compile.rs" "/checkout/src/bootstrap/util.rs" "/checkout/src/bootstrap/tool.rs" "/checkout/src/bootstrap/channel.rs" "/checkout/src/bootstrap/sanity.rs" "/checkout/src/bootstrap/bin/main.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
Build completed unsuccessfully in 0:00:13
