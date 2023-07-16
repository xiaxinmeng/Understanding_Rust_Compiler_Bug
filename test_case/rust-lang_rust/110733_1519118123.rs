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
  Stored in directory: /tmp/pip-ephem-wheel-cache-hfwl8nwj/wheels/c2/3c/b9/1120c2ab4bd82694f7e6f0537dc5b9a085c13e2c69a8d0c76d
Installing collected packages: boolean-py, binaryornot, setuptools, reuse, python-debian, markupsafe, license-expression, jinja2, chardet
  Attempting uninstall: setuptools
    Found existing installation: setuptools 59.6.0
    Not uninstalling setuptools at /usr/lib/python3/dist-packages, outside environment /usr
---
Successfully built bddf55afeb2c
Successfully tagged rust-ci:latest
Built container sha256:bddf55afeb2c7179fdeb1ad46acdded0f47e8ad2488277528b32205887d40097
Uploading finished image to https://ci-caches.rust-lang.org/docker/eee721fc1ad06e35509f22d2087fb4acdb0a4d378bec3a191179769e2503f17c7f8f4292d7013976df3a24502e822c590708d80a62f327ca4849564995e4deb7
upload failed: - to s3://rust-lang-ci-sccache2/docker/eee721fc1ad06e35509f22d2087fb4acdb0a4d378bec3a191179769e2503f17c7f8f4292d7013976df3a24502e822c590708d80a62f327ca4849564995e4deb7 Unable to locate credentials
[CI_JOB_NAME=mingw-check-tidy]
[CI_JOB_NAME=mingw-check-tidy]
---
   Compiling cargo_metadata v0.15.3
   Compiling tidy v0.1.0 (/checkout/src/tools/tidy)
    Finished release [optimized] target(s) in 13.28s
fmt check
Diff in /checkout/library/core/src/prelude/v1.rs at line 77:
 // (no public module for them to be re-exported from).
 #[stable(feature = "builtin_macro_prelude", since = "1.38.0")]
 #[allow(deprecated)]
-pub use crate::macros::builtin::{alloc_error_handler, bench, derive, global_allocator, test, test_case};
+pub use crate::macros::builtin::{
+    alloc_error_handler, bench, derive, global_allocator, test, test_case,
 
 #[unstable(feature = "derive_const", issue = "none")]
 pub use crate::macros::builtin::derive_const;
Build completed unsuccessfully in 0:00:23
Build completed unsuccessfully in 0:00:23
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/rustfmt/bin/rustfmt" "--config-path" "/checkout" "--edition" "2021" "--unstable-features" "--skip-children" "--check" "/checkout/library/core/src/mem/manually_drop.rs" "/checkout/library/core/benches/fmt.rs" "/checkout/library/core/src/mem/mod.rs" "/checkout/library/core/src/mem/transmutability.rs" "/checkout/library/core/src/default.rs" "/checkout/library/core/benches/ascii/is_ascii.rs" "/checkout/library/core/src/prelude/v1.rs" "/checkout/library/core/src/mem/maybe_uninit.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
