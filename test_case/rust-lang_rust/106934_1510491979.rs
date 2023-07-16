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
  Stored in directory: /tmp/pip-ephem-wheel-cache-k8q32xfj/wheels/c2/3c/b9/1120c2ab4bd82694f7e6f0537dc5b9a085c13e2c69a8d0c76d
Installing collected packages: boolean-py, binaryornot, setuptools, reuse, python-debian, markupsafe, license-expression, jinja2, chardet
  Attempting uninstall: setuptools
    Found existing installation: setuptools 59.6.0
    Not uninstalling setuptools at /usr/lib/python3/dist-packages, outside environment /usr
---
Successfully built 64b2024e21af
Successfully tagged rust-ci:latest
Built container sha256:64b2024e21af7b05a4f07df033b0f8a8c73c136cdbfc2c91127c58eccddcdb54
Uploading finished image to https://ci-caches.rust-lang.org/docker/5a83b1174027be8472df586f36f1b4e77c39a20882cc84831c17b85c3cc5731b4e272c71c37dd18e09cacea39bf89c1b3226a304723983ec1270656c20627ec9
upload failed: - to s3://rust-lang-ci-sccache2/docker/5a83b1174027be8472df586f36f1b4e77c39a20882cc84831c17b85c3cc5731b4e272c71c37dd18e09cacea39bf89c1b3226a304723983ec1270656c20627ec9 Unable to locate credentials
[CI_JOB_NAME=mingw-check-tidy]
[CI_JOB_NAME=mingw-check-tidy]
---
   Compiling cargo_metadata v0.15.3
   Compiling tidy v0.1.0 (/checkout/src/tools/tidy)
    Finished release [optimized] target(s) in 13.40s
fmt check
Diff in /checkout/compiler/rustc_passes/src/dead.rs at line 255:
                     self.insert_def_id(field.did);
                     let field_ty = field.ty(self.tcx, subst);
-                    current_ty =
-                    current_ty =
-                        self.tcx.normalize_erasing_regions(param_env, field_ty);
+                    current_ty = self.tcx.normalize_erasing_regions(param_env, field_ty);
                 // we don't need to mark tuple fields as live,
                 // we don't need to mark tuple fields as live,
                 // but we may need to mark subfields
Diff in /checkout/compiler/rustc_passes/src/dead.rs at line 263:
                 ty::Tuple(tys) => {
-                    current_ty = self.tcx.normalize_erasing_regions(
-                        param_env,
-                        tys[index.as_usize()],
+                    current_ty =
+                    current_ty =
+                        self.tcx.normalize_erasing_regions(param_env, tys[index.as_usize()]);
                 }
                 _ => span_bug!(expr.span, "named field access on non-ADT"),
             }
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/rustfmt/bin/rustfmt" "--config-path" "/checkout" "--edition" "2021" "--unstable-features" "--skip-children" "--check" "/checkout/compiler/rustc_passes/src/lib_features.rs" "/checkout/compiler/rustc_passes/src/dead.rs" "/checkout/compiler/rustc_passes/src/liveness/rwu_table.rs" "/checkout/compiler/rustc_const_eval/src/errors.rs" "/checkout/compiler/rustc_arena/src/tests.rs" "/checkout/compiler/rustc_arena/src/lib.rs" "/checkout/compiler/rustc_const_eval/src/lib.rs" "/checkout/compiler/rustc_passes/src/entry.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
