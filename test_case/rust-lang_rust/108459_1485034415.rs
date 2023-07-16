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
  Stored in directory: /tmp/pip-ephem-wheel-cache-do9dl_gh/wheels/c2/3c/b9/1120c2ab4bd82694f7e6f0537dc5b9a085c13e2c69a8d0c76d
Installing collected packages: boolean-py, binaryornot, setuptools, reuse, python-debian, markupsafe, license-expression, jinja2, chardet
  Attempting uninstall: setuptools
    Found existing installation: setuptools 59.6.0
    Not uninstalling setuptools at /usr/lib/python3/dist-packages, outside environment /usr
---
Successfully built b81dc596b53f
Successfully tagged rust-ci:latest
Built container sha256:b81dc596b53fef6dd5f45aa068eb6de090d9497be857280ab7355f7cfba26808
Uploading finished image to https://ci-caches.rust-lang.org/docker/a9e2a06b4251eb289588a369c7f56080c669ec63e1f1f16574e1f51b4f34f3159d382fb12e55d549e0394ca3b5e0576095d2a8ec7e3d06d45cebdc807e9f0dd8
upload failed: - to s3://rust-lang-ci-sccache2/docker/a9e2a06b4251eb289588a369c7f56080c669ec63e1f1f16574e1f51b4f34f3159d382fb12e55d549e0394ca3b5e0576095d2a8ec7e3d06d45cebdc807e9f0dd8 Unable to locate credentials
[CI_JOB_NAME=mingw-check-tidy]
[CI_JOB_NAME=mingw-check-tidy]
---
   Compiling cargo_metadata v0.14.0
   Compiling tidy v0.1.0 (/checkout/src/tools/tidy)
    Finished release [optimized] target(s) in 12.84s
fmt check
Diff in /checkout/src/librustdoc/html/markdown.rs at line 406:
                 trace!("saw text {}", text);
                 if let Some(link) = self.shortcut_link {
                     // NOTE: same limitations as `Event::Code`
-                    if let Some(link) =
-                        self.links.iter().find(|l| l.href == link.href && **text == *l.original_text)
+                    if let Some(link) = self
+                        .links
+                        .iter()
+                        .find(|l| l.href == link.href && **text == *l.original_text)
                     {
                         debug!("replacing {} with {}", text, link.new_text);
                         *text = CowStr::Borrowed(&link.new_text);
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/rustfmt/bin/rustfmt" "--config-path" "/checkout" "--edition" "2021" "--unstable-features" "--skip-children" "--check" "/checkout/src/librustdoc/html/url_parts_builder/tests.rs" "/checkout/src/librustdoc/html/toc/tests.rs" "/checkout/src/librustdoc/html/length_limit/tests.rs" "/checkout/src/librustdoc/html/highlight/tests.rs" "/checkout/src/librustdoc/html/url_parts_builder.rs" "/checkout/src/librustdoc/html/markdown/tests.rs" "/checkout/src/librustdoc/html/render/sidebar.rs" "/checkout/src/librustdoc/html/markdown.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
