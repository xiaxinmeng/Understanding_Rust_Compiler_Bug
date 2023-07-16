plain
Prepare all required actions
Getting action download info
Download action repository 'actions/checkout@v3' (SHA:ac593985615ec2ede58e132d2e21d2b1cbd6127c)
Download action repository 'rust-lang/simpleinfra@master' (SHA:e58f560dd5e72b61cf9aeebf432d862b23ac76a8)
Complete job name: PR (mingw-check-tidy, true, ubuntu-20.04-xl)
git config --global core.autocrlf false
shell: /usr/bin/bash --noprofile --norc -e -o pipefail {0}
env:
  CI_JOB_NAME: mingw-check-tidy
---
Building wheels for collected packages: reuse
  Building wheel for reuse (pyproject.toml): started
  Building wheel for reuse (pyproject.toml): finished with status 'done'
  Created wheel for reuse: filename=reuse-1.1.0-cp310-cp310-manylinux_2_35_x86_64.whl size=180118 sha256=2f37b3a1d0677cd3228b6c9d3baf1ad4a6f9d44f04404765e8544fc47767cbcc
  Stored in directory: /tmp/pip-ephem-wheel-cache-emjbt54_/wheels/c2/3c/b9/1120c2ab4bd82694f7e6f0537dc5b9a085c13e2c69a8d0c76d
Installing collected packages: boolean-py, binaryornot, setuptools, reuse, python-debian, markupsafe, license-expression, jinja2, chardet
  Attempting uninstall: setuptools
    Found existing installation: setuptools 59.6.0
    Not uninstalling setuptools at /usr/lib/python3/dist-packages, outside environment /usr
---
Successfully built 56a77cfc83e3
Successfully tagged rust-ci:latest
Built container sha256:56a77cfc83e362a08a207427385c1c7ada08871cf4fcc971204059e316a86cd1
Uploading finished image to https://ci-caches.rust-lang.org/docker/38eba79578bd6d379f830cfbe873c54dad8d10db1dfcf4fbac8af27e25e93a7ecaa2a7c62028c216d50741db91fde3d04f83489b4d576287b0239d35dee6e197
upload failed: - to s3://rust-lang-ci-sccache2/docker/38eba79578bd6d379f830cfbe873c54dad8d10db1dfcf4fbac8af27e25e93a7ecaa2a7c62028c216d50741db91fde3d04f83489b4d576287b0239d35dee6e197 Unable to locate credentials
[CI_JOB_NAME=mingw-check-tidy]
[CI_JOB_NAME=mingw-check-tidy]
---
   Compiling tidy v0.1.0 (/checkout/src/tools/tidy)
    Finished release [optimized] target(s) in 18.77s
fmt check
skip untracked path cpu-usage.csv during rustfmt invocations
Diff in /checkout/compiler/rustc_middle/src/mir/interpret/allocation.rs at line 42:
 
     /// Create a zeroed `AllocBytes` of the specified size and alignment;
     /// call the callback error handler if there is an error in allocating the memory.
-    fn zeroed(
-        _align: Align,
-    ) -> Option<Self>;
-    ) -> Option<Self>;
+    fn zeroed(size: Size, _align: Align) -> Option<Self>;
 
 
 // Default `bytes` for `Allocation` is a `Box<[u8]>`.
Diff in /checkout/compiler/rustc_middle/src/mir/interpret/allocation.rs at line 58:
         Box::<[u8]>::from(slice.into())
 
-    fn zeroed(
-        size: Size,
-        _align: Align,
-        _align: Align,
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/rustfmt/bin/rustfmt" "--config-path" "/checkout" "--edition" "2021" "--unstable-features" "--skip-children" "--check" "/checkout/compiler/rustc_builtin_macros/src/cfg_eval.rs" "/checkout/compiler/rustc_middle/src/mir/interpret/allocation/tests.rs" "/checkout/compiler/rustc_middle/src/mir/interpret/allocation/provenance_map.rs" "/checkout/compiler/rustc_middle/src/mir/interpret/allocation/init_mask.rs" "/checkout/compiler/rustc_middle/src/mir/interpret/error.rs" "/checkout/compiler/rustc_middle/src/mir/interpret/mod.rs" "/checkout/compiler/rustc_middle/src/mir/interpret/allocation.rs" "/checkout/compiler/rustc_lint/src/internal.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
-    ) -> Option<Self> {
+    fn zeroed(size: Size, _align: Align) -> Option<Self> {
         let bytes = Box::<[u8]>::try_new_zeroed_slice(size.bytes_usize()).ok()?;
         // SAFETY: the box was zero-allocated, which is a valid initial value for Box<[u8]>
         let bytes = unsafe { bytes.assume_init() };
