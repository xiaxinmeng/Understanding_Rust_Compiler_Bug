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
  Stored in directory: /tmp/pip-ephem-wheel-cache-uu8511gx/wheels/c2/3c/b9/1120c2ab4bd82694f7e6f0537dc5b9a085c13e2c69a8d0c76d
Installing collected packages: boolean-py, binaryornot, setuptools, reuse, python-debian, markupsafe, license-expression, jinja2, chardet
  Attempting uninstall: setuptools
    Found existing installation: setuptools 59.6.0
    Not uninstalling setuptools at /usr/lib/python3/dist-packages, outside environment /usr
---
Successfully built 4ce25b117119
Successfully tagged rust-ci:latest
Built container sha256:4ce25b1171194fc8d369d943770f30b736b576b63f6813c7909e907fd133c265
Uploading finished image to https://ci-caches.rust-lang.org/docker/38eba79578bd6d379f830cfbe873c54dad8d10db1dfcf4fbac8af27e25e93a7ecaa2a7c62028c216d50741db91fde3d04f83489b4d576287b0239d35dee6e197
upload failed: - to s3://rust-lang-ci-sccache2/docker/38eba79578bd6d379f830cfbe873c54dad8d10db1dfcf4fbac8af27e25e93a7ecaa2a7c62028c216d50741db91fde3d04f83489b4d576287b0239d35dee6e197 Unable to locate credentials
[CI_JOB_NAME=mingw-check-tidy]
[CI_JOB_NAME=mingw-check-tidy]
---
Diff in /checkout/src/tools/tidy/src/deps.rs at line 572:
     }
 }
 
-fn direct_deps_of<'a>(metadata: &'a Metadata, pkg_id: &'a PackageId) -> impl Iterator<Item = &'a Package> {
+fn direct_deps_of<'a>(
+    metadata: &'a Metadata,
+    pkg_id: &'a PackageId,
+) -> impl Iterator<Item = &'a Package> {
     let resolve = metadata.resolve.as_ref().unwrap();
     let node = resolve.nodes.iter().find(|n| &n.id == pkg_id).unwrap();
     node.deps.iter().map(|dep| pkg_from_id(metadata, &dep.pkg))
Diff in /checkout/src/tools/tidy/src/deps.rs at line 580:
 
 fn check_rustfix(rust_metadata: &Metadata, cargo_metadata: &Metadata, bad: &mut bool) {
     let cargo = pkg_from_name(cargo_metadata, "cargo");
-    let cargo_rustfix = direct_deps_of(cargo_metadata, &cargo.id).find(|p| p.name == "rustfix").unwrap();
+    let cargo_rustfix =
+        direct_deps_of(cargo_metadata, &cargo.id).find(|p| p.name == "rustfix").unwrap();
 
     let compiletest = pkg_from_name(rust_metadata, "compiletest");
-    let compiletest_rustfix = direct_deps_of(rust_metadata, &compiletest.id).find(|p| p.name == "rustfix").unwrap();
+    let compiletest_rustfix =
+        direct_deps_of(rust_metadata, &compiletest.id).find(|p| p.name == "rustfix").unwrap();
 
     if cargo_rustfix.version != compiletest_rustfix.version {
         tidy_error!(
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/rustfmt/bin/rustfmt" "--config-path" "/checkout" "--edition" "2021" "--unstable-features" "--skip-children" "--check" "/checkout/src/tools/tidy/src/deps.rs" "/checkout/src/tools/tidy/src/pal.rs" "/checkout/src/tools/jsondoclint/src/main.rs" "/checkout/src/tools/tidy/src/bins.rs" "/checkout/src/tools/tidy/src/x_version.rs" "/checkout/src/tools/tidy/src/features.rs" "/checkout/src/tools/tidy/src/mir_opt_tests.rs" "/checkout/src/tools/tidy/src/error_codes.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
