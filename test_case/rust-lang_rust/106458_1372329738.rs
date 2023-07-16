plain
  IMAGE: x86_64-gnu-tools
##[endgroup]
From https://github.com/rust-lang/rust
 * branch              master     -> FETCH_HEAD
Searching for toolstate changes between e94fab69d020d75517cb55fafacb2d270ad6e0ac and 029f6438531d4531ed178e9df216bb2329080c3f
Tool subtrees were updated
##[group]Run src/ci/scripts/verify-channel.sh
src/ci/scripts/verify-channel.sh
shell: /usr/bin/bash --noprofile --norc -e -o pipefail {0}
env:
---
Successfully built c6fa8e1a8dd0
Successfully tagged rust-ci:latest
Built container sha256:c6fa8e1a8dd088114ff861ab75cf2a2e8e121a5ad2a037cca47382680407068d
Uploading finished image to https://ci-caches.rust-lang.org/docker/26b450fac390e4f50222c361d4ab7aa11c0e774b1521f43fe2f059cf5169485c5005bfc649082a3f0648dce898c9dac6499c05937814f10f93bb21a737fb3c05
upload failed: - to s3://rust-lang-ci-sccache2/docker/26b450fac390e4f50222c361d4ab7aa11c0e774b1521f43fe2f059cf5169485c5005bfc649082a3f0648dce898c9dac6499c05937814f10f93bb21a737fb3c05 Unable to locate credentials
[CI_JOB_NAME=x86_64-gnu-tools]
---
.......... (90/97)
.......    (97/97)


/checkout/tests/rustdoc-gui/scrape-examples-button-focus.goml scrape-examples-button-focus... FAILED
[ERROR] (line 12) Error: Evaluation failed: The following errors happened (for selector `.scraped-example-list > .scraped-example pre`): [Expected `248` for property `scrollTop`, found `249`]: for command `assert-property: (".scraped-example-list > .scraped-example pre", {
 "scrollTop": |initialScrollTop|

Build completed unsuccessfully in 0:02:27
