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
  Stored in directory: /tmp/pip-ephem-wheel-cache-fxv48_8d/wheels/c2/3c/b9/1120c2ab4bd82694f7e6f0537dc5b9a085c13e2c69a8d0c76d
Installing collected packages: boolean-py, binaryornot, setuptools, reuse, python-debian, markupsafe, license-expression, jinja2, chardet
  Attempting uninstall: setuptools
    Found existing installation: setuptools 59.6.0
    Not uninstalling setuptools at /usr/lib/python3/dist-packages, outside environment /usr
---
Successfully built 7560bad5c83a
Successfully tagged rust-ci:latest
Built container sha256:7560bad5c83ac81105a7f6a80d50f88a791d9edc0505d9a9142bb4dc1c0510bd
Uploading finished image to https://ci-caches.rust-lang.org/docker/5a83b1174027be8472df586f36f1b4e77c39a20882cc84831c17b85c3cc5731b4e272c71c37dd18e09cacea39bf89c1b3226a304723983ec1270656c20627ec9
upload failed: - to s3://rust-lang-ci-sccache2/docker/5a83b1174027be8472df586f36f1b4e77c39a20882cc84831c17b85c3cc5731b4e272c71c37dd18e09cacea39bf89c1b3226a304723983ec1270656c20627ec9 Unable to locate credentials
[CI_JOB_NAME=mingw-check-tidy]
[CI_JOB_NAME=mingw-check-tidy]
---
    Finished release [optimized] target(s) in 12.93s
fmt check
Diff in /checkout/library/std/src/collections/mod.rs at line 252:
 //!
 //! Several other collection methods also return iterators to yield a sequence
 //! of results but avoid allocating an entire collection to store the result in.
-//! This provides maximum flexibility as `collect` or 
-//! [`extend`](https://doc.rust-lang.org/stable/std/iter/trait.Extend.html) 
+//! This provides maximum flexibility as `collect` or
+//! [`extend`](https://doc.rust-lang.org/stable/std/iter/trait.Extend.html)
 //! can be called to
 //! "pipe" the sequence into any collection if desired. Otherwise, the sequence
 //! can be looped over with a `for` loop. The iterator can also be discarded
