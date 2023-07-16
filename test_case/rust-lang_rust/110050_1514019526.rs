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
  Stored in directory: /tmp/pip-ephem-wheel-cache-wawrlma2/wheels/c2/3c/b9/1120c2ab4bd82694f7e6f0537dc5b9a085c13e2c69a8d0c76d
Installing collected packages: boolean-py, binaryornot, setuptools, reuse, python-debian, markupsafe, license-expression, jinja2, chardet
  Attempting uninstall: setuptools
    Found existing installation: setuptools 59.6.0
    Not uninstalling setuptools at /usr/lib/python3/dist-packages, outside environment /usr
---
Successfully built 93ea2f4f3ae2
Successfully tagged rust-ci:latest
Built container sha256:93ea2f4f3ae238746691cb77fac8912fadd6a794f3758baeae6d157c9125999d
Uploading finished image to https://ci-caches.rust-lang.org/docker/5a83b1174027be8472df586f36f1b4e77c39a20882cc84831c17b85c3cc5731b4e272c71c37dd18e09cacea39bf89c1b3226a304723983ec1270656c20627ec9
upload failed: - to s3://rust-lang-ci-sccache2/docker/5a83b1174027be8472df586f36f1b4e77c39a20882cc84831c17b85c3cc5731b4e272c71c37dd18e09cacea39bf89c1b3226a304723983ec1270656c20627ec9 Unable to locate credentials
[CI_JOB_NAME=mingw-check-tidy]
[CI_JOB_NAME=mingw-check-tidy]
---
######################################################################## 100.0%
extracting /checkout/obj/build/cache/llvm-b3f13795093c3c6473fb88910947bf6a35bedb26-true/rust-dev-nightly-x86_64-unknown-linux-gnu.tar.xz to /checkout/obj/build/x86_64-unknown-linux-gnu/ci-llvm
Building tool tidy (stage0)
fmt check
Diff in /checkout/compiler/rustc_serialize/src/opaque.rs at line 648:
         let bytes: [u8; 4] =
             self.data.get(self.position..self.position + 4).unwrap().try_into().unwrap();
         u32::from_le_bytes(bytes)
+        /*
         let bytes: [u8; 8] =
         let bytes: [u8; 8] =
             self.data.get(self.position..self.position + 8).unwrap().try_into().unwrap();
         let len = (bytes[0] & 0x3) as usize + 1;
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/rustfmt/bin/rustfmt" "--config-path" "/checkout" "--edition" "2021" "--unstable-features" "--skip-children" "--check" "/checkout/compiler/rustc_codegen_llvm/src/type_of.rs" "/checkout/compiler/rustc_codegen_llvm/src/llvm_util.rs" "/checkout/compiler/rustc_codegen_llvm/src/mono_item.rs" "/checkout/compiler/rustc_serialize/src/collection_impls.rs" "/checkout/compiler/rustc_serialize/src/opaque.rs" "/checkout/compiler/rustc_serialize/src/lib.rs" "/checkout/compiler/rustc_serialize/src/leb128.rs" "/checkout/compiler/rustc_codegen_llvm/src/consts.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
