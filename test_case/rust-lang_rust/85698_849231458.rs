plain
Successfully built 0549de58e83f
Successfully tagged rust-ci:latest
Built container sha256:0549de58e83ffabca7996d1cd1b5dcce89bf635040c4945fc26944ef7fe9e660
Uploading finished image to https://ci-caches.rust-lang.org/docker/5077ec353cd1491e6ee21dfbed017c039592486e2f15c335d9cc920ab986ce86dc748c415ad9e07b56bf3231849d98e46583667527e0138e1dc31d247bb94cde
upload failed: - to s3://rust-lang-ci-sccache2/docker/5077ec353cd1491e6ee21dfbed017c039592486e2f15c335d9cc920ab986ce86dc748c415ad9e07b56bf3231849d98e46583667527e0138e1dc31d247bb94cde Unable to locate credentials
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
tidy: Skipping binary file check, read-only filesystem
Checking which error codes lack tests...
* 625 error codes
* highest error code: E0783
tidy error: /checkout/src/test/run-make/incremental-session-fail/foo.rs: empty file
tidy error: /checkout/src/test/run-make/incremental-session-fail/foo.rs: leading newline
Found 0 error codes with no tests
Done!
* 337 features
some tidy checks failed
some tidy checks failed


command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/rust-tidy" "/checkout" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "/checkout/obj/build" "16"


failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test --stage 2 src/tools/tidy
Build completed unsuccessfully in 0:00:13
