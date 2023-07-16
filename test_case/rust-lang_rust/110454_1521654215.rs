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
  Stored in directory: /tmp/pip-ephem-wheel-cache-ha4mapl_/wheels/c2/3c/b9/1120c2ab4bd82694f7e6f0537dc5b9a085c13e2c69a8d0c76d
Installing collected packages: boolean-py, binaryornot, setuptools, reuse, python-debian, markupsafe, license-expression, jinja2, chardet
  Attempting uninstall: setuptools
    Found existing installation: setuptools 59.6.0
    Not uninstalling setuptools at /usr/lib/python3/dist-packages, outside environment /usr
---
Successfully built 6b64c3e86164
Successfully tagged rust-ci:latest
Built container sha256:6b64c3e86164f7d6fe30daf5d50cdc11be0ccc275605395a13bbe3f888f0f64e
Uploading finished image to https://ci-caches.rust-lang.org/docker/eee721fc1ad06e35509f22d2087fb4acdb0a4d378bec3a191179769e2503f17c7f8f4292d7013976df3a24502e822c590708d80a62f327ca4849564995e4deb7
upload failed: - to s3://rust-lang-ci-sccache2/docker/eee721fc1ad06e35509f22d2087fb4acdb0a4d378bec3a191179769e2503f17c7f8f4292d7013976df3a24502e822c590708d80a62f327ca4849564995e4deb7 Unable to locate credentials
[CI_JOB_NAME=mingw-check-tidy]
[CI_JOB_NAME=mingw-check-tidy]
---
   Compiling tidy v0.1.0 (/checkout/src/tools/tidy)
    Finished release [optimized] target(s) in 13.48s
##[endgroup]
fmt check
Diff in /checkout/compiler/rustc_hir_analysis/src/collect/type_of.rs at line 426:
                     let substs = InternalSubsts::identity_for_item(tcx, def_id);
                     tcx.mk_adt(def, substs)
                 }
-                ItemKind::OpaqueTy(OpaqueTy { origin: hir::OpaqueTyOrigin::TyAlias { .. }, .. }) => {
-                    find_opaque_ty_constraints_for_tait(tcx, def_id)
-                }
+                ItemKind::OpaqueTy(OpaqueTy {
+                    origin: hir::OpaqueTyOrigin::TyAlias { .. },
+                    ..
+                }) => find_opaque_ty_constraints_for_tait(tcx, def_id),
                 // Opaque types desugared from `impl Trait`.
                 ItemKind::OpaqueTy(OpaqueTy {
                     origin:
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/rustfmt/bin/rustfmt" "--config-path" "/checkout" "--edition" "2021" "--unstable-features" "--skip-children" "--check" "/checkout/compiler/rustc_hir_analysis/src/collect/predicates_of.rs" "/checkout/compiler/rustc_hir_analysis/src/collect/generics_of.rs" "/checkout/compiler/rustc_hir_analysis/src/collect/resolve_bound_vars.rs" "/checkout/compiler/rustc_hir_analysis/src/collect/type_of.rs" "/checkout/compiler/rustc_lexer/src/lib.rs" "/checkout/compiler/rustc_ast_lowering/src/asm.rs" "/checkout/compiler/rustc_ast_lowering/src/item.rs" "/checkout/compiler/rustc_hir_analysis/src/collect/item_bounds.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
