plain
Prepare all required actions
Getting action download info
Download action repository 'actions/checkout@v3' (SHA:8e5e7e5ab8b370d6c329ec480221332ada57f0ab)
Download action repository 'rust-lang/simpleinfra@master' (SHA:a244d6aab06843e1f4ae41ff60c7091ad809a20e)
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
  Stored in directory: /tmp/pip-ephem-wheel-cache-lv6ljeb5/wheels/c2/3c/b9/1120c2ab4bd82694f7e6f0537dc5b9a085c13e2c69a8d0c76d
Installing collected packages: boolean-py, binaryornot, setuptools, reuse, python-debian, markupsafe, license-expression, jinja2, chardet
  Attempting uninstall: setuptools
    Found existing installation: setuptools 59.6.0
    Not uninstalling setuptools at /usr/lib/python3/dist-packages, outside environment /usr
---
Successfully built 05c5ffd2f567
Successfully tagged rust-ci:latest
Built container sha256:05c5ffd2f567431361ce8138c353535d4333b61d24d3c777c65b2c1ad0a8025c
Uploading finished image to https://ci-caches.rust-lang.org/docker/e51156f19850ce886cec818e46dc2a021e0aa7c270d15673e8fe74cd8522fc8ac3995109aebb688ee49ed586735ed4cf5f8c06d44c48298fa09c35ae4b082281
upload failed: - to s3://rust-lang-ci-sccache2/docker/e51156f19850ce886cec818e46dc2a021e0aa7c270d15673e8fe74cd8522fc8ac3995109aebb688ee49ed586735ed4cf5f8c06d44c48298fa09c35ae4b082281 Unable to locate credentials
[CI_JOB_NAME=mingw-check-tidy]
[CI_JOB_NAME=mingw-check-tidy]
---
   Compiling tidy v0.1.0 (/checkout/src/tools/tidy)
    Finished release [optimized] target(s) in 16.74s
##[endgroup]
fmt check
Diff in /checkout/compiler/rustc_mir_transform/src/check_niches.rs at line 78:
 
 
 impl<'tcx> NicheFinder<'tcx> {
-        &mut self,
-        statement: &Statement<'tcx>,
-        statement: &Statement<'tcx>,
-    ) -> Vec<(Operand<'tcx>, Niche)> {
+    fn find(&mut self, statement: &Statement<'tcx>) -> Vec<(Operand<'tcx>, Niche)> {
         self.operands.clear();
         self.visit_statement(statement, Location::START);
         core::mem::take(&mut self.operands)
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/rustfmt/bin/rustfmt" "--config-path" "/checkout" "--edition" "2021" "--unstable-features" "--skip-children" "--check" "/checkout/compiler/rustc_mir_transform/src/ctfe_limit.rs" "/checkout/compiler/rustc_mir_transform/src/ffi_unwind_calls.rs" "/checkout/compiler/rustc_mir_transform/src/shim.rs" "/checkout/compiler/rustc_mir_transform/src/check_niches.rs" "/checkout/compiler/rustc_mir_transform/src/simplify.rs" "/checkout/compiler/rustc_mir_transform/src/check_const_item_mutation.rs" "/checkout/compiler/rustc_mir_transform/src/inline/cycle.rs" "/checkout/compiler/rustc_mir_transform/src/multiple_return_terminators.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
