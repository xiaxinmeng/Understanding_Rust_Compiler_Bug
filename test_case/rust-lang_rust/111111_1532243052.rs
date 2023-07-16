plain
Prepare all required actions
Getting action download info
Download action repository 'actions/checkout@v3' (SHA:8e5e7e5ab8b370d6c329ec480221332ada57f0ab)
Download action repository 'rust-lang/simpleinfra@master' (SHA:11f435bab7d0fe51e2f33ae0ce77f457216db8a6)
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
  Stored in directory: /tmp/pip-ephem-wheel-cache-g4nlmc5x/wheels/c2/3c/b9/1120c2ab4bd82694f7e6f0537dc5b9a085c13e2c69a8d0c76d
Installing collected packages: boolean-py, binaryornot, setuptools, reuse, python-debian, markupsafe, license-expression, jinja2, chardet
  Attempting uninstall: setuptools
    Found existing installation: setuptools 59.6.0
    Not uninstalling setuptools at /usr/lib/python3/dist-packages, outside environment /usr
---
Successfully built ae7ed4fe7e3f
Successfully tagged rust-ci:latest
Built container sha256:ae7ed4fe7e3f536fc7a70397aecc71778227d2048e05f04142058f7525794ac2
Uploading finished image to https://ci-caches.rust-lang.org/docker/e51156f19850ce886cec818e46dc2a021e0aa7c270d15673e8fe74cd8522fc8ac3995109aebb688ee49ed586735ed4cf5f8c06d44c48298fa09c35ae4b082281
upload failed: - to s3://rust-lang-ci-sccache2/docker/e51156f19850ce886cec818e46dc2a021e0aa7c270d15673e8fe74cd8522fc8ac3995109aebb688ee49ed586735ed4cf5f8c06d44c48298fa09c35ae4b082281 Unable to locate credentials
[CI_JOB_NAME=mingw-check-tidy]
[CI_JOB_NAME=mingw-check-tidy]
---
   Compiling tidy v0.1.0 (/checkout/src/tools/tidy)
    Finished release [optimized] target(s) in 16.62s
##[endgroup]
fmt check
error[internal]: left behind trailing whitespace
   --> /checkout/compiler/rustc_hir_analysis/src/collect/resolve_bound_vars.rs:921:921:53
921 |                         if let Some(arg) = default { 
    |                                                     ^
    |


warning: rustfmt has failed to format. See previous 1 errors.

Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/rustfmt/bin/rustfmt" "--config-path" "/checkout" "--edition" "2021" "--unstable-features" "--skip-children" "--check" "/checkout/compiler/rustc_hir_analysis/src/coherence/orphan.rs" "/checkout/compiler/rustc_hir_analysis/src/coherence/inherent_impls.rs" "/checkout/compiler/rustc_hir_analysis/src/collect/generics_of.rs" "/checkout/compiler/rustc_hir_analysis/src/coherence/inherent_impls_overlap.rs" "/checkout/compiler/rustc_hir_analysis/src/collect/type_of.rs" "/checkout/compiler/rustc_hir_analysis/src/coherence/builtin.rs" "/checkout/compiler/rustc_hir_analysis/src/collect/resolve_bound_vars.rs" "/checkout/compiler/rustc_hir_analysis/src/coherence/mod.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
