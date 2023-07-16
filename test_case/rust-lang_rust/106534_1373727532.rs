plain
  IMAGE: x86_64-gnu-tools
##[endgroup]
From https://github.com/rust-lang/rust
 * branch              master     -> FETCH_HEAD
Searching for toolstate changes between 1146560e1a53d26d04b33548d4eeb8e083d78509 and 52d9a16d1402604fdc44ed6ef9e29dacaba652e1
Rustdoc was updated
##[group]Run src/ci/scripts/verify-channel.sh
src/ci/scripts/verify-channel.sh
shell: /usr/bin/bash --noprofile --norc -e -o pipefail {0}
env:
---
Successfully built 1e2edfdfe9be
Successfully tagged rust-ci:latest
Built container sha256:1e2edfdfe9be420b366dbd40ce43bf67f3f721bab725846a2cb430ad5fdb9ef9
Uploading finished image to https://ci-caches.rust-lang.org/docker/20e282a34cee7bfac34a363fba5eab7188feb5765e1df3f3806f81ab75a29adf844f1d59d006d045b25f47b01f6793d2e2aa6491654583b056774e0ea7eeb528
upload failed: - to s3://rust-lang-ci-sccache2/docker/20e282a34cee7bfac34a363fba5eab7188feb5765e1df3f3806f81ab75a29adf844f1d59d006d045b25f47b01f6793d2e2aa6491654583b056774e0ea7eeb528 Unable to locate credentials
[CI_JOB_NAME=x86_64-gnu-tools]
---
......... (90/97)
.......    (97/97)


/checkout/src/test/rustdoc-gui/source-code-page.goml source-code-page... FAILED
[ERROR] line 137: variable `x` not found in options nor environment
