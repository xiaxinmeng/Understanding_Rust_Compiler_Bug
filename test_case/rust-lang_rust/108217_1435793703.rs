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
  Stored in directory: /tmp/pip-ephem-wheel-cache-k4fo_dtv/wheels/c2/3c/b9/1120c2ab4bd82694f7e6f0537dc5b9a085c13e2c69a8d0c76d
Installing collected packages: boolean-py, binaryornot, setuptools, reuse, python-debian, markupsafe, license-expression, jinja2, chardet
  Attempting uninstall: setuptools
    Found existing installation: setuptools 59.6.0
    Not uninstalling setuptools at /usr/lib/python3/dist-packages, outside environment /usr
---
Successfully built b5e72b566c4f
Successfully tagged rust-ci:latest
Built container sha256:b5e72b566c4f423e6d13f90c9357ece14e73b668cd07e5155901f3616a02aac1
Uploading finished image to https://ci-caches.rust-lang.org/docker/f36b5782f7228aca51ac4b42fc8d834a0357048eedd2f8ec11ebf729b3642b4f40d12a04648dc10cace4cb151f5ecc2ac820f64f301d3380602cceb15ce3c48d
upload failed: - to s3://rust-lang-ci-sccache2/docker/f36b5782f7228aca51ac4b42fc8d834a0357048eedd2f8ec11ebf729b3642b4f40d12a04648dc10cace4cb151f5ecc2ac820f64f301d3380602cceb15ce3c48d Unable to locate credentials
[CI_JOB_NAME=mingw-check-tidy]
[CI_JOB_NAME=mingw-check-tidy]
---
   Compiling tidy v0.1.0 (/checkout/src/tools/tidy)
    Finished release [optimized] target(s) in 18.63s
fmt check
skip untracked path cpu-usage.csv during rustfmt invocations
Diff in /checkout/library/core/src/str/validations.rs at line 146:
                     Some(&byte) => byte,
                     None => err!(None),
-            }
+            };
         }
         macro_rules! check_cont_byte {
         macro_rules! check_cont_byte {
             ($off:expr) => {
Diff in /checkout/library/core/src/str/validations.rs at line 153:
                 if !utf8_is_cont_byte(get_byte!($off)) {
                     err!(Some($off as u8))
-            }
+            };
         }
 
 
         if first >= 128 {
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/rustfmt/bin/rustfmt" "--config-path" "/checkout" "--edition" "2021" "--unstable-features" "--skip-children" "--check" "/checkout/library/core/src/str/pattern.rs" "/checkout/library/core/src/str/traits.rs" "/checkout/library/core/src/str/validations.rs" "/checkout/library/core/src/str/count.rs" "/checkout/library/core/src/mem/mod.rs" "/checkout/library/core/src/ops/try_trait.rs" "/checkout/library/core/src/async_iter/async_iter.rs" "/checkout/library/core/src/ops/range.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
