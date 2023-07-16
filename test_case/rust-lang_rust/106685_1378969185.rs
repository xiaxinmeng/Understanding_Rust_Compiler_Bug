plain
  IMAGE: x86_64-gnu-tools
##[endgroup]
From https://github.com/rust-lang/rust
 * branch              stable     -> FETCH_HEAD
Searching for toolstate changes between 69f9c33d71c871fc16ac445211281c6e7a340943 and e074679babd9405613d6cc573e59b3548b2b322d
Submodules were updated
##[group]Run src/ci/scripts/verify-channel.sh
src/ci/scripts/verify-channel.sh
shell: /usr/bin/bash --noprofile --norc -e -o pipefail {0}
env:
---
  CACHE_DOMAIN: ci-caches.rust-lang.org
  CI_ONLY_WHEN_SUBMODULES_CHANGED: 1
  IMAGE: x86_64-gnu-tools
##[endgroup]
fatal: unknown commit origin/master
fatal: unknown commit origin/beta
All commits in `HEAD` are present in `master`
All commits in `HEAD` are present in `beta`
src/ci/scripts/verify-stable-version-number.sh
shell: /usr/bin/bash --noprofile --norc -e -o pipefail {0}
env:
  CI_JOB_NAME: x86_64-gnu-tools
---
Successfully built 688d6463777b
Successfully tagged rust-ci:latest
Built container sha256:688d6463777b9cda0cc38e9a61c9836630715690386e354343c5892666e5b065
Uploading finished image to https://ci-caches.rust-lang.org/docker/3cce3287a6ccbb8896e4ab15f01b2364be6585eb8b9c83c8427653d219a1c38f9a5c712c396a390ddcd36b7d8114915f405cd93de61741383695e972b8dab852
upload failed: - to s3://rust-lang-ci-sccache2/docker/3cce3287a6ccbb8896e4ab15f01b2364be6585eb8b9c83c8427653d219a1c38f9a5c712c396a390ddcd36b7d8114915f405cd93de61741383695e972b8dab852 Unable to locate credentials
[CI_JOB_NAME=x86_64-gnu-tools]
---
curl: (22) The requested URL returned error: 404

error: failed to download llvm from ci

help: old builds get deleted after a certain time
help: if trying to compile an old commit of rustc, disable `download-ci-llvm` in config.toml:
[llvm]
download-ci-llvm = false

Build completed unsuccessfully in 0:00:53
