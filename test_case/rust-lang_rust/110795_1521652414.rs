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
  Stored in directory: /tmp/pip-ephem-wheel-cache-djp5cp8t/wheels/c2/3c/b9/1120c2ab4bd82694f7e6f0537dc5b9a085c13e2c69a8d0c76d
Installing collected packages: boolean-py, binaryornot, setuptools, reuse, python-debian, markupsafe, license-expression, jinja2, chardet
  Attempting uninstall: setuptools
    Found existing installation: setuptools 59.6.0
    Not uninstalling setuptools at /usr/lib/python3/dist-packages, outside environment /usr
---
Successfully built 096c8bcd3848
Successfully tagged rust-ci:latest
Built container sha256:096c8bcd3848303878aec5f1a1113d5c9976260351fc7b181e6d62c1b55bd41b
Uploading finished image to https://ci-caches.rust-lang.org/docker/eee721fc1ad06e35509f22d2087fb4acdb0a4d378bec3a191179769e2503f17c7f8f4292d7013976df3a24502e822c590708d80a62f327ca4849564995e4deb7
upload failed: - to s3://rust-lang-ci-sccache2/docker/eee721fc1ad06e35509f22d2087fb4acdb0a4d378bec3a191179769e2503f17c7f8f4292d7013976df3a24502e822c590708d80a62f327ca4849564995e4deb7 Unable to locate credentials
[CI_JOB_NAME=mingw-check-tidy]
[CI_JOB_NAME=mingw-check-tidy]
---
fmt check
Diff in /checkout/compiler/rustc_middle/src/ty/subst.rs at line 67:
                 // Ensure we can use the tag bits.
 
                 let r: &WithCachedTypeInfo<ty::TyKind<'tcx>> = ty.0.pointer().0;
-                assert_eq!(mem::align_of_val::<WithCachedTypeInfo<ty::TyKind<'tcx>>>(r) & TAG_MASK, 0);
+                assert_eq!(
+                    mem::align_of_val::<WithCachedTypeInfo<ty::TyKind<'tcx>>>(r) & TAG_MASK,
+                );
+                );
                 (TYPE_TAG, r as *const WithCachedTypeInfo<ty::TyKind<'tcx>> as usize)
             }
             GenericArgKind::Const(ct) => {
Diff in /checkout/compiler/rustc_middle/src/ty/sty.rs at line 5:
 use crate::infer::canonical::Canonical;
 use crate::ty::subst::{GenericArg, InternalSubsts, SubstsRef};
 use crate::ty::visit::ValidateBoundVars;
-use crate::ty::{InferTy::*, HotTypeFlags};
 use crate::ty::{
     self, AdtDef, Discr, Term, Ty, TyCtxt, TypeFlags, TypeSuperVisitable, TypeVisitable,
     TypeVisitableExt, TypeVisitor,
Diff in /checkout/compiler/rustc_middle/src/ty/sty.rs at line 12:
 };
+use crate::ty::{HotTypeFlags, InferTy::*};
 use crate::ty::{List, ParamEnv};
 use hir::def::DefKind;
Diff in /checkout/compiler/rustc_middle/src/ty/sty.rs at line 1716:
     pub fn flags(self) -> TypeFlags {
         self.0.flags
     }
     }
-    
+
     #[inline(always)]
     pub fn hot_flags(self) -> HotTypeFlags {
         self.0.tag()
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/rustfmt/bin/rustfmt" "--config-path" "/checkout" "--edition" "2021" "--unstable-features" "--skip-children" "--check" "/checkout/compiler/rustc_middle/src/ty/consts/int.rs" "/checkout/compiler/rustc_middle/src/ty/consts/kind.rs" "/checkout/compiler/rustc_middle/src/ty/query.rs" "/checkout/compiler/rustc_middle/src/ty/adjustment.rs" "/checkout/compiler/rustc_middle/src/ty/typeck_results.rs" "/checkout/compiler/rustc_middle/src/ty/subst.rs" "/checkout/compiler/rustc_middle/src/ty/list.rs" "/checkout/compiler/rustc_middle/src/ty/erase_regions.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
