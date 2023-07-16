plain
Prepare all required actions
Getting action download info
Download action repository 'actions/checkout@v3' (SHA:ac593985615ec2ede58e132d2e21d2b1cbd6127c)
Download action repository 'rust-lang/simpleinfra@master' (SHA:aa723573e04016ede7da6c5d7b029e72cb8a05a3)
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
  Stored in directory: /tmp/pip-ephem-wheel-cache-43lwh8dd/wheels/c2/3c/b9/1120c2ab4bd82694f7e6f0537dc5b9a085c13e2c69a8d0c76d
Installing collected packages: boolean-py, binaryornot, setuptools, reuse, python-debian, markupsafe, license-expression, jinja2, chardet
  Attempting uninstall: setuptools
    Found existing installation: setuptools 59.6.0
    Not uninstalling setuptools at /usr/lib/python3/dist-packages, outside environment /usr
---
Successfully built 0dedfc6177ef
Successfully tagged rust-ci:latest
Built container sha256:0dedfc6177ef51362369551a3b1561ef5b6a41ef7df2ef8900d76dba53a2de9a
Uploading finished image to https://ci-caches.rust-lang.org/docker/f36b5782f7228aca51ac4b42fc8d834a0357048eedd2f8ec11ebf729b3642b4f40d12a04648dc10cace4cb151f5ecc2ac820f64f301d3380602cceb15ce3c48d
upload failed: - to s3://rust-lang-ci-sccache2/docker/f36b5782f7228aca51ac4b42fc8d834a0357048eedd2f8ec11ebf729b3642b4f40d12a04648dc10cace4cb151f5ecc2ac820f64f301d3380602cceb15ce3c48d Unable to locate credentials
[CI_JOB_NAME=mingw-check-tidy]
[CI_JOB_NAME=mingw-check-tidy]
---
Highest error code: `E0793`
* 395 features
fmt check
skip untracked path cpu-usage.csv during rustfmt invocations
Diff in /checkout/src/bootstrap/flags.rs at line 80:
     pub llvm_profile_generate: bool,
     pub llvm_bolt_profile_generate: bool,
     pub llvm_bolt_profile_use: Option<String>,
-    pub llvm_targets: Option<String>
+    pub llvm_targets: Option<String>,
 
 #[derive(Debug)]
 #[derive(Debug)]
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/rustfmt/bin/rustfmt" "--config-path" "/checkout" "--edition" "2021" "--unstable-features" "--skip-children" "--check" "/checkout/src/librustdoc/passes/lint/check_code_block_syntax.rs" "/checkout/src/bootstrap/flags.rs" "/checkout/src/librustdoc/passes/mod.rs" "/checkout/src/librustdoc/passes/check_doc_test_visibility.rs" "/checkout/src/bootstrap/dylib_util.rs" "/checkout/src/librustdoc/passes/calculate_doc_coverage.rs" "/checkout/src/librustdoc/passes/collect_intra_doc_links.rs" "/checkout/src/librustdoc/passes/lint/html_tags.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
