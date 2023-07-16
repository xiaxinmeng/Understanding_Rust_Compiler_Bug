plain
Prepare all required actions
Getting action download info
Download action repository 'actions/checkout@v3' (SHA:8e5e7e5ab8b370d6c329ec480221332ada57f0ab)
Download action repository 'rust-lang/simpleinfra@master' (SHA:cdb59432ef9e018dd6db5879e720eccc3289da68)
Complete job name: PR - mingw-check-tidy
git config --global core.autocrlf false
shell: /usr/bin/bash --noprofile --norc -e -o pipefail {0}
env:
  CI_JOB_NAME: mingw-check-tidy
---
Building wheels for collected packages: reuse
  Building wheel for reuse (pyproject.toml): started
  Building wheel for reuse (pyproject.toml): finished with status 'done'
  Created wheel for reuse: filename=reuse-1.1.0-cp310-cp310-manylinux_2_35_x86_64.whl size=180115 sha256=c3e1cf75e5aea7348ad201d16633573bc9f3aa58c8e29a6ca459f306300d6c9d
  Stored in directory: /tmp/pip-ephem-wheel-cache-qyn3fulq/wheels/c2/3c/b9/1120c2ab4bd82694f7e6f0537dc5b9a085c13e2c69a8d0c76d
Installing collected packages: boolean-py, binaryornot, setuptools, reuse, python-debian, markupsafe, license-expression, jinja2, chardet
  Attempting uninstall: setuptools
    Found existing installation: setuptools 59.6.0
    Not uninstalling setuptools at /usr/lib/python3/dist-packages, outside environment /usr
---
Successfully tagged rust-ci:latest
Built container sha256:788e4ec426bf797936bd89d1176a8d4b775f463f43b08c9ba34e979976255e05
Uploading finished image to https://ci-caches.rust-lang.org/docker/e51156f19850ce886cec818e46dc2a021e0aa7c270d15673e8fe74cd8522fc8ac3995109aebb688ee49ed586735ed4cf5f8c06d44c48298fa09c35ae4b082281

<botocore.awsrequest.AWSRequest object at 0x7fd816f83e90>
gzip: stdout: Broken pipe
xargs: docker: terminated by signal 13
[CI_JOB_NAME=mingw-check-tidy]
[CI_JOB_NAME=mingw-check-tidy]
---
   Compiling tidy v0.1.0 (/checkout/src/tools/tidy)
    Finished release [optimized] target(s) in 15.24s
##[endgroup]
fmt check
Diff in /checkout/compiler/rustc_hir_typeck/src/writeback.rs at line 13:
 use rustc_middle::mir::FakeReadCause;
 use rustc_middle::ty::adjustment::{Adjust, Adjustment, PointerCast};
 use rustc_middle::ty::fold::{TypeFoldable, TypeFolder, TypeSuperFoldable};
-use rustc_middle::ty::visit::{TypeVisitableExt};
+use rustc_middle::ty::visit::TypeVisitableExt;
 use rustc_middle::ty::TypeckResults;
 use rustc_middle::ty::{self, ClosureSizeProfileData, Ty, TyCtxt};
 use rustc_span::symbol::sym;
Diff in /checkout/compiler/rustc_hir_typeck/src/writeback.rs at line 560:
         for (opaque_type_key, decl) in opaque_types {
             let hidden_type = self.resolve(decl.hidden_type, &decl.hidden_type.span);
             let opaque_type_key = self.resolve(opaque_type_key, &decl.hidden_type.span);
+
+
             // When defining an opaque to be literally itself we don't record that
             // as it may be a non-defining use. MIR typecheck will handle this instead.
-            if let ty::Alias(ty::Opaque, alias_ty) =  hidden_type.ty.kind() {
+            if let ty::Alias(ty::Opaque, alias_ty) = hidden_type.ty.kind() {
                 if alias_ty.def_id == opaque_type_key.def_id.to_def_id() {
                 }
                 }
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/rustfmt/bin/rustfmt" "--config-path" "/checkout" "--edition" "2021" "--unstable-features" "--skip-children" "--check" "/checkout/compiler/rustc_hir_analysis/src/outlives/utils.rs" "/checkout/compiler/rustc_hir_analysis/src/outlives/mod.rs" "/checkout/compiler/rustc_hir_analysis/src/outlives/test.rs" "/checkout/compiler/rustc_hir_typeck/src/writeback.rs" "/checkout/compiler/rustc_hir_typeck/src/fn_ctxt/adjust_fulfillment_errors.rs" "/checkout/compiler/rustc_hir_typeck/src/fn_ctxt/arg_matrix.rs" "/checkout/compiler/rustc_hir_typeck/src/fn_ctxt/_impl.rs" "/checkout/compiler/rustc_hir_analysis/src/outlives/explicit.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
