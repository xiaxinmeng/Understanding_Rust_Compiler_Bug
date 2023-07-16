plain
Prepare all required actions
Getting action download info
Download action repository 'actions/checkout@v3' (SHA:ac593985615ec2ede58e132d2e21d2b1cbd6127c)
Download action repository 'rust-lang/simpleinfra@master' (SHA:7ea263296c702710d7fac3c6d5bccdb2895b4e79)
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
  Stored in directory: /tmp/pip-ephem-wheel-cache-upuq83tj/wheels/c2/3c/b9/1120c2ab4bd82694f7e6f0537dc5b9a085c13e2c69a8d0c76d
Installing collected packages: boolean-py, binaryornot, setuptools, reuse, python-debian, markupsafe, license-expression, jinja2, chardet
  Attempting uninstall: setuptools
    Found existing installation: setuptools 59.6.0
    Not uninstalling setuptools at /usr/lib/python3/dist-packages, outside environment /usr
---
Successfully built 91ca3f3de7c5
Successfully tagged rust-ci:latest
Built container sha256:91ca3f3de7c5635e55545826d5bce762a15ecc568b8a7efcd943b53089e2495c
Uploading finished image to https://ci-caches.rust-lang.org/docker/f36b5782f7228aca51ac4b42fc8d834a0357048eedd2f8ec11ebf729b3642b4f40d12a04648dc10cace4cb151f5ecc2ac820f64f301d3380602cceb15ce3c48d
upload failed: - to s3://rust-lang-ci-sccache2/docker/f36b5782f7228aca51ac4b42fc8d834a0357048eedd2f8ec11ebf729b3642b4f40d12a04648dc10cace4cb151f5ecc2ac820f64f301d3380602cceb15ce3c48d Unable to locate credentials
[CI_JOB_NAME=mingw-check-tidy]
[CI_JOB_NAME=mingw-check-tidy]
---
tidy check
tidy: Skipping binary file check, read-only filesystem
Found 505 error codes
Highest error code: `E0793`
tidy error: following path contains more than 940 entries, you should move the test to some relevant subdirectory (current: 941): /checkout/tests/ui
some tidy checks failed
Build completed unsuccessfully in 0:00:22
