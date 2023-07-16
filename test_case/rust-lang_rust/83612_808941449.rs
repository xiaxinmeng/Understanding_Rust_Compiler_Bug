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
Diff in /checkout/src/bootstrap/test.rs at line 744:
 
 
                 let mut command = builder.rustdoc_cmd(self.compiler);
-                command.arg(&Path::new("src/test/rustdoc-gui").join(file_name))
+                command
+                    .arg(&Path::new("src/test/rustdoc-gui").join(file_name))
                     .arg("-o")
                     .arg(&out_dir)
                     .arg("-Zunstable-options")
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2018" "--unstable-features" "--skip-children" "--check" "/checkout/src/bootstrap/channel.rs" "/checkout/src/librustdoc/clean/mod.rs" "/checkout/src/librustdoc/clean/cfg.rs" "/checkout/src/bootstrap/flags.rs" "/checkout/src/librustdoc/clean/auto_trait.rs" "/checkout/src/bootstrap/builder.rs" "/checkout/src/librustdoc/clean/simplify.rs" "/checkout/src/bootstrap/test.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
Build completed unsuccessfully in 0:00:15
