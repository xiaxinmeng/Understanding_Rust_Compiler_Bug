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
  Stored in directory: /tmp/pip-ephem-wheel-cache-37y5mbg4/wheels/c2/3c/b9/1120c2ab4bd82694f7e6f0537dc5b9a085c13e2c69a8d0c76d
Installing collected packages: boolean-py, binaryornot, setuptools, reuse, python-debian, markupsafe, license-expression, jinja2, chardet
  Attempting uninstall: setuptools
    Found existing installation: setuptools 59.6.0
    Not uninstalling setuptools at /usr/lib/python3/dist-packages, outside environment /usr
---
Successfully built 43912d5b3a96
Successfully tagged rust-ci:latest
Built container sha256:43912d5b3a96a72b017c161e1399a5cb88100bd4aebe2ee60187dd099cffa62f
Uploading finished image to https://ci-caches.rust-lang.org/docker/eee721fc1ad06e35509f22d2087fb4acdb0a4d378bec3a191179769e2503f17c7f8f4292d7013976df3a24502e822c590708d80a62f327ca4849564995e4deb7
upload failed: - to s3://rust-lang-ci-sccache2/docker/eee721fc1ad06e35509f22d2087fb4acdb0a4d378bec3a191179769e2503f17c7f8f4292d7013976df3a24502e822c590708d80a62f327ca4849564995e4deb7 Unable to locate credentials
[CI_JOB_NAME=mingw-check-tidy]
[CI_JOB_NAME=mingw-check-tidy]
---
   Compiling cargo_metadata v0.15.3
   Compiling tidy v0.1.0 (/checkout/src/tools/tidy)
    Finished release [optimized] target(s) in 13.35s
fmt check
Diff in /checkout/compiler/rustc_codegen_ssa/src/back/metadata.rs at line 122:
         let n_namsz: u32 = 4; // Size of the n_name field
         let n_descsz: u32 = 16; // Size of the n_desc field
         let n_type: u32 = NT_GNU_PROPERTY_TYPE_0; // Type of note descriptor
-        vec![n_namsz, n_descsz, n_type].iter().for_each(|v| data.extend_from_slice(&(v.to_ne_bytes())));
+        vec![n_namsz, n_descsz, n_type]
+            .iter()
+            .for_each(|v| data.extend_from_slice(&(v.to_ne_bytes())));
         data.push(b'G'); // Owner of the program property note
         data.push(b'N');
         data.push(b'U');
Diff in /checkout/compiler/rustc_codegen_ssa/src/back/metadata.rs at line 131:
         let pr_data: u32 = 3; //program property descriptor
         let pr_padding: u32 = 3;
         vec![pr_type, pr_datasz, pr_data, pr_padding]
-           .iter()
-           .for_each(|v| data.extend_from_slice(&(v.to_ne_bytes())));
+            .iter()
+            .for_each(|v| data.extend_from_slice(&(v.to_ne_bytes())));
         file.append_section_data(section, &data, 4);
 }
Build completed unsuccessfully in 0:00:23
Build completed unsuccessfully in 0:00:23
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/rustfmt/bin/rustfmt" "--config-path" "/checkout" "--edition" "2021" "--unstable-features" "--skip-children" "--check" "/checkout/compiler/rustc_codegen_ssa/src/back/mod.rs" "/checkout/compiler/rustc_codegen_ssa/src/back/rpath.rs" "/checkout/compiler/rustc_codegen_ssa/src/back/symbol_export.rs" "/checkout/compiler/rustc_codegen_ssa/src/back/link.rs" "/checkout/compiler/rustc_codegen_ssa/src/back/linker.rs" "/checkout/compiler/rustc_codegen_ssa/src/back/archive.rs" "/checkout/compiler/rustc_codegen_ssa/src/back/metadata.rs" "/checkout/compiler/rustc_codegen_ssa/src/back/lto.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
