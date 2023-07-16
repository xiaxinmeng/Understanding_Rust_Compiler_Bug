plain
  IMAGE: x86_64-gnu-tools
##[endgroup]
From https://github.com/rust-lang/rust
 * branch              master     -> FETCH_HEAD
Searching for toolstate changes between 6f0c4a6c5c36f1f8f433a12e10a29918f3d40a31 and ee8f18645fb91a295b6196bef6d2789d571d425d
Rustdoc was updated
##[group]Run src/ci/scripts/verify-channel.sh
src/ci/scripts/verify-channel.sh
shell: /usr/bin/bash --noprofile --norc -e -o pipefail {0}
env:
---
Successfully built 92bcfe129aec
Successfully tagged rust-ci:latest
Built container sha256:92bcfe129aec18722c0b84aa62fe4d5cf1cb7ab6e2f7fad398f310eb5cacea6d
Uploading finished image to https://ci-caches.rust-lang.org/docker/9fa68dae399e708cb552ab505caa8ca0450986fa24e89d59ac4331b9831c5530f80a8870e13e780973a939b6f668181c1d332a45b5eb4263c3de4f25dafd4002
upload failed: - to s3://rust-lang-ci-sccache2/docker/9fa68dae399e708cb552ab505caa8ca0450986fa24e89d59ac4331b9831c5530f80a8870e13e780973a939b6f668181c1d332a45b5eb4263c3de4f25dafd4002 Unable to locate credentials
[CI_JOB_NAME=x86_64-gnu-tools]
---
.......... (70/72)
.         (72/72)


/checkout/src/test/rustdoc-gui/type-declation-overflow.goml type-declation-overflow... FAILED
[ERROR] (line 35) Error: Evaluation failed: The following errors happened (for selector `.mobile-topbar .location`): [Expected `502` for property `scrollWidth`, found `500`]: for command `assert-property: (".mobile-topbar .location", {"scrollWidth": "502"})`
[ERROR] (line 36) Error: Evaluation failed: The following errors happened (for selector `.mobile-topbar .location`): [Expected `502` for property `clientWidth`, found `500`]: for command `assert-property: (".mobile-topbar .location", {"clientWidth": "502"})`
Build completed unsuccessfully in 0:01:56
