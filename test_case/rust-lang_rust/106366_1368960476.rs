plain
Successfully built adea30389d0e
Successfully tagged rust-ci:latest
Built container sha256:adea30389d0e72c3c8c184f3c97c43aa44c03bbfa4a8440a10986cb89f01d8e2
Uploading finished image to https://ci-caches.rust-lang.org/docker/dc5e36fabab9a418dcd1388689e70a41ae6bbea7195d4bd033228b0312de39f9e1d1fdc941775cd0a93a74a0110ae5c291a6c8ed0f0bf9425f4848a8035010bf
upload failed: - to s3://rust-lang-ci-sccache2/docker/dc5e36fabab9a418dcd1388689e70a41ae6bbea7195d4bd033228b0312de39f9e1d1fdc941775cd0a93a74a0110ae5c291a6c8ed0f0bf9425f4848a8035010bf Unable to locate credentials
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
