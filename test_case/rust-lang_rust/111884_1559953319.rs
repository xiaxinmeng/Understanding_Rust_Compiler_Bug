plain
Prepare all required actions
Getting action download info
Download action repository 'actions/checkout@v3' (SHA:8e5e7e5ab8b370d6c329ec480221332ada57f0ab)
Download action repository 'rust-lang/simpleinfra@master' (SHA:cdb59432ef9e018dd6db5879e720eccc3289da68)
Complete job name: PR - mingw-check-tidy
git config --global core.autocrlf false
shell: /usr/bin/bash --noprofile --norc -e -o pipefail {0}
env:
  CI_JOB_NAME: mingw-check-tidy
---
Building wheels for collected packages: reuse
  Building wheel for reuse (pyproject.toml): started
  Building wheel for reuse (pyproject.toml): finished with status 'done'
  Created wheel for reuse: filename=reuse-1.1.0-cp310-cp310-manylinux_2_35_x86_64.whl size=180115 sha256=c3e1cf75e5aea7348ad201d16633573bc9f3aa58c8e29a6ca459f306300d6c9d
  Stored in directory: /tmp/pip-ephem-wheel-cache-nyr73mss/wheels/c2/3c/b9/1120c2ab4bd82694f7e6f0537dc5b9a085c13e2c69a8d0c76d
Installing collected packages: boolean-py, binaryornot, setuptools, reuse, python-debian, markupsafe, license-expression, jinja2, chardet
  Attempting uninstall: setuptools
    Found existing installation: setuptools 59.6.0
    Not uninstalling setuptools at /usr/lib/python3/dist-packages, outside environment /usr
---
Successfully tagged rust-ci:latest
Built container sha256:66828594d992165f75d0141303f4a3cfc1b3589e4bdbadd6baca80193955fff4
Uploading finished image to https://ci-caches.rust-lang.org/docker/e51156f19850ce886cec818e46dc2a021e0aa7c270d15673e8fe74cd8522fc8ac3995109aebb688ee49ed586735ed4cf5f8c06d44c48298fa09c35ae4b082281

<botocore.awsrequest.AWSRequest object at 0x7fe82040ff10>
gzip: stdout: Broken pipe
xargs: docker: terminated by signal 13
[CI_JOB_NAME=mingw-check-tidy]
[CI_JOB_NAME=mingw-check-tidy]
---
   Compiling tidy v0.1.0 (/checkout/src/tools/tidy)
    Finished release [optimized] target(s) in 13.31s
##[endgroup]
fmt check
Diff in /checkout/library/core/src/convert/mod.rs at line 30:
 //! For example, [`From`]/[`TryFrom`] implementations exist for string to string, list to list,
 //! number to number, or error to error conversions. However, implementing `From<i32>` for
 //! `String` or `TryFrom<&str>` for `i32` would not be appropriate.
+
+
 //! # Generic Implementations
 //!
 //! - [`AsRef`] and [`AsMut`] auto-dereference if the inner type is a reference
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/rustfmt/bin/rustfmt" "--config-path" "/checkout" "--edition" "2021" "--unstable-features" "--skip-children" "--check" "/checkout/library/core/src/escape.rs" "/checkout/library/core/src/convert/num.rs" "/checkout/library/core/src/option.rs" "/checkout/library/core/src/intrinsics.rs" "/checkout/library/core/src/convert/mod.rs" "/checkout/library/core/src/clone.rs" "/checkout/library/core/src/bool.rs" "/checkout/library/core/src/array/ascii.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
