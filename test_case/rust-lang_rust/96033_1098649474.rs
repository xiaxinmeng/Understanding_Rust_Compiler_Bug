plain
Successfully built 46571496f62e
Successfully tagged rust-ci:latest
Built container sha256:46571496f62e4d3732afabd271be1e76b9e48d2d211e32033f6b6e40e9330193
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
    --> library/core/src/result.rs:1062:9
     |
1062 |       /// 