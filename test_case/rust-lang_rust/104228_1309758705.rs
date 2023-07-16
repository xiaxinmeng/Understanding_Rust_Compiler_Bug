plain
  IMAGE: x86_64-gnu-tools
##[endgroup]
From https://github.com/rust-lang/rust
 * branch              master     -> FETCH_HEAD
Searching for toolstate changes between 34115d040b43d9a0dcc313c6282520a86d1e6f61 and c3afb779383240b3a633164be431fb0b649220a3
Tool subtrees were updated
##[group]Run src/ci/scripts/verify-channel.sh
src/ci/scripts/verify-channel.sh
shell: /usr/bin/bash --noprofile --norc -e -o pipefail {0}
env:
---
Successfully built 8352dbccb878
Successfully tagged rust-ci:latest
Built container sha256:8352dbccb878c80a458086132cfc1d75abaa1ee32efad812ae7f25547fc87d4e
Uploading finished image to https://ci-caches.rust-lang.org/docker/a7de7081292bf9550c60f774babc3a3cc20c71cf4ee88f4e0e22a5b13590c6a167ff6a8ef62d853752a5958294e2204fc7d742f5d68eccfbbe7381ab495c4964
upload failed: - to s3://rust-lang-ci-sccache2/docker/a7de7081292bf9550c60f774babc3a3cc20c71cf4ee88f4e0e22a5b13590c6a167ff6a8ef62d853752a5958294e2204fc7d742f5d68eccfbbe7381ab495c4964 Unable to locate credentials
[CI_JOB_NAME=x86_64-gnu-tools]
---
.......... (80/83)
...        (83/83)


/checkout/src/test/rustdoc-gui/code-sidebar-toggle.goml code-sidebar-toggle... FAILED
[ERROR] (line 4) Error: Execution context was destroyed, most likely because of a navigation.: for command `wait-for: "#sidebar-toggle"`
Build completed unsuccessfully in 0:02:06
