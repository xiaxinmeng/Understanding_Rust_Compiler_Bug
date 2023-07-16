plain
  TOOLSTATE_REPO: https://github.com/rust-lang-nursery/rust-toolstate
  CACHE_DOMAIN: ci-caches.rust-lang.org
  IMAGE: mingw-check
##[endgroup]
fatal: unknown commit origin/beta
fatal: unknown commit origin/master
All commits in `HEAD` are present in `beta`
All commits in `HEAD` are present in `master`
src/ci/scripts/verify-stable-version-number.sh
shell: /usr/bin/bash --noprofile --norc -e -o pipefail {0}
env:
  CI_JOB_NAME: mingw-check
---
configure: rust.debug-assertions := True
configure: rust.overflow-checks := True
configure: llvm.assertions      := True
configure: dist.missing-tools   := True
configure: build.configure-args := ['--enable-sccache', '--disable-manage-submodu ...
configure: writing `config.toml` in current directory
configure: 
configure: run `python /checkout/x.py --help`
Attempting with retry: make prepare
---
#=#=#                                                                         
curl: (22) The requested URL returned error: 404 
error: failed to download llvm from ci

help: old builds get deleted after a certain time
help: if trying to compile an old commit of rustc, disable `download-ci-llvm` in config.toml:
[llvm]
download-ci-llvm = false

Build completed unsuccessfully in 0:00:00
