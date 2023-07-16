plain
Successfully built d1e22513add6
Successfully tagged rust-ci:latest
Built container sha256:d1e22513add638cdd4e30a0349f6e412be7873cbc5c11f60e6ebc7f800b4a346
Uploading finished image to https://ci-caches.rust-lang.org/docker/c165c0dbe07f979c65839814f65d46fc3b20640fec5c6162fa0d12eccd662a5501f008e9d9c1953dc5ee940f259fa67db4f7e378e34b63c10ab0f0e1d88ab56a
upload failed: - to s3://rust-lang-ci-sccache2/docker/c165c0dbe07f979c65839814f65d46fc3b20640fec5c6162fa0d12eccd662a5501f008e9d9c1953dc5ee940f259fa67db4f7e378e34b63c10ab0f0e1d88ab56a Unable to locate credentials
[CI_JOB_NAME=mingw-check]
---
configure: rust.debug-assertions := True
configure: rust.overflow-checks := True
configure: llvm.assertions      := True
configure: dist.missing-tools   := True
configure: build.configure-args := ['--enable-sccache', '--disable-manage-submodu ...
configure: writing `config.toml` in current directory
configure: 
configure: run `python /checkout/x.py --help`
configure: 
---
   Compiling thiserror v1.0.30
   Compiling yaml-merge-keys v0.4.1
   Compiling expand-yaml-anchors v0.1.0 (/checkout/src/tools/expand-yaml-anchors)
    Finished release [optimized] target(s) in 5.24s
error: .github/workflows/ci.yml is not up to date; please run `x.py run src/tools/expand-yaml-anchors`.
caused by: src/ci/github-actions/ci.yml and .github/workflows/ci.yml are different
