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
  Stored in directory: /tmp/pip-ephem-wheel-cache-b_wa0t95/wheels/c2/3c/b9/1120c2ab4bd82694f7e6f0537dc5b9a085c13e2c69a8d0c76d
Installing collected packages: boolean-py, binaryornot, setuptools, reuse, python-debian, markupsafe, license-expression, jinja2, chardet
  Attempting uninstall: setuptools
    Found existing installation: setuptools 59.6.0
    Not uninstalling setuptools at /usr/lib/python3/dist-packages, outside environment /usr
---
Successfully tagged rust-ci:latest
Built container sha256:91b3d116d0fc1ae7384d2c9e1ae713bd09168c241c42e5fd42f815cdb39defc3
Uploading finished image to https://ci-caches.rust-lang.org/docker/e51156f19850ce886cec818e46dc2a021e0aa7c270d15673e8fe74cd8522fc8ac3995109aebb688ee49ed586735ed4cf5f8c06d44c48298fa09c35ae4b082281

<botocore.awsrequest.AWSRequest object at 0x7f0dae57c250>
gzip: stdout: Broken pipe
xargs: docker: terminated by signal 13
[CI_JOB_NAME=mingw-check-tidy]
[CI_JOB_NAME=mingw-check-tidy]
---
   Compiling tidy v0.1.0 (/checkout/src/tools/tidy)
    Finished release [optimized] target(s) in 13.18s
##[endgroup]
fmt check
Diff in /checkout/library/std/src/sys/windows/path.rs at line 27:
 pub(crate) fn has_trailing_slash(path: &OsStr) -> bool {
     let is_verbatim = path.as_os_str_bytes().starts_with(br"\\?\");
     let is_separator = if is_verbatim { is_verbatim_sep } else { is_sep_byte };
-    if let Some(&c) = path.as_os_str_bytes().last() {
-        is_separator(c)
-        false
-    }
-    }
+    if let Some(&c) = path.as_os_str_bytes().last() { is_separator(c) } else { false }
 
 
 /// Appends a suffix to a path.
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/rustfmt/bin/rustfmt" "--config-path" "/checkout" "--edition" "2021" "--unstable-features" "--skip-children" "--check" "/checkout/library/std/src/sys/windows/thread_local_dtor.rs" "/checkout/library/std/src/sys/windows/path.rs" "/checkout/library/std/src/sys/windows/thread_parking.rs" "/checkout/library/std/src/sys/windows/io.rs" "/checkout/library/std/src/sys/windows/env.rs" "/checkout/library/std/src/sys/windows/c/windows_sys.rs" "/checkout/library/std/src/sys/windows/stack_overflow.rs" "/checkout/library/std/src/sys/unsupported/locks/mutex.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
