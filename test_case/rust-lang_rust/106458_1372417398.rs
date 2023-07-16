plain
  IMAGE: x86_64-gnu-tools
##[endgroup]
From https://github.com/rust-lang/rust
 * branch              master     -> FETCH_HEAD
Searching for toolstate changes between e94fab69d020d75517cb55fafacb2d270ad6e0ac and 81aba71d78ff5abc1c85d0c0c3763e15997c31ce
Tool subtrees were updated
##[group]Run src/ci/scripts/verify-channel.sh
src/ci/scripts/verify-channel.sh
shell: /usr/bin/bash --noprofile --norc -e -o pipefail {0}
env:
---
Successfully built 28fd335f69ee
Successfully tagged rust-ci:latest
Built container sha256:28fd335f69ee26eb81da60e401d6ee736d720c582aab97b1c8de0b59aacc4e89
Uploading finished image to https://ci-caches.rust-lang.org/docker/26b450fac390e4f50222c361d4ab7aa11c0e774b1521f43fe2f059cf5169485c5005bfc649082a3f0648dce898c9dac6499c05937814f10f93bb21a737fb3c05
upload failed: - to s3://rust-lang-ci-sccache2/docker/26b450fac390e4f50222c361d4ab7aa11c0e774b1521f43fe2f059cf5169485c5005bfc649082a3f0648dce898c9dac6499c05937814f10f93bb21a737fb3c05 Unable to locate credentials
[CI_JOB_NAME=x86_64-gnu-tools]
---
.......... (90/97)
.......    (97/97)


/checkout/tests/rustdoc-gui/basic-code.goml basic-code... FAILED
[ERROR] (line 3) Error: Execution context was destroyed, most likely because of a navigation.: for command `wait-for: ".src-line-numbers"`
Build completed unsuccessfully in 0:02:36
