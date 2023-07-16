plain
Prepare all required actions
Getting action download info
Download action repository 'actions/checkout@v3' (SHA:ac593985615ec2ede58e132d2e21d2b1cbd6127c)
Download action repository 'rust-lang/simpleinfra@master' (SHA:75573f9759179a720f4c3af6c9fb518ac0061dca)
Complete job name: PR (mingw-check-tidy, true, ubuntu-20.04-xl)
git config --global core.autocrlf false
shell: /usr/bin/bash --noprofile --norc -e -o pipefail {0}
env:
  CI_JOB_NAME: mingw-check-tidy
---
Building wheels for collected packages: reuse
  Building wheel for reuse (pyproject.toml): started
  Building wheel for reuse (pyproject.toml): finished with status 'done'
  Created wheel for reuse: filename=reuse-1.1.0-cp310-cp310-manylinux_2_35_x86_64.whl size=180117 sha256=a2b5d39c8ff2686626c851b00c3d3ae10157feb2cc6d0d07e414234b479dbb17
  Stored in directory: /tmp/pip-ephem-wheel-cache-_md8vnyz/wheels/c2/3c/b9/1120c2ab4bd82694f7e6f0537dc5b9a085c13e2c69a8d0c76d
Installing collected packages: boolean-py, binaryornot, setuptools, reuse, python-debian, markupsafe, license-expression, jinja2, chardet
  Attempting uninstall: setuptools
    Found existing installation: setuptools 59.6.0
    Not uninstalling setuptools at /usr/lib/python3/dist-packages, outside environment /usr
---
Successfully built 5689ff566e75
Successfully tagged rust-ci:latest
Built container sha256:5689ff566e751b10b084052384e8c323647e42316c4888fed806d3f37bdaf0c3
Uploading finished image to https://ci-caches.rust-lang.org/docker/38eba79578bd6d379f830cfbe873c54dad8d10db1dfcf4fbac8af27e25e93a7ecaa2a7c62028c216d50741db91fde3d04f83489b4d576287b0239d35dee6e197
upload failed: - to s3://rust-lang-ci-sccache2/docker/38eba79578bd6d379f830cfbe873c54dad8d10db1dfcf4fbac8af27e25e93a7ecaa2a7c62028c216d50741db91fde3d04f83489b4d576287b0239d35dee6e197 Unable to locate credentials
[CI_JOB_NAME=mingw-check-tidy]
[CI_JOB_NAME=mingw-check-tidy]
---
tidy check
tidy: Skipping binary file check, read-only filesystem
Found 505 error codes
Highest error code: `E0793`
tidy error: /checkout/src/bootstrap/configure.py:385: trailing whitespace
tidy error: /checkout/src/bootstrap/configure.py:386: trailing whitespace
tidy error: /checkout/src/bootstrap/configure.py:468: trailing whitespace
tidy error: /checkout/src/bootstrap/configure.py:469: trailing whitespace
tidy error: /checkout/src/bootstrap/configure.py:470: trailing whitespace
tidy error: /checkout/src/bootstrap/configure.py:472: trailing whitespace
tidy error: /checkout/src/bootstrap/configure.py:473: trailing whitespace
tidy error: /checkout/src/bootstrap/configure.py:476: trailing whitespace
tidy error: /checkout/src/bootstrap/configure.py:478: trailing whitespace
tidy error: /checkout/src/bootstrap/configure.py:480: trailing whitespace
tidy error: /checkout/src/bootstrap/configure.py:481: trailing whitespace
some tidy checks failed
Build completed unsuccessfully in 0:00:26
