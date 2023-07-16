plain
Successfully built b2a628d380d6
Successfully tagged rust-ci:latest
Built container sha256:b2a628d380d6c4db3d8473faaa4a32ed310998e8a33aaec9045d603599cb8bae
Uploading finished image to https://ci-caches.rust-lang.org/docker/48fdf29bfc4f9026ec96c8a85dff312c9e9ccd9f42556c6958615d3f16066dd80ab877a78ff3faa4828c10a84100780674fc25621e1515a9c0654c68804292fd
upload failed: - to s3://rust-lang-ci-sccache2/docker/48fdf29bfc4f9026ec96c8a85dff312c9e9ccd9f42556c6958615d3f16066dd80ab877a78ff3faa4828c10a84100780674fc25621e1515a9c0654c68804292fd Unable to locate credentials
[CI_JOB_NAME=mingw-check]
---
configure: rust.channel         := nightly
configure: rust.debug-assertions := True
configure: llvm.assertions      := True
configure: dist.missing-tools   := True
configure: build.configure-args := ['--enable-sccache', '--disable-manage-submodu ...
configure: writing `config.toml` in current directory
configure: 
configure: run `python /checkout/x.py --help`
configure: 
