plain
  CACHE_DOMAIN: ci-caches.rust-lang.org
  CI_ONLY_WHEN_SUBMODULES_CHANGED: 1
  IMAGE: x86_64-gnu-tools
##[endgroup]
fatal: unknown commit 53fd98ca776cb875bc9e5514f56b52eb74f9e7a9
All commits in `HEAD` are present in `master`
src/ci/scripts/verify-stable-version-number.sh
shell: /usr/bin/bash --noprofile --norc -e -o pipefail {0}
env:
  CI_JOB_NAME: x86_64-gnu-tools
---
.......... (90/91)
.          (91/91)


/checkout/src/test/rustdoc-gui/code-sidebar-toggle.goml code-sidebar-toggle... FAILED
[ERROR] (line 4) Error: Execution context was destroyed, most likely because of a navigation.: for command `wait-for: "#sidebar-toggle"`
Build completed unsuccessfully in 0:02:22
