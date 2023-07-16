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
Successfully built 6dae3dc12fdf
Successfully tagged rust-ci:latest
Built container sha256:6dae3dc12fdfd9f0c98b9358a0b9eef8c6c35bddc015aee05248b8f05054d68d
Uploading finished image to https://ci-caches.rust-lang.org/docker/eee721fc1ad06e35509f22d2087fb4acdb0a4d378bec3a191179769e2503f17c7f8f4292d7013976df3a24502e822c590708d80a62f327ca4849564995e4deb7
upload failed: - to s3://rust-lang-ci-sccache2/docker/eee721fc1ad06e35509f22d2087fb4acdb0a4d378bec3a191179769e2503f17c7f8f4292d7013976df3a24502e822c590708d80a62f327ca4849564995e4deb7 Unable to locate credentials
[CI_JOB_NAME=mingw-check-tidy]
[CI_JOB_NAME=mingw-check-tidy]
---
fmt check
error: prefix `builtin` is unknown
  --> /checkout/library/core/src/mem/offset_of.rs:39:5
   |
39 |     builtin#offset_of($Container, $($fields).+)
   |     ^^^^^^^ unknown prefix
   = note: prefixed identifiers and literals are reserved since Rust 2021
Build completed unsuccessfully in 0:00:22
help: consider inserting whitespace here
   |
   |
39 |     builtin #offset_of($Container, $($fields).+)


Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/rustfmt/bin/rustfmt" "--config-path" "/checkout" "--edition" "2021" "--unstable-features" "--skip-children" "--check" "/checkout/library/core/src/future/poll_fn.rs" "/checkout/library/core/src/ffi/mod.rs" "/checkout/library/core/src/mem/mod.rs" "/checkout/library/core/src/prelude/mod.rs" "/checkout/library/core/src/mem/offset_of.rs" "/checkout/library/core/src/prelude/v1.rs" "/checkout/library/core/src/mem/maybe_uninit.rs" "/checkout/library/core/src/async_iter/from_iter.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
