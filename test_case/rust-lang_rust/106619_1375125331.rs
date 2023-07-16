plain
Successfully built 9b49b2ca6b94
Successfully tagged rust-ci:latest
Built container sha256:9b49b2ca6b946864bb570b25303fc562c3d24e929f0d8f233256d6c55e1adc21
Uploading finished image to https://ci-caches.rust-lang.org/docker/b8cdc76b9b2adf6aaac1172d56fab8a31470587d5348a0014d9e27d5bcbbe8577c6dad29501e9359634fe46a53fc6079832ca8161c7036f56b2737f782f779b6
upload failed: - to s3://rust-lang-ci-sccache2/docker/b8cdc76b9b2adf6aaac1172d56fab8a31470587d5348a0014d9e27d5bcbbe8577c6dad29501e9359634fe46a53fc6079832ca8161c7036f56b2737f782f779b6 Unable to locate credentials
[CI_JOB_NAME=mingw-check-tidy]
[CI_JOB_NAME=mingw-check-tidy]
---
configure: rust.debug-assertions := True
configure: rust.overflow-checks := True
configure: llvm.assertions      := True
configure: dist.missing-tools   := True
configure: build.configure-args := ['--enable-sccache', '--disable-manage-submodu ...
configure: writing `config.toml` in current directory
configure: 
configure: run `python /checkout/x.py --help`
Attempting with retry: make prepare
---
DirectMap1G:    52428800 kB
Building rustbuild
    Finished dev [unoptimized] target(s) in 0.06s
Building stage0 tool tidy (x86_64-unknown-linux-gnu)
    Updating git repository `https://github.com/agausmann/object.git`
    Updating git submodule `https://github.com/gimli-rs/object-testfiles`
---
   Compiling cargo_metadata v0.14.0
   Compiling tidy v0.1.0 (/checkout/src/tools/tidy)
    Finished release [optimized] target(s) in 20.08s
tidy check
tidy error: invalid source: "git+https://github.com/agausmann/object.git?branch=0.29.0-avr-flags#91e59da7255daf2bb02be6c34a82d60a8ee98251"
Found 505 error codes
Highest error code: `E0791`
* 392 features
some tidy checks failed
