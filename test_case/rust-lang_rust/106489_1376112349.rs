plain
Successfully built 8bcd813a5758
Successfully tagged rust-ci:latest
Built container sha256:8bcd813a5758e0002659109f77b257a1bece02c8e8085b77cf71087d1b32b777
Uploading finished image to https://ci-caches.rust-lang.org/docker/b8cdc76b9b2adf6aaac1172d56fab8a31470587d5348a0014d9e27d5bcbbe8577c6dad29501e9359634fe46a53fc6079832ca8161c7036f56b2737f782f779b6
upload failed: - to s3://rust-lang-ci-sccache2/docker/b8cdc76b9b2adf6aaac1172d56fab8a31470587d5348a0014d9e27d5bcbbe8577c6dad29501e9359634fe46a53fc6079832ca8161c7036f56b2737f782f779b6 Unable to locate credentials
[CI_JOB_NAME=mingw-check-tidy]
[CI_JOB_NAME=mingw-check-tidy]
---
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
Diff in /checkout/compiler/rustc_codegen_ssa/src/back/link.rs at line 1257:
                 } else if stem == "lld-link" || stem.starts_with("lld-link-") {
                     LinkerFlavor::Msvc(Lld::Yes)
                 } else if stem == "lld"
-                    || stem.strip_prefix("lld-").filter(|x| x.chars().all(char::is_numeric)).is_some()
+                    || stem
+                        .strip_prefix("lld-")
+                        .filter(|x| x.chars().all(char::is_numeric))
+                        .is_some()
                     || stem == "rust-lld"
                 {
                     let lld_flavor = sess.target.linker_flavor.lld_flavor();
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2021" "--unstable-features" "--skip-children" "--check" "/checkout/compiler/rustc_codegen_ssa/src/back/lto.rs" "/checkout/compiler/rustc_codegen_ssa/src/back/link.rs" "/checkout/compiler/rustc_codegen_ssa/src/back/write.rs" "/checkout/compiler/rustc_codegen_ssa/src/back/linker.rs" "/checkout/compiler/rustc_codegen_ssa/src/mir/intrinsic.rs" "/checkout/compiler/rustc_codegen_ssa/src/back/symbol_export.rs" "/checkout/compiler/rustc_codegen_ssa/src/mir/rvalue.rs" "/checkout/compiler/rustc_codegen_ssa/src/back/rpath/tests.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
