plain
Prepare all required actions
Getting action download info
Download action repository 'actions/checkout@v3' (SHA:8e5e7e5ab8b370d6c329ec480221332ada57f0ab)
Download action repository 'rust-lang/simpleinfra@master' (SHA:af9a3f1b7e12a54c737d8aa371acc8d05cb83a8f)
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
  Stored in directory: /tmp/pip-ephem-wheel-cache-x7hssf16/wheels/c2/3c/b9/1120c2ab4bd82694f7e6f0537dc5b9a085c13e2c69a8d0c76d
Installing collected packages: boolean-py, binaryornot, setuptools, reuse, python-debian, markupsafe, license-expression, jinja2, chardet
  Attempting uninstall: setuptools
    Found existing installation: setuptools 59.6.0
    Not uninstalling setuptools at /usr/lib/python3/dist-packages, outside environment /usr
---
Successfully built acc351a6bb08
Successfully tagged rust-ci:latest
Built container sha256:acc351a6bb0888b62b9091a34eb7cbaa3d0cfe4b08668039d20d325cf091c9ad
Uploading finished image to https://ci-caches.rust-lang.org/docker/e51156f19850ce886cec818e46dc2a021e0aa7c270d15673e8fe74cd8522fc8ac3995109aebb688ee49ed586735ed4cf5f8c06d44c48298fa09c35ae4b082281
upload failed: - to s3://rust-lang-ci-sccache2/docker/e51156f19850ce886cec818e46dc2a021e0aa7c270d15673e8fe74cd8522fc8ac3995109aebb688ee49ed586735ed4cf5f8c06d44c48298fa09c35ae4b082281 Unable to locate credentials
[CI_JOB_NAME=mingw-check-tidy]
[CI_JOB_NAME=mingw-check-tidy]
---
##[endgroup]
fmt check
tidy check
tidy: Skipping binary file check, read-only filesystem
tidy error: Dependency for rustc not explicitly permitted: windows_i686_gnu 0.48.0 (registry+https://github.com/rust-lang/crates.io-index)
tidy error: Dependency for rustc not explicitly permitted: iana-time-zone-haiku 0.1.1 (registry+https://github.com/rust-lang/crates.io-index)
tidy error: Dependency for rustc not explicitly permitted: windows_x86_64_msvc 0.48.0 (registry+https://github.com/rust-lang/crates.io-index)
tidy error: Dependency for rustc not explicitly permitted: iana-time-zone 0.1.56 (registry+https://github.com/rust-lang/crates.io-index)
tidy error: Dependency for rustc not explicitly permitted: windows_aarch64_msvc 0.48.0 (registry+https://github.com/rust-lang/crates.io-index)
tidy error: Dependency for rustc not explicitly permitted: scratch 1.0.5 (registry+https://github.com/rust-lang/crates.io-index)
tidy error: Dependency for rustc not explicitly permitted: windows_aarch64_gnullvm 0.48.0 (registry+https://github.com/rust-lang/crates.io-index)
tidy error: Dependency for rustc not explicitly permitted: windows_i686_msvc 0.48.0 (registry+https://github.com/rust-lang/crates.io-index)
tidy error: Dependency for rustc not explicitly permitted: cxx 1.0.94 (registry+https://github.com/rust-lang/crates.io-index)
tidy error: Dependency for rustc not explicitly permitted: windows_x86_64_gnu 0.48.0 (registry+https://github.com/rust-lang/crates.io-index)
tidy error: Dependency for rustc not explicitly permitted: link-cplusplus 1.0.8 (registry+https://github.com/rust-lang/crates.io-index)
tidy error: Dependency for rustc not explicitly permitted: windows_x86_64_gnullvm 0.48.0 (registry+https://github.com/rust-lang/crates.io-index)
tidy error: Dependency for rustc not explicitly permitted: android_system_properties 0.1.5 (registry+https://github.com/rust-lang/crates.io-index)
tidy error: Dependency for rustc not explicitly permitted: cxx-build 1.0.94 (registry+https://github.com/rust-lang/crates.io-index)
tidy error: Dependency for rustc not explicitly permitted: core-foundation-sys 0.8.4 (registry+https://github.com/rust-lang/crates.io-index)
tidy error: Dependency for rustc not explicitly permitted: cxxbridge-macro 1.0.94 (registry+https://github.com/rust-lang/crates.io-index)
tidy error: Dependency for rustc not explicitly permitted: codespan-reporting 0.11.1 (registry+https://github.com/rust-lang/crates.io-index)
tidy error: Dependency for rustc not explicitly permitted: windows-targets 0.48.0 (registry+https://github.com/rust-lang/crates.io-index)
tidy error: Dependency for rustc not explicitly permitted: cxxbridge-flags 1.0.94 (registry+https://github.com/rust-lang/crates.io-index)
some tidy checks failed
