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
  Stored in directory: /tmp/pip-ephem-wheel-cache-2xch5h8v/wheels/c2/3c/b9/1120c2ab4bd82694f7e6f0537dc5b9a085c13e2c69a8d0c76d
Installing collected packages: boolean-py, binaryornot, setuptools, reuse, python-debian, markupsafe, license-expression, jinja2, chardet
  Attempting uninstall: setuptools
    Found existing installation: setuptools 59.6.0
    Not uninstalling setuptools at /usr/lib/python3/dist-packages, outside environment /usr
---
Successfully built 7e662a4ff643
Successfully tagged rust-ci:latest
Built container sha256:7e662a4ff6437975197e7a9c5b8102dc2ea1160873cd8363521c33018144af2a
Uploading finished image to https://ci-caches.rust-lang.org/docker/38eba79578bd6d379f830cfbe873c54dad8d10db1dfcf4fbac8af27e25e93a7ecaa2a7c62028c216d50741db91fde3d04f83489b4d576287b0239d35dee6e197
upload failed: - to s3://rust-lang-ci-sccache2/docker/38eba79578bd6d379f830cfbe873c54dad8d10db1dfcf4fbac8af27e25e93a7ecaa2a7c62028c216d50741db91fde3d04f83489b4d576287b0239d35dee6e197 Unable to locate credentials
[CI_JOB_NAME=mingw-check-tidy]
[CI_JOB_NAME=mingw-check-tidy]
---
   Compiling tidy v0.1.0 (/checkout/src/tools/tidy)
    Finished release [optimized] target(s) in 17.58s
fmt check
skip untracked path cpu-usage.csv during rustfmt invocations
Diff in /checkout/compiler/rustc_resolve/src/late.rs at line 2043:
                     TyKind::ImplicitSelf => true,
                     TyKind::Path(None, _) => {
                         let path_res = self.r.partial_res_map[&ty.id].full_res();
-                        if matches!(path_res, Some(Res::SelfTyParam { .. } | Res::SelfTyAlias { .. })) {
+                        if matches!(
+                            path_res,
+                            Some(Res::SelfTyParam { .. } | Res::SelfTyAlias { .. })
+                        ) {
                         }
                         }
                         self.impl_self.is_some() && path_res == self.impl_self
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/rustfmt/bin/rustfmt" "--config-path" "/checkout" "--edition" "2021" "--unstable-features" "--skip-children" "--check" "/checkout/compiler/rustc_codegen_ssa/src/traits/consts.rs" "/checkout/compiler/rustc_resolve/src/effective_visibilities.rs" "/checkout/compiler/rustc_codegen_ssa/src/traits/intrinsic.rs" "/checkout/compiler/rustc_resolve/src/rustdoc.rs" "/checkout/compiler/rustc_codegen_ssa/src/traits/builder.rs" "/checkout/compiler/rustc_resolve/src/late.rs" "/checkout/compiler/rustc_codegen_ssa/src/traits/debuginfo.rs" "/checkout/compiler/rustc_resolve/src/diagnostics.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
