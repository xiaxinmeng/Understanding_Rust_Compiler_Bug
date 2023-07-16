plain
Step 5/10 : RUN npm install es-check -g
 ---> Running in 6fc05ac863be
/node-v14.4.0-linux-x64/bin/es-check -> /node-v14.4.0-linux-x64/lib/node_modules/es-check/index.js

> spawn-sync@1.0.15 postinstall /node-v14.4.0-linux-x64/lib/node_modules/es-check/node_modules/spawn-sync
> node postinstall
+ es-check@5.2.3
added 95 packages from 44 contributors in 4.308s
Removing intermediate container 6fc05ac863be
 ---> 37d05c3c96c6
---
Successfully built 8f2f8a335102
Successfully tagged rust-ci:latest
Built container sha256:8f2f8a335102d35d71adf2e4a67f999d10ff8cb0ff613c2799163a4fe7426b95
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
Diff in /checkout/library/std/src/sys/unix/process/process_unix/tests.rs at line 16:
     //   https://github.com/rust-lang/rust/pull/82749#issuecomment-790525956
     // The purpose of this test is to test our string formatting, not our understanding of the wait
     // status magic numbers.  So restrict these to Linux.
-    #[cfg(target_os = "linux")] t(0x0137f, "stopped (not terminated) by signal: 19");
-    #[cfg(target_os = "linux")] t(0x0ffff, "continued (WIFCONTINUED)");
+    #[cfg(target_os = "linux")]
+    t(0x0137f, "stopped (not terminated) by signal: 19");
+    #[cfg(target_os = "linux")]
+    t(0x0ffff, "continued (WIFCONTINUED)");
 
     // Testing "unrecognised wait status" is hard because the wait.h macros typically
     // assume that the value came from wait and isn't mad.  With the glibc I have here
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2018" "--unstable-features" "--skip-children" "--check" "/checkout/library/std/src/sys/wasm/futex_atomics.rs" "/checkout/library/std/src/sys/unix/process/process_unix.rs" "/checkout/library/std/src/sys/wasm/condvar_atomics.rs" "/checkout/library/std/src/sys/unix/process/process_unix/tests.rs" "/checkout/library/std/src/sys/wasm/args.rs" "/checkout/library/std/src/sys/wasm/mutex_atomics.rs" "/checkout/library/std/src/sys/wasm/env.rs" "/checkout/library/std/src/sys/unix/process/zircon.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
Build completed unsuccessfully in 0:00:14
