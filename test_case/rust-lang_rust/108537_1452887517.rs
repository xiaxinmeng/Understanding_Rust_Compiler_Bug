plain
Prepare all required actions
Getting action download info
Download action repository 'actions/checkout@v3' (SHA:ac593985615ec2ede58e132d2e21d2b1cbd6127c)
Download action repository 'rust-lang/simpleinfra@master' (SHA:0fdabd83e1d3faaa8e9cfd7c00031e3a92997344)
Complete job name: PR (x86_64-gnu-tools, false, ubuntu-20.04-xl)
git config --global core.autocrlf false
shell: /usr/bin/bash --noprofile --norc -e -o pipefail {0}
env:
  CI_JOB_NAME: x86_64-gnu-tools
---
Successfully built eedd0204933f
Successfully tagged rust-ci:latest
Built container sha256:eedd0204933ff5e0310093718166894ca12063ad4d6a0ef136bf05d18d3db7a2
Uploading finished image to https://ci-caches.rust-lang.org/docker/2022246eea0fceb8dd4e4dc9e56d028016fd8dd73ce83342144a1d278bb5faf6c136af0476a9cbe3f734e6749b99181e6738ca41b0f413c1eb4f076dd7a84292
upload failed: - to s3://rust-lang-ci-sccache2/docker/2022246eea0fceb8dd4e4dc9e56d028016fd8dd73ce83342144a1d278bb5faf6c136af0476a9cbe3f734e6749b99181e6738ca41b0f413c1eb4f076dd7a84292 Unable to locate credentials
[CI_JOB_NAME=x86_64-gnu-tools]
---
.......... (90/97)
......    (97/97)


/checkout/tests/rustdoc-gui/search-tab-change-title-fn-sig.goml search-tab-change-title-fn-sig... FAILED
[ERROR] (line 64) Error: Evaluation failed: The following errors happened: [`In Names (0)` doesn't start with `In Function Parameters` (for STARTS_WITH check)]: for command `assert-text: ("#search-tabs > button:nth-of-type(1)", "In Function Parameters", STARTS_WITH)`
Build completed unsuccessfully in 0:02:38
