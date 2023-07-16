plain
Prepare all required actions
Getting action download info
Download action repository 'actions/checkout@v3' (SHA:8e5e7e5ab8b370d6c329ec480221332ada57f0ab)
Download action repository 'rust-lang/simpleinfra@master' (SHA:c6b1c5004fc2af7b3d08a3da4b781692c43b8b5a)
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
  Stored in directory: /tmp/pip-ephem-wheel-cache-dk5__17j/wheels/c2/3c/b9/1120c2ab4bd82694f7e6f0537dc5b9a085c13e2c69a8d0c76d
Installing collected packages: boolean-py, binaryornot, setuptools, reuse, python-debian, markupsafe, license-expression, jinja2, chardet
  Attempting uninstall: setuptools
    Found existing installation: setuptools 59.6.0
    Not uninstalling setuptools at /usr/lib/python3/dist-packages, outside environment /usr
---
Successfully built 5ea9160cc91c
Successfully tagged rust-ci:latest
Built container sha256:5ea9160cc91ce200cc0c43833785fbdc25e0535dcd7bf1670d97f9b5038d93e5
Uploading finished image to https://ci-caches.rust-lang.org/docker/eee721fc1ad06e35509f22d2087fb4acdb0a4d378bec3a191179769e2503f17c7f8f4292d7013976df3a24502e822c590708d80a62f327ca4849564995e4deb7
upload failed: - to s3://rust-lang-ci-sccache2/docker/eee721fc1ad06e35509f22d2087fb4acdb0a4d378bec3a191179769e2503f17c7f8f4292d7013976df3a24502e822c590708d80a62f327ca4849564995e4deb7 Unable to locate credentials
[CI_JOB_NAME=mingw-check-tidy]
[CI_JOB_NAME=mingw-check-tidy]
---
##[endgroup]
fmt check
Diff in /checkout/library/std/src/fs.rs at line 2282:
 ///
 /// # Errors
 ///
-/// See [`fs::remove_file`] and [`fs::remove_dir`]. 
+/// See [`fs::remove_file`] and [`fs::remove_dir`].
 ///
-/// `remove_dir_all` will fail if `remove_dir` or `remove_file` fail on any constituent paths, including the root path. 
-/// As a result, the directory you are deleting must exist, meaning that this function is not idempotent. 
+/// `remove_dir_all` will fail if `remove_dir` or `remove_file` fail on any constituent paths, including the root path.
+/// As a result, the directory you are deleting must exist, meaning that this function is not idempotent.
 ///
 /// Consider ignoring the error if validating the removal is not required for your use case.
 ///
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/rustfmt/bin/rustfmt" "--config-path" "/checkout" "--edition" "2021" "--unstable-features" "--skip-children" "--check" "/checkout/library/std/src/thread/mod.rs" "/checkout/library/std/src/personality.rs" "/checkout/library/std/src/personality/emcc.rs" "/checkout/library/std/src/f64.rs" "/checkout/library/std/src/backtrace.rs" "/checkout/library/std/src/fs.rs" "/checkout/library/std/src/f32/tests.rs" "/checkout/library/std/src/panicking.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
