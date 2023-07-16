plain
Prepare all required actions
Getting action download info
Download action repository 'actions/checkout@v3' (SHA:8f4b7f84864484a7bf31766abe9204da3cbe65b3)
Download action repository 'rust-lang/simpleinfra@master' (SHA:13c1b4e09b845ddb9664cee13d03879444a1054d)
Complete job name: PR (mingw-check-tidy, true, ubuntu-20.04-16core-64gb)
git config --global core.autocrlf false
shell: /usr/bin/bash --noprofile --norc -e -o pipefail {0}
env:
  CI_JOB_NAME: mingw-check-tidy
---
Building wheels for collected packages: reuse
  Building wheel for reuse (pyproject.toml): started
  Building wheel for reuse (pyproject.toml): finished with status 'done'
  Created wheel for reuse: filename=reuse-1.1.0-cp310-cp310-manylinux_2_35_x86_64.whl size=180119 sha256=9fa76c45f3193f307e02ea67d1a48cfe7ef2b854a074b766938a88e046bc7887
  Stored in directory: /tmp/pip-ephem-wheel-cache-asn9ey2r/wheels/c2/3c/b9/1120c2ab4bd82694f7e6f0537dc5b9a085c13e2c69a8d0c76d
Installing collected packages: boolean-py, binaryornot, setuptools, reuse, python-debian, markupsafe, license-expression, jinja2, chardet
  Attempting uninstall: setuptools
    Found existing installation: setuptools 59.6.0
    Not uninstalling setuptools at /usr/lib/python3/dist-packages, outside environment /usr
---
Successfully built 2cd004e1a023
Successfully tagged rust-ci:latest
Built container sha256:2cd004e1a023a6da62a37b1f2e29e9e03270e4291a4f9eb7b5477c52fae5de59
Uploading finished image to https://ci-caches.rust-lang.org/docker/a9e2a06b4251eb289588a369c7f56080c669ec63e1f1f16574e1f51b4f34f3159d382fb12e55d549e0394ca3b5e0576095d2a8ec7e3d06d45cebdc807e9f0dd8
upload failed: - to s3://rust-lang-ci-sccache2/docker/a9e2a06b4251eb289588a369c7f56080c669ec63e1f1f16574e1f51b4f34f3159d382fb12e55d549e0394ca3b5e0576095d2a8ec7e3d06d45cebdc807e9f0dd8 Unable to locate credentials
[CI_JOB_NAME=mingw-check-tidy]
[CI_JOB_NAME=mingw-check-tidy]
---
   Compiling cargo_metadata v0.14.0
   Compiling tidy v0.1.0 (/checkout/src/tools/tidy)
    Finished release [optimized] target(s) in 13.15s
fmt check
Diff in /checkout/src/bootstrap/test.rs at line 23:
 use crate::render_tests::add_flags_and_try_run_tests;
 use crate::tool::{self, SourceType, Tool};
 use crate::toolstate::ToolState;
+use crate::util::up_to_date;
 use crate::util::{self, add_link_lib_path, dylib_path, dylib_path_var, output, t};
 use crate::{envify, CLang, DocTests, GitRepo, Mode};
-use crate::util::up_to_date;
 
 const ADB_TEST_DIR: &str = "/data/local/tmp/work";
Diff in /checkout/src/bootstrap/test.rs at line 2813:
Diff in /checkout/src/bootstrap/test.rs at line 2813:
             .compile("rust_test_helpers");
 }
+
+
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/rustfmt/bin/rustfmt" "--config-path" "/checkout" "--edition" "2021" "--unstable-features" "--skip-children" "--check" "/checkout/compiler/rustc_span/src/source_map.rs" "/checkout/compiler/rustc_span/src/tests.rs" "/checkout/compiler/rustc_span/src/profiling.rs" "/checkout/compiler/rustc_span/src/analyze_source_file.rs" "/checkout/compiler/rustc_span/src/edit_distance/tests.rs" "/checkout/compiler/rustc_span/src/edition.rs" "/checkout/compiler/rustc_span/src/hygiene.rs" "/checkout/src/bootstrap/test.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
