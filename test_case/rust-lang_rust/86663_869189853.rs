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
Diff in /checkout/src/bootstrap/doc.rs at line 380:
                     .arg("https://doc.rust-lang.org/rust.css");
             } else {
                 cmd.arg("--markdown-css")
-                .arg(format!("rustdoc{}.css", &builder.version))
-                .arg("--markdown-css")
-                .arg("rust.css");
+                    .arg(format!("rustdoc{}.css", &builder.version))
+                    .arg("--markdown-css")
+                    .arg("rust.css");
             builder.run(&mut cmd);
         }
         }
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2018" "--unstable-features" "--skip-children" "--check" "/checkout/src/bootstrap/setup.rs" "/checkout/src/bootstrap/util.rs" "/checkout/src/bootstrap/toolstate.rs" "/checkout/src/bootstrap/dist.rs" "/checkout/src/bootstrap/run.rs" "/checkout/src/bootstrap/doc.rs" "/checkout/src/bootstrap/format.rs" "/checkout/src/bootstrap/bin/sccache-plus-cl.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
Build completed unsuccessfully in 0:00:15
