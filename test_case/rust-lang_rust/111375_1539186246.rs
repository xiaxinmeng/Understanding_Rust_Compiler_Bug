plain
Prepare all required actions
Getting action download info
Download action repository 'actions/checkout@v3' (SHA:8e5e7e5ab8b370d6c329ec480221332ada57f0ab)
Download action repository 'rust-lang/simpleinfra@master' (SHA:af9a3f1b7e12a54c737d8aa371acc8d05cb83a8f)
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
  Stored in directory: /tmp/pip-ephem-wheel-cache-zvs32yyv/wheels/c2/3c/b9/1120c2ab4bd82694f7e6f0537dc5b9a085c13e2c69a8d0c76d
Installing collected packages: boolean-py, binaryornot, setuptools, reuse, python-debian, markupsafe, license-expression, jinja2, chardet
  Attempting uninstall: setuptools
    Found existing installation: setuptools 59.6.0
    Not uninstalling setuptools at /usr/lib/python3/dist-packages, outside environment /usr
---
Successfully tagged rust-ci:latest
Built container sha256:7f7876f43fb1814e69c094fa7109c53166b6dc8ac4cffe9c8118d44561597d5f
Uploading finished image to https://ci-caches.rust-lang.org/docker/e51156f19850ce886cec818e46dc2a021e0aa7c270d15673e8fe74cd8522fc8ac3995109aebb688ee49ed586735ed4cf5f8c06d44c48298fa09c35ae4b082281

<botocore.awsrequest.AWSRequest object at 0x7f8803cf5550>
gzip: stdout: Broken pipe
xargs: docker: terminated by signal 13
[CI_JOB_NAME=mingw-check-tidy]
[CI_JOB_NAME=mingw-check-tidy]
---
   Compiling tidy v0.1.0 (/checkout/src/tools/tidy)
    Finished release [optimized] target(s) in 16.31s
##[endgroup]
fmt check
Diff in /checkout/compiler/rustc_symbol_mangling/src/typeid/typeid_itanium_cxx_abi.rs at line 1052:
                 }
             } else {
             } else {
-                bug!("typeid_for_instance: concrete self isn't a reference {:?}", fn_abi.args[0].layout.ty);
+                bug!(
+                    "typeid_for_instance: concrete self isn't a reference {:?}",
+                    fn_abi.args[0].layout.ty
             };
 
 
             // Replace the concrete self in an fn_abi clone by the reference to a trait object
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/rustfmt/bin/rustfmt" "--config-path" "/checkout" "--edition" "2021" "--unstable-features" "--skip-children" "--check" "/checkout/compiler/rustc_symbol_mangling/src/legacy.rs" "/checkout/compiler/rustc_index/src/vec.rs" "/checkout/compiler/rustc_symbol_mangling/src/typeid/typeid_itanium_cxx_abi.rs" "/checkout/compiler/rustc_symbol_mangling/src/lib.rs" "/checkout/compiler/rustc_index/src/bit_set.rs" "/checkout/compiler/rustc_symbol_mangling/src/errors.rs" "/checkout/compiler/rustc_symbol_mangling/src/test.rs" "/checkout/compiler/rustc_index/src/idx.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
