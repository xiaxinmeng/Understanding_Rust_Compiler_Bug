plain
Removing intermediate container 83b0f72e453a
 ---> 8292d051df8a
Step 5/10 : RUN npm install es-check -g
 ---> Running in 4ad8010d6ef9
/node-v14.4.0-linux-x64/bin/es-check -> /node-v14.4.0-linux-x64/lib/node_modules/es-check/index.js

> spawn-sync@1.0.15 postinstall /node-v14.4.0-linux-x64/lib/node_modules/es-check/node_modules/spawn-sync
> node postinstall
+ es-check@5.2.1
added 95 packages from 44 contributors in 3.581s
Removing intermediate container 4ad8010d6ef9
 ---> 9467df7855a6
---
Successfully built 3c46b266ec56
Successfully tagged rust-ci:latest
Built container sha256:3c46b266ec56879acdb2ea9d25d9fd202558a86f7a01a4442537fe8ff0aa3d18
Uploading finished image to https://ci-caches.rust-lang.org/docker/9f2a38e35a8211f9c2c342213b6dabbf1ce1e957c3f9f4a6874af054aa93d446d1c6f8252277cb4118d76ddf7862eec7c972b385df9acf97d8b518b20c0181e6
upload failed: - to s3://rust-lang-ci-sccache2/docker/9f2a38e35a8211f9c2c342213b6dabbf1ce1e957c3f9f4a6874af054aa93d446d1c6f8252277cb4118d76ddf7862eec7c972b385df9acf97d8b518b20c0181e6 Unable to locate credentials
[CI_JOB_NAME=mingw-check]
---
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
Diff in /checkout/src/librustdoc/clean/types.rs at line 869:
                     None => {
                         if let Some(ref fragment) = *fragment {
                             let url = match cache.extern_locations.get(krate) {
-                                Some(&(_, _, ExternalLocation::Local)) => {
-                                    "../".repeat(depth)
-                                }
+                                Some(&(_, _, ExternalLocation::Local)) => "../".repeat(depth),
                                 Some(&(_, _, ExternalLocation::Remote(ref s))) => s.to_string(),
                                 Some(&(_, _, ExternalLocation::Unknown)) | None => String::from(
                                     // NOTE: intentionally doesn't pass crate name to avoid having
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2018" "--unstable-features" "--skip-children" "--check" "/checkout/src/librustdoc/clean/cfg.rs" "/checkout/src/librustdoc/doctree.rs" "/checkout/src/librustdoc/clean/simplify.rs" "/checkout/src/librustdoc/core.rs" "/checkout/src/librustdoc/clean/cfg/tests.rs" "/checkout/src/librustdoc/clean/types.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
Build completed unsuccessfully in 0:00:13
