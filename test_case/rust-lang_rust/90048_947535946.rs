plain
  IMAGE: x86_64-gnu-tools
##[endgroup]
From https://github.com/rust-lang/rust
 * branch              master     -> FETCH_HEAD
Searching for toolstate changes between 42983a28ab3c70728da7a9b932b667c978dd898d and 91253a143ff74074a214d88c0b8ffe11060efe55
Rustdoc was updated
##[group]Run src/ci/scripts/verify-channel.sh
src/ci/scripts/verify-channel.sh
shell: /bin/bash --noprofile --norc -e -o pipefail {0}
env:
---
Successfully built b43ae3cc8bb8
Successfully tagged rust-ci:latest
Built container sha256:b43ae3cc8bb85bb68b2ab3239b0bdc9f77b8b78c6b3dda63966c1fa929a1019f
Uploading finished image to https://ci-caches.rust-lang.org/docker/5294d739a53747a359932b2a7d080ef7f276f7764dd58169beab129a99fc427e583428427a4d1ba8f78dca7de85496d3dd0f2e11f0efef0ee2a625ab1856f35e
upload failed: - to s3://rust-lang-ci-sccache2/docker/5294d739a53747a359932b2a7d080ef7f276f7764dd58169beab129a99fc427e583428427a4d1ba8f78dca7de85496d3dd0f2e11f0efef0ee2a625ab1856f35e Unable to locate credentials
[CI_JOB_NAME=x86_64-gnu-tools]
---
.......... (40/46)
......     (46/46)


docblock-code-block-line-number... FAILED
[ERROR] line 20: unexpected `,` before `}`


command did not execute successfully: "/node-v14.4.0-linux-x64/bin/node" "/checkout/src/tools/rustdoc-gui/tester.js" "--jobs" "16" "--doc-folder" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-gui/doc" "--tests-folder" "/checkout/src/test/rustdoc-gui"


Build completed unsuccessfully in 0:00:16
