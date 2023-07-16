plain
Prepare all required actions
Getting action download info
Download action repository 'actions/checkout@v3' (SHA:8e5e7e5ab8b370d6c329ec480221332ada57f0ab)
Download action repository 'rust-lang/simpleinfra@master' (SHA:a244d6aab06843e1f4ae41ff60c7091ad809a20e)
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
  Stored in directory: /tmp/pip-ephem-wheel-cache-iha05zio/wheels/c2/3c/b9/1120c2ab4bd82694f7e6f0537dc5b9a085c13e2c69a8d0c76d
Installing collected packages: boolean-py, binaryornot, setuptools, reuse, python-debian, markupsafe, license-expression, jinja2, chardet
  Attempting uninstall: setuptools
    Found existing installation: setuptools 59.6.0
    Not uninstalling setuptools at /usr/lib/python3/dist-packages, outside environment /usr
---
Successfully built aacc16091b22
Successfully tagged rust-ci:latest
Built container sha256:aacc16091b221f07df5bb7463ed2f6476d25bbca8f81098b75a86a14c6ae6001
Uploading finished image to https://ci-caches.rust-lang.org/docker/e51156f19850ce886cec818e46dc2a021e0aa7c270d15673e8fe74cd8522fc8ac3995109aebb688ee49ed586735ed4cf5f8c06d44c48298fa09c35ae4b082281
upload failed: - to s3://rust-lang-ci-sccache2/docker/e51156f19850ce886cec818e46dc2a021e0aa7c270d15673e8fe74cd8522fc8ac3995109aebb688ee49ed586735ed4cf5f8c06d44c48298fa09c35ae4b082281 Unable to locate credentials
[CI_JOB_NAME=mingw-check-tidy]
[CI_JOB_NAME=mingw-check-tidy]
---
   Compiling tidy v0.1.0 (/checkout/src/tools/tidy)
    Finished release [optimized] target(s) in 13.10s
##[endgroup]
fmt check
Diff in /checkout/library/std/src/sys/unix/kernel_copy.rs at line 68:
 mod tests;
 
 
-pub(crate) fn copy_spec<R: Read, W: Write>(
-    read: R,
-    write: W,
-) -> Result<u64> {
+pub(crate) fn copy_spec<R: Read, W: Write>(read: R, write: W) -> Result<u64> {
     let copier = Copier { read, write };
     SpecCopy::copy(copier)
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/rustfmt/bin/rustfmt" "--config-path" "/checkout" "--edition" "2021" "--unstable-features" "--skip-children" "--check" "/checkout/library/std/src/sys/unix/kernel_copy.rs" "/checkout/library/std/src/sys/unix/os_str/tests.rs" "/checkout/library/std/src/sys/unix/path.rs" "/checkout/library/std/src/sys/unix/io.rs" "/checkout/library/std/src/sys/unix/futex.rs" "/checkout/library/std/src/sys/unix/env.rs" "/checkout/library/std/src/sys/unix/stack_overflow.rs" "/checkout/library/std/src/sys/itron/mutex.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
Build completed unsuccessfully in 0:00:24
