plain
Prepare all required actions
Getting action download info
Download action repository 'actions/checkout@v3' (SHA:24cb9080177205b6e8c946b17badbe402adc938f)
Download action repository 'rust-lang/simpleinfra@master' (SHA:3fb2b44a4eaebb9ed8086446bde46c27199ef5ed)
Complete job name: PR (mingw-check-tidy, true, ubuntu-20.04-xl)
git config --global core.autocrlf false
shell: /usr/bin/bash --noprofile --norc -e -o pipefail {0}
env:
  CI_JOB_NAME: mingw-check-tidy
---
Building wheels for collected packages: reuse
  Building wheel for reuse (pyproject.toml): started
  Building wheel for reuse (pyproject.toml): finished with status 'done'
  Created wheel for reuse: filename=reuse-1.1.0-cp310-cp310-manylinux_2_35_x86_64.whl size=180119 sha256=9fa76c45f3193f307e02ea67d1a48cfe7ef2b854a074b766938a88e046bc7887
  Stored in directory: /tmp/pip-ephem-wheel-cache-6f7u8q73/wheels/c2/3c/b9/1120c2ab4bd82694f7e6f0537dc5b9a085c13e2c69a8d0c76d
Installing collected packages: boolean-py, binaryornot, setuptools, reuse, python-debian, markupsafe, license-expression, jinja2, chardet
  Attempting uninstall: setuptools
    Found existing installation: setuptools 59.6.0
    Not uninstalling setuptools at /usr/lib/python3/dist-packages, outside environment /usr
---
Successfully built f932c217d387
Successfully tagged rust-ci:latest
Built container sha256:f932c217d387795ea709b4eb5d2e7189f8f7741a793e5079e22d3e18566dc168
Uploading finished image to https://ci-caches.rust-lang.org/docker/38eba79578bd6d379f830cfbe873c54dad8d10db1dfcf4fbac8af27e25e93a7ecaa2a7c62028c216d50741db91fde3d04f83489b4d576287b0239d35dee6e197
upload failed: - to s3://rust-lang-ci-sccache2/docker/38eba79578bd6d379f830cfbe873c54dad8d10db1dfcf4fbac8af27e25e93a7ecaa2a7c62028c216d50741db91fde3d04f83489b4d576287b0239d35dee6e197 Unable to locate credentials
[CI_JOB_NAME=mingw-check-tidy]
[CI_JOB_NAME=mingw-check-tidy]
---
   Compiling cargo_metadata v0.14.0
   Compiling tidy v0.1.0 (/checkout/src/tools/tidy)
    Finished release [optimized] target(s) in 17.37s
fmt check
Diff in /checkout/src/librustdoc/html/tests.rs at line 51:
 #[test]
 fn test_html_remover() {
-    use std::fmt::Write;
     use super::format::HtmlRemover;
     use super::format::HtmlRemover;
+    use std::fmt::Write;
 
     fn assert_removed_eq(input: &str, output: &str) {
         let mut remover = HtmlRemover::new(String::new());
Diff in /checkout/src/librustdoc/html/tests.rs at line 84:
     let d = Plain("<strong>alpha</strong> &lt;bet&gt;");
     assert_eq!(&d.to_string(), "alpha <bet>");
+
+
Diff in /checkout/src/librustdoc/html/format.rs at line 1623:
 
     WithFormatter(Cell::new(Some(f)))
+
+
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/rustfmt/bin/rustfmt" "--config-path" "/checkout" "--edition" "2021" "--unstable-features" "--skip-children" "--check" "/checkout/src/librustdoc/externalfiles.rs" "/checkout/src/librustdoc/config.rs" "/checkout/src/librustdoc/html/tests.rs" "/checkout/src/librustdoc/html/escape.rs" "/checkout/src/librustdoc/html/mod.rs" "/checkout/src/librustdoc/html/layout.rs" "/checkout/src/librustdoc/html/static_files.rs" "/checkout/src/librustdoc/visit_lib.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
