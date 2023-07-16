plain
Successfully built 7a232ae27a56
Successfully tagged rust-ci:latest
Built container sha256:7a232ae27a56e2054ffd3c283e14a87391362ae27c524759370b17bb46a5d437
Uploading finished image to https://ci-caches.rust-lang.org/docker/f660c3ad6bae343070b47dc83125c3e2707aea949eaba6c703e6e712f1d5f695815823062f0ccc90ed4dc634b4c2097c53175bcc314ac8405b0a16c0fd2aa018
upload failed: - to s3://rust-lang-ci-sccache2/docker/f660c3ad6bae343070b47dc83125c3e2707aea949eaba6c703e6e712f1d5f695815823062f0ccc90ed4dc634b4c2097c53175bcc314ac8405b0a16c0fd2aa018 Unable to locate credentials
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
Diff in /checkout/library/alloc/src/lib.rs at line 65:
 #![doc(
     html_playground_url = "https://play.rust-lang.org/",
     issue_tracker_base_url = "https://github.com/rust-lang/rust/issues/",
-    test(no_crate_inject, attr(allow(unused_variables), deny(warnings))),
+    test(no_crate_inject, attr(allow(unused_variables), deny(warnings)))
 )]
-#![cfg_attr(not(bootstrap),
+#![cfg_attr(
+    not(bootstrap),
     doc(cfg_hide(not(test), not(any(test, bootstrap)), target_has_atomic = "ptr"))
 )]
 #![no_std]
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2018" "--unstable-features" "--skip-children" "--check" "/checkout/library/alloc/src/collections/btree/node/tests.rs" "/checkout/library/alloc/src/collections/vec_deque/mod.rs" "/checkout/library/alloc/src/collections/binary_heap.rs" "/checkout/library/alloc/src/collections/mod.rs" "/checkout/library/alloc/src/lib.rs" "/checkout/library/alloc/src/rc/tests.rs" "/checkout/library/alloc/src/prelude/mod.rs" "/checkout/library/alloc/src/collections/btree/mod.rs"` failed.
Build completed unsuccessfully in 0:00:11
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
