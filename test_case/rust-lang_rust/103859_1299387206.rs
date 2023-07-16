plain
  CACHE_DOMAIN: ci-caches.rust-lang.org
  CI_ONLY_WHEN_SUBMODULES_CHANGED: 1
  IMAGE: x86_64-gnu-tools
##[endgroup]
fatal: unknown commit origin/master
fatal: unknown commit origin/beta
All commits in `HEAD` are present in `beta`
All commits in `HEAD` are present in `master`
src/ci/scripts/verify-stable-version-number.sh
shell: /usr/bin/bash --noprofile --norc -e -o pipefail {0}
env:
  CI_JOB_NAME: x86_64-gnu-tools
---
curl: (22) The requested URL returned error: 404

error: failed to download llvm from ci

help: old builds get deleted after a certain time
help: if trying to compile an old commit of rustc, disable `download-ci-llvm` in config.toml:

[llvm]
download-ci-llvm = false

