plain
Prepare all required actions
Getting action download info
Download action repository 'actions/checkout@v3' (SHA:8f4b7f84864484a7bf31766abe9204da3cbe65b3)
Download action repository 'rust-lang/simpleinfra@master' (SHA:010cbab48d11ea0c3fac8ecc803f840a1f91311c)
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
  Stored in directory: /tmp/pip-ephem-wheel-cache-v9j8ej_0/wheels/c2/3c/b9/1120c2ab4bd82694f7e6f0537dc5b9a085c13e2c69a8d0c76d
Installing collected packages: boolean-py, binaryornot, setuptools, reuse, python-debian, markupsafe, license-expression, jinja2, chardet
  Attempting uninstall: setuptools
    Found existing installation: setuptools 59.6.0
    Not uninstalling setuptools at /usr/lib/python3/dist-packages, outside environment /usr
---
Successfully built b555038b63f4
Successfully tagged rust-ci:latest
Built container sha256:b555038b63f475540f75ed8e16e3402efeaac2ed6c611282cf94f0403d582554
Uploading finished image to https://ci-caches.rust-lang.org/docker/a9e2a06b4251eb289588a369c7f56080c669ec63e1f1f16574e1f51b4f34f3159d382fb12e55d549e0394ca3b5e0576095d2a8ec7e3d06d45cebdc807e9f0dd8
upload failed: - to s3://rust-lang-ci-sccache2/docker/a9e2a06b4251eb289588a369c7f56080c669ec63e1f1f16574e1f51b4f34f3159d382fb12e55d549e0394ca3b5e0576095d2a8ec7e3d06d45cebdc807e9f0dd8 Unable to locate credentials
[CI_JOB_NAME=mingw-check-tidy]
[CI_JOB_NAME=mingw-check-tidy]
---
   Compiling cargo_metadata v0.14.0
   Compiling tidy v0.1.0 (/checkout/src/tools/tidy)
    Finished release [optimized] target(s) in 13.34s
fmt check
Diff in /checkout/compiler/rustc_serialize/src/opaque.rs at line 686:
         let slice = self.reader.as_slice();
         assert!(slice[len] == STR_SENTINEL);
         self.reader.advance_by(len + 1).unwrap();
-        unsafe {
-            std::str::from_utf8_unchecked(&slice[..len])
-        }
+        unsafe { std::str::from_utf8_unchecked(&slice[..len]) }
 
     #[inline]
     #[inline]
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/rustfmt/bin/rustfmt" "--config-path" "/checkout" "--edition" "2021" "--unstable-features" "--skip-children" "--check" "/checkout/compiler/rustc_serialize/src/opaque.rs" "/checkout/compiler/rustc_serialize/src/collection_impls.rs" "/checkout/compiler/rustc_error_codes/src/lib.rs" "/checkout/compiler/rustc_mir_transform/src/const_prop.rs" "/checkout/compiler/rustc_error_codes/src/error_codes.rs" "/checkout/compiler/rustc_const_eval/src/errors.rs" "/checkout/compiler/rustc_const_eval/src/transform/mod.rs" "/checkout/compiler/rustc_serialize/src/serialize.rs"` failed.
Build completed unsuccessfully in 0:00:15
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
