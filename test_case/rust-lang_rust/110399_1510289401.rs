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
  Stored in directory: /tmp/pip-ephem-wheel-cache-h05702uk/wheels/c2/3c/b9/1120c2ab4bd82694f7e6f0537dc5b9a085c13e2c69a8d0c76d
Installing collected packages: boolean-py, binaryornot, setuptools, reuse, python-debian, markupsafe, license-expression, jinja2, chardet
  Attempting uninstall: setuptools
    Found existing installation: setuptools 59.6.0
    Not uninstalling setuptools at /usr/lib/python3/dist-packages, outside environment /usr
---
Successfully built cb2b6c0aab98
Successfully tagged rust-ci:latest
Built container sha256:cb2b6c0aab98758290411c24b17a24cdcddab28e2505e938996f8fe4ad596ecb
Uploading finished image to https://ci-caches.rust-lang.org/docker/5a83b1174027be8472df586f36f1b4e77c39a20882cc84831c17b85c3cc5731b4e272c71c37dd18e09cacea39bf89c1b3226a304723983ec1270656c20627ec9
upload failed: - to s3://rust-lang-ci-sccache2/docker/5a83b1174027be8472df586f36f1b4e77c39a20882cc84831c17b85c3cc5731b4e272c71c37dd18e09cacea39bf89c1b3226a304723983ec1270656c20627ec9 Unable to locate credentials
[CI_JOB_NAME=mingw-check-tidy]
[CI_JOB_NAME=mingw-check-tidy]
---
   Compiling cargo_metadata v0.15.3
   Compiling tidy v0.1.0 (/checkout/src/tools/tidy)
    Finished release [optimized] target(s) in 13.05s
fmt check
Diff in /checkout/compiler/rustc_infer/src/infer/outlives/components.rs at line 200:
     visited: &mut SsoHashSet<GenericArg<'tcx>>,
 ) {
     let ty::Alias(kind, alias_ty) = alias_ty.kind() else { bug!() };
-    let opt_variances = if *kind == ty::Opaque {
-        tcx.variances_of(alias_ty.def_id)
-        &[]
-    };
-    };
+    let opt_variances = if *kind == ty::Opaque { tcx.variances_of(alias_ty.def_id) } else { &[] };
     for (index, child) in alias_ty.substs.iter().enumerate() {
         if opt_variances.get(index) == Some(&ty::Bivariant) {
Diff in /checkout/compiler/rustc_infer/src/infer/outlives/verify.rs at line 130:
         // see the extensive comment in projection_must_outlive
         let recursive_bound = {
             let mut components = smallvec![];
             let mut components = smallvec![];
-            compute_alias_components_recursive(self.tcx, alias_ty_as_ty.into(), &mut components, visited);
+                self.tcx,
+                self.tcx,
+                alias_ty_as_ty.into(),
+                visited,
+            );
+            );
             self.bound_from_components(&components, visited)
 
 
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/rustfmt/bin/rustfmt" "--config-path" "/checkout" "--edition" "2021" "--unstable-features" "--skip-children" "--check" "/checkout/compiler/rustc_infer/src/infer/nll_relate/mod.rs" "/checkout/compiler/rustc_infer/src/infer/outlives/mod.rs" "/checkout/compiler/rustc_infer/src/infer/outlives/components.rs" "/checkout/compiler/rustc_infer/src/infer/outlives/env.rs" "/checkout/compiler/rustc_infer/src/infer/outlives/test_type_match.rs" "/checkout/compiler/rustc_infer/src/infer/outlives/verify.rs" "/checkout/compiler/rustc_infer/src/infer/outlives/obligations.rs" "/checkout/compiler/rustc_infer/src/infer/mod.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
