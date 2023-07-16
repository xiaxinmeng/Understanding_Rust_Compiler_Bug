plain
Prepare all required actions
Getting action download info
Download action repository 'actions/checkout@v3' (SHA:8e5e7e5ab8b370d6c329ec480221332ada57f0ab)
Download action repository 'rust-lang/simpleinfra@master' (SHA:75291a2cdf0dd5ba60860701274f9500703102da)
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
  Stored in directory: /tmp/pip-ephem-wheel-cache-ahz7kykc/wheels/c2/3c/b9/1120c2ab4bd82694f7e6f0537dc5b9a085c13e2c69a8d0c76d
Installing collected packages: boolean-py, binaryornot, setuptools, reuse, python-debian, markupsafe, license-expression, jinja2, chardet
  Attempting uninstall: setuptools
    Found existing installation: setuptools 59.6.0
    Not uninstalling setuptools at /usr/lib/python3/dist-packages, outside environment /usr
---
Successfully built b7cb36fcd6b0
Successfully tagged rust-ci:latest
Built container sha256:b7cb36fcd6b0be20370777063abe385d4a3428a0a5adcda0b9e5efeda30b0564
Uploading finished image to https://ci-caches.rust-lang.org/docker/5a83b1174027be8472df586f36f1b4e77c39a20882cc84831c17b85c3cc5731b4e272c71c37dd18e09cacea39bf89c1b3226a304723983ec1270656c20627ec9
upload failed: - to s3://rust-lang-ci-sccache2/docker/5a83b1174027be8472df586f36f1b4e77c39a20882cc84831c17b85c3cc5731b4e272c71c37dd18e09cacea39bf89c1b3226a304723983ec1270656c20627ec9 Unable to locate credentials
[CI_JOB_NAME=mingw-check-tidy]
[CI_JOB_NAME=mingw-check-tidy]
---
   Compiling cargo_metadata v0.15.3
   Compiling tidy v0.1.0 (/checkout/src/tools/tidy)
    Finished release [optimized] target(s) in 13.38s
fmt check
Diff in /checkout/library/core/src/num/flt2dec/strategy/grisu.rs at line 164:
 pub fn pow10_up_to_9(n: usize) -> u32 {
     debug_assert!(n <= 9);
 
-    const POW10: [u32; 10] = [
-        10,
-        100,
-        1000,
-        10_000,
-        10_000,
-        100_000,
-        1_000_000,
-        10_000_000,
-        100_000_000,
-        1_000_000_000,
-    ];
+    const POW10: [u32; 10] =
+        [1, 10, 100, 1000, 10_000, 100_000, 1_000_000, 10_000_000, 100_000_000, 1_000_000_000];
 
     POW10[n as usize]
Build completed unsuccessfully in 0:00:14
Build completed unsuccessfully in 0:00:14
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/rustfmt/bin/rustfmt" "--config-path" "/checkout" "--edition" "2021" "--unstable-features" "--skip-children" "--check" "/checkout/library/test/src/cli.rs" "/checkout/library/core/src/num/flt2dec/mod.rs" "/checkout/library/core/src/num/flt2dec/estimator.rs" "/checkout/library/core/src/num/flt2dec/strategy/dragon.rs" "/checkout/library/core/src/num/flt2dec/strategy/grisu.rs" "/checkout/library/core/src/num/flt2dec/decoder.rs" "/checkout/library/core/src/num/bignum.rs" "/checkout/library/core/src/num/mod.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
