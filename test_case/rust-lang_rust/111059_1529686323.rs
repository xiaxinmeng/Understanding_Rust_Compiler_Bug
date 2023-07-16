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
  Stored in directory: /tmp/pip-ephem-wheel-cache-j8kwzqm0/wheels/c2/3c/b9/1120c2ab4bd82694f7e6f0537dc5b9a085c13e2c69a8d0c76d
Installing collected packages: boolean-py, binaryornot, setuptools, reuse, python-debian, markupsafe, license-expression, jinja2, chardet
  Attempting uninstall: setuptools
    Found existing installation: setuptools 59.6.0
    Not uninstalling setuptools at /usr/lib/python3/dist-packages, outside environment /usr
---
Successfully built e85fce956eae
Successfully tagged rust-ci:latest
Built container sha256:e85fce956eae159a7f2ba59aecc36e041672633660deb6bd86cf00496c3a3b13
Uploading finished image to https://ci-caches.rust-lang.org/docker/e51156f19850ce886cec818e46dc2a021e0aa7c270d15673e8fe74cd8522fc8ac3995109aebb688ee49ed586735ed4cf5f8c06d44c48298fa09c35ae4b082281
upload failed: - to s3://rust-lang-ci-sccache2/docker/e51156f19850ce886cec818e46dc2a021e0aa7c270d15673e8fe74cd8522fc8ac3995109aebb688ee49ed586735ed4cf5f8c06d44c48298fa09c35ae4b082281 Unable to locate credentials
[CI_JOB_NAME=mingw-check-tidy]
[CI_JOB_NAME=mingw-check-tidy]
---
   Compiling tidy v0.1.0 (/checkout/src/tools/tidy)
    Finished release [optimized] target(s) in 12.73s
##[endgroup]
fmt check
Diff in /checkout/library/std/src/ffi/os_str.rs at line 8:
 use crate::hash::{Hash, Hasher};
 use crate::ops;
 use crate::rc::Rc;
-use crate::str::FromStr;
 use crate::str::pattern::Pattern;
+use crate::str::FromStr;
 use crate::sync::Arc;
 
 use crate::sys::os_str::{Buf, Slice};
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/rustfmt/bin/rustfmt" "--config-path" "/checkout" "--edition" "2021" "--unstable-features" "--skip-children" "--check" "/checkout/library/std/src/os/vxworks/fs.rs" "/checkout/library/std/src/os/nto/mod.rs" "/checkout/library/std/src/os/nto/fs.rs" "/checkout/library/std/src/os/vxworks/mod.rs" "/checkout/library/std/src/ffi/os_str.rs" "/checkout/library/std/src/os/vxworks/raw.rs" "/checkout/library/std/src/os/illumos/raw.rs" "/checkout/library/std/src/os/nto/raw.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
Build completed unsuccessfully in 0:00:22
Diff in /checkout/library/std/src/sys_common/wtf8.rs at line 31:
 use crate::rc::Rc;
 use crate::slice;
 use crate::str;
-use crate::str::pattern::{Pattern, Searcher, SearchStep};
+use crate::str::pattern::{Pattern, SearchStep, Searcher};
 use crate::sync::Arc;
 use crate::sys_common::AsInner;
