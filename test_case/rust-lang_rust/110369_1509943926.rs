plain
Prepare all required actions
Getting action download info
Download action repository 'actions/checkout@v3' (SHA:8e5e7e5ab8b370d6c329ec480221332ada57f0ab)
Download action repository 'rust-lang/simpleinfra@master' (SHA:75291a2cdf0dd5ba60860701274f9500703102da)
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
  Stored in directory: /tmp/pip-ephem-wheel-cache-p5pruett/wheels/c2/3c/b9/1120c2ab4bd82694f7e6f0537dc5b9a085c13e2c69a8d0c76d
Installing collected packages: boolean-py, binaryornot, setuptools, reuse, python-debian, markupsafe, license-expression, jinja2, chardet
  Attempting uninstall: setuptools
    Found existing installation: setuptools 59.6.0
    Not uninstalling setuptools at /usr/lib/python3/dist-packages, outside environment /usr
---
Successfully built 17190f37c7d6
Successfully tagged rust-ci:latest
Built container sha256:17190f37c7d6319945a148437689fddd51efaad229cad84f6f535863997f5a11
Uploading finished image to https://ci-caches.rust-lang.org/docker/5a83b1174027be8472df586f36f1b4e77c39a20882cc84831c17b85c3cc5731b4e272c71c37dd18e09cacea39bf89c1b3226a304723983ec1270656c20627ec9
upload failed: - to s3://rust-lang-ci-sccache2/docker/5a83b1174027be8472df586f36f1b4e77c39a20882cc84831c17b85c3cc5731b4e272c71c37dd18e09cacea39bf89c1b3226a304723983ec1270656c20627ec9 Unable to locate credentials
[CI_JOB_NAME=mingw-check-tidy]
[CI_JOB_NAME=mingw-check-tidy]
---
   Compiling getrandom v0.2.9
   Compiling unicode-normalization v0.1.22
   Compiling gix-sec v0.6.2
   Compiling filetime v0.2.16
   Compiling uluru v3.0.0
   Compiling nix v0.26.2
   Compiling aho-corasick v0.7.18
   Compiling syn v1.0.102
   Compiling imara-diff v0.1.5
---
    Finished release [optimized] target(s) in 14.34s
fmt check
tidy check
tidy: Skipping binary file check, read-only filesystem
tidy error: invalid license `Apache-2.0` in `bytesize 1.2.0 (registry+https://github.com/rust-lang/crates.io-index)`
tidy error: invalid license `CC0-1.0 OR MIT-0` in `dunce 1.0.3 (registry+https://github.com/rust-lang/crates.io-index)`
tidy error: invalid license `Apache-2.0` in `imara-diff 0.1.5 (registry+https://github.com/rust-lang/crates.io-index)`
tidy error: invalid license `BSD-3-Clause` in `instant 0.1.12 (registry+https://github.com/rust-lang/crates.io-index)`
tidy error: invalid license `BSD-3-Clause` in `sha1_smol 1.0.0 (registry+https://github.com/rust-lang/crates.io-index)`
tidy error: invalid license `MPL-2.0` in `uluru 3.0.0 (registry+https://github.com/rust-lang/crates.io-index)`
tidy error: invalid license `Apache-2.0` in `unicode-bom 1.1.4 (registry+https://github.com/rust-lang/crates.io-index)`
some tidy checks failed
