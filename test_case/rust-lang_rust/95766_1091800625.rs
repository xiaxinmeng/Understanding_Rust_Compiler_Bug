plain
configure: rust.debug-assertions := True
configure: rust.overflow-checks := True
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
Diff in /checkout/src/bootstrap/builder.rs at line 1045:
                 // Only run clippy on a very limited subset of crates (in particular, not build scripts).
                 cargo.arg("-Zunstable-options");
                 // Explicitly does *not* set `--cfg=bootstrap`, since we're using a nightly clippy.
-                let host_version = Command::new(&self.initial_rustc).arg("--version").output().map_err(|_| ());
+                let host_version =
+                    Command::new(&self.initial_rustc).arg("--version").output().map_err(|_| ());
                 let output = host_version.and_then(|output| {
                     if output.status.success() {
                         Ok(output)
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2021" "--unstable-features" "--skip-children" "--check" "/checkout/src/bootstrap/builder.rs" "/checkout/src/bootstrap/builder/tests.rs" "/checkout/src/librustdoc/formats/item_type.rs" "/checkout/src/librustdoc/formats/renderer.rs" "/checkout/src/librustdoc/formats/cache.rs" "/checkout/src/librustdoc/formats/mod.rs" "/checkout/compiler/rustc_fs_util/src/lib.rs" "/checkout/src/librustdoc/theme/tests.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
