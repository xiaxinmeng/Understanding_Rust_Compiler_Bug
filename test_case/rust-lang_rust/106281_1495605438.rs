plain
Prepare all required actions
Getting action download info
Download action repository 'actions/checkout@v3' (SHA:8f4b7f84864484a7bf31766abe9204da3cbe65b3)
Download action repository 'rust-lang/simpleinfra@master' (SHA:976c0fd7f953d1b34bea4ff6f03b3baa6d7a705f)
Complete job name: PR - mingw-check-tidy
git config --global core.autocrlf false
shell: /usr/bin/bash --noprofile --norc -e -o pipefail {0}
env:
  CI_JOB_NAME: mingw-check-tidy
---
Building wheels for collected packages: reuse
  Building wheel for reuse (pyproject.toml): started
  Building wheel for reuse (pyproject.toml): finished with status 'done'
  Created wheel for reuse: filename=reuse-1.1.0-cp310-cp310-manylinux_2_35_x86_64.whl size=180119 sha256=9fa76c45f3193f307e02ea67d1a48cfe7ef2b854a074b766938a88e046bc7887
  Stored in directory: /tmp/pip-ephem-wheel-cache-25eb7amn/wheels/c2/3c/b9/1120c2ab4bd82694f7e6f0537dc5b9a085c13e2c69a8d0c76d
Installing collected packages: boolean-py, binaryornot, setuptools, reuse, python-debian, markupsafe, license-expression, jinja2, chardet
  Attempting uninstall: setuptools
    Found existing installation: setuptools 59.6.0
    Not uninstalling setuptools at /usr/lib/python3/dist-packages, outside environment /usr
---
Successfully built aa854368ec43
Successfully tagged rust-ci:latest
Built container sha256:aa854368ec4310cec655280ffa668baf2ba39460340d4e0f912ef26bb2f1cf82
Uploading finished image to https://ci-caches.rust-lang.org/docker/a9e2a06b4251eb289588a369c7f56080c669ec63e1f1f16574e1f51b4f34f3159d382fb12e55d549e0394ca3b5e0576095d2a8ec7e3d06d45cebdc807e9f0dd8
upload failed: - to s3://rust-lang-ci-sccache2/docker/a9e2a06b4251eb289588a369c7f56080c669ec63e1f1f16574e1f51b4f34f3159d382fb12e55d549e0394ca3b5e0576095d2a8ec7e3d06d45cebdc807e9f0dd8 Unable to locate credentials
[CI_JOB_NAME=mingw-check-tidy]
[CI_JOB_NAME=mingw-check-tidy]
---
    Finished release [optimized] target(s) in 12.79s
fmt check
tidy check
tidy: Skipping binary file check, read-only filesystem
##[error]tidy error: /checkout/compiler/rustc_feature/src/active.rs:521: failed to parse since: 1.71. (ParseIntError(ParseIntError { kind: Empty }))
##[error]tidy error: /checkout/compiler/rustc_feature/src/active.rs:521: no tracking issue for feature transmute_generic_consts
Expected a gate test for the feature 'transmute_generic_consts'.
Hint: create a failing test file named 'feature-gate-transmute_generic_consts.rs'
      in the 'ui' test suite, with its failures due to
      missing usage of `#![feature(transmute_generic_consts)]`.
Hint: If you already have such a test and don't want to rename it,
      you can also add a // gate-test-transmute_generic_consts line to the test file.
tidy error: Found 1 features without a gate test.
some tidy checks failed
