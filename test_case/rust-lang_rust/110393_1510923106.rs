plain
Prepare all required actions
Getting action download info
Download action repository 'actions/checkout@v3' (SHA:8e5e7e5ab8b370d6c329ec480221332ada57f0ab)
Download action repository 'rust-lang/simpleinfra@master' (SHA:4d706a8d448f000965ed9e883febfb36661202fe)
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
  Stored in directory: /tmp/pip-ephem-wheel-cache-9vf4q7ef/wheels/c2/3c/b9/1120c2ab4bd82694f7e6f0537dc5b9a085c13e2c69a8d0c76d
Installing collected packages: boolean-py, binaryornot, setuptools, reuse, python-debian, markupsafe, license-expression, jinja2, chardet
  Attempting uninstall: setuptools
    Found existing installation: setuptools 59.6.0
    Not uninstalling setuptools at /usr/lib/python3/dist-packages, outside environment /usr
---
Successfully built 18d55e055fae
Successfully tagged rust-ci:latest
Built container sha256:18d55e055faefc313ac680123d484a9f192f2716b3a85c2f8e30d5cc03fe3c8d
Uploading finished image to https://ci-caches.rust-lang.org/docker/5a83b1174027be8472df586f36f1b4e77c39a20882cc84831c17b85c3cc5731b4e272c71c37dd18e09cacea39bf89c1b3226a304723983ec1270656c20627ec9
upload failed: - to s3://rust-lang-ci-sccache2/docker/5a83b1174027be8472df586f36f1b4e77c39a20882cc84831c17b85c3cc5731b4e272c71c37dd18e09cacea39bf89c1b3226a304723983ec1270656c20627ec9 Unable to locate credentials
[CI_JOB_NAME=mingw-check-tidy]
[CI_JOB_NAME=mingw-check-tidy]
---
Building bootstrap
    Finished dev [unoptimized] target(s) in 0.03s
Building tool tidy (stage0)
fmt check
Diff in /checkout/library/core/src/num/mod.rs at line 231:
         fn conv(x: u32) -> $SelfT {
             // FIXME(const-hack) replace with `.try_into().ok().unwrap_unchecked()`.
             // SAFETY: Any legal shift amount must be losslessly representable in the self type.
-            unsafe {
-                x.try_into().ok().unwrap_unchecked()
-            }
+            unsafe { x.try_into().ok().unwrap_unchecked() }
         #[inline]
         #[inline]
         const fn const_conv(x: u32) -> $SelfT {
Diff in /checkout/library/core/src/num/mod.rs at line 241:
 
 
         intrinsics::const_eval_select(($x,), const_conv, conv)
+    }};
 }
 
 impl i8 {
 impl i8 {
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/rustfmt/bin/rustfmt" "--config-path" "/checkout" "--edition" "2021" "--unstable-features" "--skip-children" "--check" "/checkout/library/core/src/prelude/mod.rs" "/checkout/library/core/src/num/bignum.rs" "/checkout/library/core/src/num/int_macros.rs" "/checkout/library/core/src/num/f32.rs" "/checkout/library/core/src/num/error.rs" "/checkout/library/core/src/num/diy_float.rs" "/checkout/library/core/src/num/nonzero.rs" "/checkout/library/core/src/num/mod.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
