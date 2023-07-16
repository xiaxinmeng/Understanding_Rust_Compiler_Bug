
Run src/ci/scripts/verify-backported-commits.sh
  src/ci/scripts/verify-backported-commits.sh
  shell: /bin/bash --noprofile --norc -e -o pipefail {0}
  env:
    CI_JOB_NAME: x86_64-gnu-tools
    SCCACHE_BUCKET: rust-lang-ci-sccache2
    TOOLSTATE_REPO: https://github.com/rust-lang-nursery/rust-toolstate
    CACHE_DOMAIN: ci-caches.rust-lang.org
    CI_ONLY_WHEN_SUBMODULES_CHANGED: 1
    IMAGE: x86_64-gnu-tools
fatal: unknown commit 53fd98ca776cb875bc9e5514f56b52eb74f9e7a9
All commits in `HEAD` are present in `master`
