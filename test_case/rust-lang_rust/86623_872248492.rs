plain
Successfully built 2e707c743fc9
Successfully tagged rust-ci:latest
Built container sha256:2e707c743fc9c2d0db3de0c772184d15a2be49f306e9714fe6008717eeafc602
Uploading finished image to https://ci-caches.rust-lang.org/docker/2c434b6d7ee6b941de8702087775018a2e9071a3bc304b04a01a34639177d8e2e73518dd8add4365c41db966877a173397bfbbad811a06a04d7f125ab6537886
upload failed: - to s3://rust-lang-ci-sccache2/docker/2c434b6d7ee6b941de8702087775018a2e9071a3bc304b04a01a34639177d8e2e73518dd8add4365c41db966877a173397bfbbad811a06a04d7f125ab6537886 Unable to locate credentials
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
* highest error code: E0783
Found 498 error codes
Found 0 error codes with no tests
Done!
tidy error: /checkout/src/ci/docker/host-x86_64/mingw-check/validate-error-codes.sh:9: line longer than 100 chars
some tidy checks failed



command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/rust-tidy" "/checkout" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "/checkout/obj/build" "16"


failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test --stage 2 src/tools/tidy
Build completed unsuccessfully in 0:00:12
