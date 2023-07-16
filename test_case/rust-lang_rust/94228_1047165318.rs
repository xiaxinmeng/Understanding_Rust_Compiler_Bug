plain
  TOOLSTATE_REPO: https://github.com/rust-lang-nursery/rust-toolstate
  CACHE_DOMAIN: ci-caches.rust-lang.org
  IMAGE: mingw-check
##[endgroup]
fatal: unknown commit 53fd98ca776cb875bc9e5514f56b52eb74f9e7a9
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
configure: 
---
    Checking structopt v0.3.25
error: unused imports: `PathBuf`, `Path`
  --> src/tools/rustfmt/src/ignore_path.rs:35:21
   |
35 |     use std::path::{Path, PathBuf};
   |
   |
   = note: `-D unused-imports` implied by `-D warnings`
error: unused imports: `Config`, `FileName`
  --> src/tools/rustfmt/src/ignore_path.rs:37:25
   |
   |
37 |     use crate::config::{Config, FileName};

error: unused import: `crate::ignore_path::IgnorePathSet`
  --> src/tools/rustfmt/src/ignore_path.rs:38:9
   |
