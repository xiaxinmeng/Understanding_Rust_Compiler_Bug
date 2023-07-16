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
  Stored in directory: /tmp/pip-ephem-wheel-cache-qwpwdffd/wheels/c2/3c/b9/1120c2ab4bd82694f7e6f0537dc5b9a085c13e2c69a8d0c76d
Installing collected packages: boolean-py, binaryornot, setuptools, reuse, python-debian, markupsafe, license-expression, jinja2, chardet
  Attempting uninstall: setuptools
    Found existing installation: setuptools 59.6.0
    Not uninstalling setuptools at /usr/lib/python3/dist-packages, outside environment /usr
---
Successfully built a686bc957b7b
Successfully tagged rust-ci:latest
Built container sha256:a686bc957b7bfe0ff5a66c0179a7dbb05b2f622a75a965afe3bdd0f230463360
Uploading finished image to https://ci-caches.rust-lang.org/docker/38eba79578bd6d379f830cfbe873c54dad8d10db1dfcf4fbac8af27e25e93a7ecaa2a7c62028c216d50741db91fde3d04f83489b4d576287b0239d35dee6e197
upload failed: - to s3://rust-lang-ci-sccache2/docker/38eba79578bd6d379f830cfbe873c54dad8d10db1dfcf4fbac8af27e25e93a7ecaa2a7c62028c216d50741db91fde3d04f83489b4d576287b0239d35dee6e197 Unable to locate credentials
[CI_JOB_NAME=mingw-check-tidy]
[CI_JOB_NAME=mingw-check-tidy]
---
   Compiling tidy v0.1.0 (/checkout/src/tools/tidy)
    Finished release [optimized] target(s) in 18.08s
fmt check
skip untracked path cpu-usage.csv during rustfmt invocations
Diff in /checkout/compiler/rustc_trait_selection/src/solve/canonical/canonicalize.rs at line 6:
 use rustc_middle::infer::canonical::CanonicalVarInfo;
 use rustc_middle::infer::canonical::CanonicalVarInfos;
 use rustc_middle::infer::canonical::CanonicalVarKind;
+use rustc_middle::ty::ir::TypeFolder;
 use rustc_middle::ty::BoundRegionKind::BrAnon;
 use rustc_middle::ty::BoundTyKind;
 use rustc_middle::ty::TyCtxt;
Diff in /checkout/compiler/rustc_trait_selection/src/solve/canonical/canonicalize.rs at line 12:
-use rustc_middle::ty::ir::TypeFolder;
 use rustc_middle::ty::{self, Ty};
 use rustc_middle::ty::{TypeFoldable, TypeSuperFoldable};
 
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/rustfmt/bin/rustfmt" "--config-path" "/checkout" "--edition" "2021" "--unstable-features" "--skip-children" "--check" "/checkout/compiler/rustc_trait_selection/src/solve/trait_goals/structural_traits.rs" "/checkout/compiler/rustc_trait_selection/src/solve/trait_goals.rs" "/checkout/compiler/rustc_trait_selection/src/traits/auto_trait.rs" "/checkout/compiler/rustc_trait_selection/src/solve/mod.rs" "/checkout/compiler/rustc_trait_selection/src/solve/infcx_ext.rs" "/checkout/compiler/rustc_trait_selection/src/solve/canonical/canonicalize.rs" "/checkout/compiler/rustc_trait_selection/src/solve/canonical/mod.rs" "/checkout/compiler/rustc_trait_selection/src/traits/error_reporting/on_unimplemented.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
