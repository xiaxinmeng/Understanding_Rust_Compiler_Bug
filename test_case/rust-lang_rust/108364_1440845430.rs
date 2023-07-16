plain
Prepare all required actions
Getting action download info
Download action repository 'actions/checkout@v3' (SHA:ac593985615ec2ede58e132d2e21d2b1cbd6127c)
Download action repository 'rust-lang/simpleinfra@master' (SHA:697bea7ddceb6696743da8f159f268aef8bfb3c6)
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
  Stored in directory: /tmp/pip-ephem-wheel-cache-mr1ncwc9/wheels/c2/3c/b9/1120c2ab4bd82694f7e6f0537dc5b9a085c13e2c69a8d0c76d
Installing collected packages: boolean-py, binaryornot, setuptools, reuse, python-debian, markupsafe, license-expression, jinja2, chardet
  Attempting uninstall: setuptools
    Found existing installation: setuptools 59.6.0
    Not uninstalling setuptools at /usr/lib/python3/dist-packages, outside environment /usr
---
Successfully built 863a7bde2cc7
Successfully tagged rust-ci:latest
Built container sha256:863a7bde2cc7c9569d05c14d6a8ad7b49d6f6848a8490c8e18a88301984c4f20
Uploading finished image to https://ci-caches.rust-lang.org/docker/38eba79578bd6d379f830cfbe873c54dad8d10db1dfcf4fbac8af27e25e93a7ecaa2a7c62028c216d50741db91fde3d04f83489b4d576287b0239d35dee6e197
upload failed: - to s3://rust-lang-ci-sccache2/docker/38eba79578bd6d379f830cfbe873c54dad8d10db1dfcf4fbac8af27e25e93a7ecaa2a7c62028c216d50741db91fde3d04f83489b4d576287b0239d35dee6e197 Unable to locate credentials
[CI_JOB_NAME=mingw-check-tidy]
[CI_JOB_NAME=mingw-check-tidy]
---
   Compiling tidy v0.1.0 (/checkout/src/tools/tidy)
    Finished release [optimized] target(s) in 18.86s
fmt check
skip untracked path cpu-usage.csv during rustfmt invocations
Diff in /checkout/compiler/rustc_const_eval/src/interpret/intrinsics.rs at line 454:
                 if intrinsic_name == sym::assert_mem_uninitialized_valid {
                     let should_panic = !self
                         .tcx
-                        .check_validity_of_init(self.param_env.and((ty::InitKind::UninitMitigated0x01Fill, ty)))
+                        .check_validity_of_init(
+                            self.param_env.and((ty::InitKind::UninitMitigated0x01Fill, ty)),
+                        )
                         .map_err(|_| err_inval!(TooGeneric))?;
                     if should_panic {
                     if should_panic {
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/rustfmt/bin/rustfmt" "--config-path" "/checkout" "--edition" "2021" "--unstable-features" "--skip-children" "--check" "/checkout/compiler/rustc_const_eval/src/interpret/util.rs" "/checkout/compiler/rustc_const_eval/src/util/alignment.rs" "/checkout/compiler/rustc_const_eval/src/interpret/operand.rs" "/checkout/compiler/rustc_const_eval/src/util/mod.rs" "/checkout/compiler/rustc_const_eval/src/interpret/validity.rs" "/checkout/compiler/rustc_const_eval/src/util/find_self_call.rs" "/checkout/compiler/rustc_const_eval/src/interpret/intrinsics.rs" "/checkout/compiler/rustc_const_eval/src/util/call_kind.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
