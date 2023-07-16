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
  Stored in directory: /tmp/pip-ephem-wheel-cache-ye9kz7gn/wheels/c2/3c/b9/1120c2ab4bd82694f7e6f0537dc5b9a085c13e2c69a8d0c76d
Installing collected packages: boolean-py, binaryornot, setuptools, reuse, python-debian, markupsafe, license-expression, jinja2, chardet
  Attempting uninstall: setuptools
    Found existing installation: setuptools 59.6.0
    Not uninstalling setuptools at /usr/lib/python3/dist-packages, outside environment /usr
---
Successfully built 6f4252d6dfb5
Successfully tagged rust-ci:latest
Built container sha256:6f4252d6dfb5984a9f03df79610efc1be1c4b36522966fa08b21227fce6f1fae
Uploading finished image to https://ci-caches.rust-lang.org/docker/f36b5782f7228aca51ac4b42fc8d834a0357048eedd2f8ec11ebf729b3642b4f40d12a04648dc10cace4cb151f5ecc2ac820f64f301d3380602cceb15ce3c48d
upload failed: - to s3://rust-lang-ci-sccache2/docker/f36b5782f7228aca51ac4b42fc8d834a0357048eedd2f8ec11ebf729b3642b4f40d12a04648dc10cace4cb151f5ecc2ac820f64f301d3380602cceb15ce3c48d Unable to locate credentials
[CI_JOB_NAME=mingw-check-tidy]
[CI_JOB_NAME=mingw-check-tidy]
---
Highest error code: `E0793`
* 395 features
fmt check
skip untracked path cpu-usage.csv during rustfmt invocations
Diff in /checkout/compiler/rustc_type_ir/src/sty.rs at line 336:
                 a_d == b_d && a_s == b_s && a_m == b_m
             }
             (GeneratorWitness(a_g), GeneratorWitness(b_g)) => a_g == b_g,
-            (
-                &GeneratorWitnessMIR(ref a_d, ref a_s),
-                &GeneratorWitnessMIR(ref b_d, ref b_s),
-            ) => a_d == b_d && a_s == b_s,
+            (&GeneratorWitnessMIR(ref a_d, ref a_s), &GeneratorWitnessMIR(ref b_d, ref b_s)) => {
+                a_d == b_d && a_s == b_s
+            }
             (Tuple(a_t), Tuple(b_t)) => a_t == b_t,
             (Alias(a_i, a_p), Alias(b_i, b_p)) => a_i == b_i && a_p == b_p,
             (Param(a_p), Param(b_p)) => a_p == b_p,
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/rustfmt/bin/rustfmt" "--config-path" "/checkout" "--edition" "2021" "--unstable-features" "--skip-children" "--check" "/checkout/compiler/rustc_codegen_llvm/src/llvm/mod.rs" "/checkout/compiler/rustc_type_ir/src/lib.rs" "/checkout/compiler/rustc_type_ir/src/codec.rs" "/checkout/compiler/rustc_type_ir/src/ty_info.rs" "/checkout/compiler/rustc_type_ir/src/sty.rs" "/checkout/library/test/src/stats.rs" "/checkout/compiler/rustc_hir_pretty/src/lib.rs" "/checkout/compiler/rustc_codegen_llvm/src/llvm/diagnostic.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
