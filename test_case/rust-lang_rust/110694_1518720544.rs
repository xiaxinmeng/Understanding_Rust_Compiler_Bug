plain
Prepare all required actions
Getting action download info
Download action repository 'actions/checkout@v3' (SHA:8e5e7e5ab8b370d6c329ec480221332ada57f0ab)
Download action repository 'rust-lang/simpleinfra@master' (SHA:4d706a8d448f000965ed9e883febfb36661202fe)
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
  Stored in directory: /tmp/pip-ephem-wheel-cache-7sgo3ubr/wheels/c2/3c/b9/1120c2ab4bd82694f7e6f0537dc5b9a085c13e2c69a8d0c76d
Installing collected packages: boolean-py, binaryornot, setuptools, reuse, python-debian, markupsafe, license-expression, jinja2, chardet
  Attempting uninstall: setuptools
    Found existing installation: setuptools 59.6.0
    Not uninstalling setuptools at /usr/lib/python3/dist-packages, outside environment /usr
---
Successfully built 114ddc908d5b
Successfully tagged rust-ci:latest
Built container sha256:114ddc908d5bb323965d814b464b5134c02df024672d06cda39d867bb879d96e
Uploading finished image to https://ci-caches.rust-lang.org/docker/eee721fc1ad06e35509f22d2087fb4acdb0a4d378bec3a191179769e2503f17c7f8f4292d7013976df3a24502e822c590708d80a62f327ca4849564995e4deb7
upload failed: - to s3://rust-lang-ci-sccache2/docker/eee721fc1ad06e35509f22d2087fb4acdb0a4d378bec3a191179769e2503f17c7f8f4292d7013976df3a24502e822c590708d80a62f327ca4849564995e4deb7 Unable to locate credentials
[CI_JOB_NAME=mingw-check-tidy]
[CI_JOB_NAME=mingw-check-tidy]
---
    Finished release [optimized] target(s) in 13.23s
fmt check
tidy check
tidy: Skipping binary file check, read-only filesystem
tidy error: following path contains more than 885 entries, you should move the test to some relevant subdirectory (current: 887): /checkout/tests/ui/parser
Expected a gate test for the feature 'builtin_syntax'.
Hint: create a failing test file named 'feature-gate-builtin_syntax.rs'
      in the 'ui' test suite, with its failures due to
      missing usage of `#![feature(builtin_syntax)]`.
Hint: If you already have such a test and don't want to rename it,
      you can also add a // gate-test-builtin_syntax line to the test file.
tidy error: Found 1 features without a gate test.
tidy error: The stabilization version 1.71.0 of lang feature `builtin_syntax` is written out but should be CURRENT_RUSTC_VERSION
some tidy checks failed
