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
Diff in /checkout/src/bootstrap/doc.rs at line 475:
             } else if !path.is_empty() {
                 &path[0]
-                continue
+                continue;
             };
             };
             if krates.contains(&requested_crate) {
                 let index = out.join(requested_crate).join("index.html");
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2018" "--unstable-features" "--skip-children" "--check" "/checkout/src/bootstrap/install.rs" "/checkout/src/tools/lint-docs/src/lib.rs" "/checkout/src/bootstrap/format.rs" "/checkout/src/tools/lint-docs/src/main.rs" "/checkout/src/bootstrap/doc.rs" "/checkout/src/bootstrap/clean.rs" "/checkout/src/bootstrap/setup.rs" "/checkout/src/tools/lint-docs/src/groups.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
Build completed unsuccessfully in 0:00:13
