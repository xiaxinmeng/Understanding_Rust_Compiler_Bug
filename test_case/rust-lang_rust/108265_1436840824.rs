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
  Stored in directory: /tmp/pip-ephem-wheel-cache-iin_6yrq/wheels/c2/3c/b9/1120c2ab4bd82694f7e6f0537dc5b9a085c13e2c69a8d0c76d
Installing collected packages: boolean-py, binaryornot, setuptools, reuse, python-debian, markupsafe, license-expression, jinja2, chardet
  Attempting uninstall: setuptools
    Found existing installation: setuptools 59.6.0
    Not uninstalling setuptools at /usr/lib/python3/dist-packages, outside environment /usr
---
Successfully built 1ca610724807
Successfully tagged rust-ci:latest
Built container sha256:1ca610724807ed38ba14c3dd550d344525fce1911f9a4d25b494542e5bf17fae
Uploading finished image to https://ci-caches.rust-lang.org/docker/38eba79578bd6d379f830cfbe873c54dad8d10db1dfcf4fbac8af27e25e93a7ecaa2a7c62028c216d50741db91fde3d04f83489b4d576287b0239d35dee6e197
upload failed: - to s3://rust-lang-ci-sccache2/docker/38eba79578bd6d379f830cfbe873c54dad8d10db1dfcf4fbac8af27e25e93a7ecaa2a7c62028c216d50741db91fde3d04f83489b4d576287b0239d35dee6e197 Unable to locate credentials
[CI_JOB_NAME=mingw-check-tidy]
[CI_JOB_NAME=mingw-check-tidy]
---
   Compiling tidy v0.1.0 (/checkout/src/tools/tidy)
    Finished release [optimized] target(s) in 17.07s
fmt check
skip untracked path cpu-usage.csv during rustfmt invocations
Diff in /checkout/compiler/rustc_hir_analysis/src/astconv/generics.rs at line 6:
 use crate::errors::AssocTypeBindingNotAllowed;
 use crate::structured_errors::{GenericArgsInfo, StructuredDiagnostic, WrongNumberOfGenericArgs};
 use rustc_ast::ast::ParamKindOrd;
-use rustc_errors::{struct_span_err, Applicability, Diagnostic, MultiSpan, ErrorGuaranteed};
+use rustc_errors::{struct_span_err, Applicability, Diagnostic, ErrorGuaranteed, MultiSpan};
 use rustc_hir as hir;
 use rustc_hir::def::{DefKind, Res};
 use rustc_hir::def_id::DefId;
Diff in /checkout/compiler/rustc_hir_analysis/src/astconv/generics.rs at line 70:
         ) => match path.res {
                 add_braces_suggestion(arg, &mut err);
                 add_braces_suggestion(arg, &mut err);
-                return err.set_primary_message("unresolved item provided when a constant was expected")
+                return err
+                    .set_primary_message("unresolved item provided when a constant was expected")
                     .emit();
             }
             Res::Def(DefKind::TyParam, src_def_id) => {
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/rustfmt/bin/rustfmt" "--config-path" "/checkout" "--edition" "2021" "--unstable-features" "--skip-children" "--check" "/checkout/compiler/rustc_hir_analysis/src/astconv/errors.rs" "/checkout/compiler/rustc_hir_analysis/src/astconv/generics.rs" "/checkout/compiler/rustc_hir_analysis/src/astconv/mod.rs" "/checkout/compiler/rustc_hir_analysis/src/coherence/builtin.rs" "/checkout/compiler/rustc_hir_analysis/src/coherence/inherent_impls.rs" "/checkout/compiler/rustc_hir_analysis/src/coherence/unsafety.rs" "/checkout/compiler/rustc_hir_analysis/src/coherence/inherent_impls_overlap.rs" "/checkout/compiler/rustc_hir_analysis/src/check/compare_impl_item.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
