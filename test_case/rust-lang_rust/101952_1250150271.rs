plain
configure: rust.debug-assertions := True
configure: rust.overflow-checks := True
configure: llvm.assertions      := True
configure: dist.missing-tools   := True
configure: build.configure-args := ['--enable-sccache', '--disable-manage-submodu ...
configure: writing `config.toml` in current directory
configure: 
configure: run `python /checkout/x.py --help`
Attempting with retry: make prepare
---
skip untracked path cpu-usage.csv during rustfmt invocations
skip untracked path src/doc/book/ during rustfmt invocations
skip untracked path src/doc/rust-by-example/ during rustfmt invocations
skip untracked path src/llvm-project/ during rustfmt invocations
Diff in /checkout/src/tools/build-manifest/src/main.rs at line 545:
                     let t = Target::from_compressed_tar(self, &tarball_name!(fallback_target));
                     // Fallbacks must always be available.
                     if !t.available {
-                        eprintln!("{:?} not available for fallback", tarball_name!(fallback_target));
+                        eprintln!(
+                            "{:?} not available for fallback",
+                            tarball_name!(fallback_target)
                         continue;
                     }
                     return t;
                     return t;
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2021" "--unstable-features" "--skip-children" "--check" "/checkout/src/tools/replace-version-placeholder/src/main.rs" "/checkout/src/tools/build-manifest/src/main.rs" "/checkout/src/tools/tidy/src/walk.rs" "/checkout/src/tools/build-manifest/src/manifest.rs" "/checkout/src/tools/tidy/src/extdeps.rs" "/checkout/src/tools/build-manifest/src/versions.rs" "/checkout/src/tools/tidy/src/unstable_book.rs" "/checkout/src/tools/linkchecker/tests/checks.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
