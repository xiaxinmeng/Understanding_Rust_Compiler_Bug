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
  Stored in directory: /tmp/pip-ephem-wheel-cache-1rc9skgm/wheels/c2/3c/b9/1120c2ab4bd82694f7e6f0537dc5b9a085c13e2c69a8d0c76d
Installing collected packages: boolean-py, binaryornot, setuptools, reuse, python-debian, markupsafe, license-expression, jinja2, chardet
  Attempting uninstall: setuptools
    Found existing installation: setuptools 59.6.0
    Not uninstalling setuptools at /usr/lib/python3/dist-packages, outside environment /usr
---
Successfully built 6618c8a69f2b
Successfully tagged rust-ci:latest
Built container sha256:6618c8a69f2bd860083588cc2f64887d3cd4dafc98fec48f24958eb4d00aa15c
Uploading finished image to https://ci-caches.rust-lang.org/docker/eee721fc1ad06e35509f22d2087fb4acdb0a4d378bec3a191179769e2503f17c7f8f4292d7013976df3a24502e822c590708d80a62f327ca4849564995e4deb7
upload failed: - to s3://rust-lang-ci-sccache2/docker/eee721fc1ad06e35509f22d2087fb4acdb0a4d378bec3a191179769e2503f17c7f8f4292d7013976df3a24502e822c590708d80a62f327ca4849564995e4deb7 Unable to locate credentials
[CI_JOB_NAME=mingw-check-tidy]
[CI_JOB_NAME=mingw-check-tidy]
---
   Compiling cargo_metadata v0.15.3
   Compiling tidy v0.1.0 (/checkout/src/tools/tidy)
    Finished release [optimized] target(s) in 13.18s
fmt check
Diff in /checkout/compiler/rustc_borrowck/src/diagnostics/mutability_errors.rs at line 1318:
 /// If possible, suggest replacing `ref` with `ref mut`.
 fn suggest_ref_mut(tcx: TyCtxt<'_>, span: Span) -> Option<Span> {
     let pattern_str = tcx.sess.source_map().span_to_snippet(span).ok()?;
-    if pattern_str.starts_with("ref") && pattern_str["ref".len()..].starts_with(rustc_lexer::is_whitespace) {
+    if pattern_str.starts_with("ref")
+        && pattern_str["ref".len()..].starts_with(rustc_lexer::is_whitespace)
+    {
         let span = span.with_lo(span.lo() + BytePos(4)).shrink_to_lo();
         Some(span)
     } else {
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/rustfmt/bin/rustfmt" "--config-path" "/checkout" "--edition" "2021" "--unstable-features" "--skip-children" "--check" "/checkout/compiler/rustc_borrowck/src/diagnostics/region_name.rs" "/checkout/compiler/rustc_borrowck/src/diagnostics/bound_region_errors.rs" "/checkout/compiler/rustc_borrowck/src/type_check/input_output.rs" "/checkout/compiler/rustc_borrowck/src/type_check/canonical.rs" "/checkout/compiler/rustc_borrowck/src/diagnostics/mutability_errors.rs" "/checkout/compiler/rustc_borrowck/src/diagnostics/find_use.rs" "/checkout/compiler/rustc_borrowck/src/type_check/free_region_relations.rs" "/checkout/compiler/rustc_borrowck/src/type_check/mod.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
