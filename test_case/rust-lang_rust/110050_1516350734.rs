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
  Stored in directory: /tmp/pip-ephem-wheel-cache-ds590o2q/wheels/c2/3c/b9/1120c2ab4bd82694f7e6f0537dc5b9a085c13e2c69a8d0c76d
Installing collected packages: boolean-py, binaryornot, setuptools, reuse, python-debian, markupsafe, license-expression, jinja2, chardet
  Attempting uninstall: setuptools
    Found existing installation: setuptools 59.6.0
    Not uninstalling setuptools at /usr/lib/python3/dist-packages, outside environment /usr
---
Successfully built e47f2b1dc7f3
Successfully tagged rust-ci:latest
Built container sha256:e47f2b1dc7f34e9204711b3e7cc79bdeaac6fa2c477d51740e7dce9da2b73476
Uploading finished image to https://ci-caches.rust-lang.org/docker/eee721fc1ad06e35509f22d2087fb4acdb0a4d378bec3a191179769e2503f17c7f8f4292d7013976df3a24502e822c590708d80a62f327ca4849564995e4deb7
upload failed: - to s3://rust-lang-ci-sccache2/docker/eee721fc1ad06e35509f22d2087fb4acdb0a4d378bec3a191179769e2503f17c7f8f4292d7013976df3a24502e822c590708d80a62f327ca4849564995e4deb7 Unable to locate credentials
[CI_JOB_NAME=mingw-check-tidy]
[CI_JOB_NAME=mingw-check-tidy]
---
   Compiling cargo_metadata v0.15.3
   Compiling tidy v0.1.0 (/checkout/src/tools/tidy)
    Finished release [optimized] target(s) in 13.05s
fmt check
Diff in /checkout/compiler/rustc_serialize/src/serialize.rs at line 63:
             v |= 0b1;
         }
         let all_bytes = v.to_le_bytes();
-        let bytes = if uses_last_byte {
-            &all_bytes[..]
-            &all_bytes[..3]
-        };
-        };
+        let bytes = if uses_last_byte { &all_bytes[..] } else { &all_bytes[..3] };
         self.emit_raw_bytes(bytes)
 }
 }
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/rustfmt/bin/rustfmt" "--config-path" "/checkout" "--edition" "2021" "--unstable-features" "--skip-children" "--check" "/checkout/compiler/rustc_serialize/src/serialize.rs" "/checkout/compiler/rustc_index/src/bit_set.rs" "/checkout/compiler/rustc_baked_icu_data/src/lib.rs" "/checkout/compiler/rustc_plugin_impl/src/lib.rs" "/checkout/compiler/rustc_plugin_impl/src/errors.rs" "/checkout/compiler/rustc_plugin_impl/src/load.rs" "/checkout/src/tools/bump-stage0/src/main.rs" "/checkout/compiler/rustc_index/src/interval/tests.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
