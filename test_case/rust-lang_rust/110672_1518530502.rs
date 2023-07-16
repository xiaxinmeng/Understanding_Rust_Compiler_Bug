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
  Stored in directory: /tmp/pip-ephem-wheel-cache-l4eysl7f/wheels/c2/3c/b9/1120c2ab4bd82694f7e6f0537dc5b9a085c13e2c69a8d0c76d
Installing collected packages: boolean-py, binaryornot, setuptools, reuse, python-debian, markupsafe, license-expression, jinja2, chardet
  Attempting uninstall: setuptools
    Found existing installation: setuptools 59.6.0
    Not uninstalling setuptools at /usr/lib/python3/dist-packages, outside environment /usr
---
Successfully built 3bb525c71e9c
Successfully tagged rust-ci:latest
Built container sha256:3bb525c71e9ca5ca71eb78fd577fe32d4deb5c14c20317378d71ad8b42007bc8
Uploading finished image to https://ci-caches.rust-lang.org/docker/eee721fc1ad06e35509f22d2087fb4acdb0a4d378bec3a191179769e2503f17c7f8f4292d7013976df3a24502e822c590708d80a62f327ca4849564995e4deb7
upload failed: - to s3://rust-lang-ci-sccache2/docker/eee721fc1ad06e35509f22d2087fb4acdb0a4d378bec3a191179769e2503f17c7f8f4292d7013976df3a24502e822c590708d80a62f327ca4849564995e4deb7 Unable to locate credentials
[CI_JOB_NAME=mingw-check-tidy]
[CI_JOB_NAME=mingw-check-tidy]
---
   Compiling cargo_metadata v0.15.3
   Compiling tidy v0.1.0 (/checkout/src/tools/tidy)
    Finished release [optimized] target(s) in 13.02s
fmt check
Diff in /checkout/compiler/rustc_hir_analysis/src/check/intrinsicck.rs at line 87:
                 let (size, ty) = match elem_ty.kind() {
                 let (size, ty) = match elem_ty.kind() {
                     ty::Array(ty, len) => {
-                        if let Some(len) = len.try_eval_target_usize(self.tcx, self.tcx.param_env(adt.did())) {
+                        if let Some(len) =
+                            len.try_eval_target_usize(self.tcx, self.tcx.param_env(adt.did()))
+                        {
                             (len, *ty)
                             return None;
                             return None;
Diff in /checkout/compiler/rustc_hir_analysis/src/check/intrinsicck.rs at line 99:
                 match ty.kind() {
                     ty::Never | ty::Error(_) => return None,
                     ty::Int(IntTy::I8) | ty::Uint(UintTy::U8) => Some(InlineAsmType::VecI8(size)),
-                    ty::Int(IntTy::I16) | ty::Uint(UintTy::U16) => Some(InlineAsmType::VecI16(size)),
-                    ty::Int(IntTy::I32) | ty::Uint(UintTy::U32) => Some(InlineAsmType::VecI32(size)),
-                    ty::Int(IntTy::I64) | ty::Uint(UintTy::U64) => Some(InlineAsmType::VecI64(size)),
-                    ty::Int(IntTy::I128) | ty::Uint(UintTy::U128) => Some(InlineAsmType::VecI128(size)),
+                    ty::Int(IntTy::I16) | ty::Uint(UintTy::U16) => {
+                        Some(InlineAsmType::VecI16(size))
+                    }
+                    ty::Int(IntTy::I32) | ty::Uint(UintTy::U32) => {
+                        Some(InlineAsmType::VecI32(size))
+                    }
+                    ty::Int(IntTy::I64) | ty::Uint(UintTy::U64) => {
+                        Some(InlineAsmType::VecI64(size))
+                    }
+                    ty::Int(IntTy::I128) | ty::Uint(UintTy::U128) => {
+                        Some(InlineAsmType::VecI128(size))
+                    }
                     ty::Int(IntTy::Isize) | ty::Uint(UintTy::Usize) => {
                         Some(match self.tcx.sess.target.pointer_width {
                             16 => InlineAsmType::VecI16(size),
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/rustfmt/bin/rustfmt" "--config-path" "/checkout" "--edition" "2021" "--unstable-features" "--skip-children" "--check" "/checkout/compiler/rustc_traits/src/dropck_outlives.rs" "/checkout/compiler/rustc_traits/src/implied_outlives_bounds.rs" "/checkout/compiler/rustc_traits/src/evaluate_obligation.rs" "/checkout/compiler/rustc_lexer/src/lib.rs" "/checkout/compiler/rustc_type_ir/src/structural_impls.rs" "/checkout/compiler/rustc_hir_analysis/src/check/intrinsicck.rs" "/checkout/compiler/rustc_hir_analysis/src/autoderef.rs" "/checkout/compiler/rustc_traits/src/normalize_projection_ty.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
