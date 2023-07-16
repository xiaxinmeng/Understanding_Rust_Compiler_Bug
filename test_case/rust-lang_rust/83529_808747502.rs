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
Diff in /checkout/src/bootstrap/tarball.rs at line 48:
                 "src/tools/rustfmt/LICENSE-APACHE",
                 "src/tools/rustfmt/LICENSE-MIT",
             ],
-            OverlayKind::RustDemangler => &[
-                "src/tools/rust-demangler/README.md",
-                "LICENSE-APACHE",
-                "LICENSE-MIT",
-            ],
+            OverlayKind::RustDemangler => {
+                &["src/tools/rust-demangler/README.md", "LICENSE-APACHE", "LICENSE-MIT"]
+            }
             OverlayKind::RLS => &[
                 "src/tools/rls/README.md",
                 "src/tools/rls/LICENSE-APACHE",
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2018" "--unstable-features" "--skip-children" "--check" "/checkout/src/librustdoc/doctest/tests.rs" "/checkout/src/bootstrap/channel.rs" "/checkout/src/librustdoc/error.rs" "/checkout/src/bootstrap/format.rs" "/checkout/src/librustdoc/visit_ast.rs" "/checkout/src/bootstrap/tarball.rs" "/checkout/src/bootstrap/dist.rs" "/checkout/src/bootstrap/toolstate.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
Build completed unsuccessfully in 0:00:14
