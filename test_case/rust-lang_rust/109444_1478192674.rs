plain
Prepare all required actions
Getting action download info
Download action repository 'actions/checkout@v3' (SHA:24cb9080177205b6e8c946b17badbe402adc938f)
Download action repository 'rust-lang/simpleinfra@master' (SHA:5f3e9487b084c5235556ffd8baa8b183de9eb120)
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
  Stored in directory: /tmp/pip-ephem-wheel-cache-qsy3jghu/wheels/c2/3c/b9/1120c2ab4bd82694f7e6f0537dc5b9a085c13e2c69a8d0c76d
Installing collected packages: boolean-py, binaryornot, setuptools, reuse, python-debian, markupsafe, license-expression, jinja2, chardet
  Attempting uninstall: setuptools
    Found existing installation: setuptools 59.6.0
    Not uninstalling setuptools at /usr/lib/python3/dist-packages, outside environment /usr
---
Successfully built 46be838c2853
Successfully tagged rust-ci:latest
Built container sha256:46be838c28539356971b29f63641df8336f313de46fd1eb2cb76a39c29166ad1
Uploading finished image to https://ci-caches.rust-lang.org/docker/a9e2a06b4251eb289588a369c7f56080c669ec63e1f1f16574e1f51b4f34f3159d382fb12e55d549e0394ca3b5e0576095d2a8ec7e3d06d45cebdc807e9f0dd8
upload failed: - to s3://rust-lang-ci-sccache2/docker/a9e2a06b4251eb289588a369c7f56080c669ec63e1f1f16574e1f51b4f34f3159d382fb12e55d549e0394ca3b5e0576095d2a8ec7e3d06d45cebdc807e9f0dd8 Unable to locate credentials
[CI_JOB_NAME=mingw-check-tidy]
[CI_JOB_NAME=mingw-check-tidy]
---
    Finished release [optimized] target(s) in 13.04s
fmt check
Diff in /checkout/src/tools/tidy/src/style.rs at line 385:
             }
             if filename != "style.rs" {
                 if trimmed.contains("TODO") {
-                    err("TODO is used for tasks that should be done before merging a PR; If you want to leave message in the codebase use FIXME")
+                    err(
+                        "TODO is used for tasks that should be done before merging a PR; If you want to leave message in the codebase use FIXME",
                 }
                 }
                 if trimmed.contains("//") && trimmed.contains(" XXX") {
                     err("Instead of XXX use FIXME or TODO")
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/rustfmt/bin/rustfmt" "--config-path" "/checkout" "--edition" "2021" "--unstable-features" "--skip-children" "--check" "/checkout/src/tools/tidy/src/main.rs" "/checkout/src/tools/tidy/src/extdeps.rs" "/checkout/src/tools/tidy/src/style.rs" "/checkout/src/tools/tidy/src/rustdoc_gui_tests.rs" "/checkout/src/tools/jsondoclint/src/json_find.rs" "/checkout/src/tools/tidy/src/deps.rs" "/checkout/src/tools/tidy/src/features/tests.rs" "/checkout/src/tools/tidy/src/alphabetical.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
