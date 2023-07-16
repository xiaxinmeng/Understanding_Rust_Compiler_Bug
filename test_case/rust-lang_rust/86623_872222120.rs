plain
Successfully built 073407d923b9
Successfully tagged rust-ci:latest
Built container sha256:073407d923b933958b924ab905d0ce86e8c8b37a483806b995223d2457cd9c89
Uploading finished image to https://ci-caches.rust-lang.org/docker/f6fe2abde95d04eb3063dc7a10581b22c9acd1ac716232aeb58e198d801e1d61a8ecf396a3345e3f0ccfd4f5d16368c6dcefc74d7c944f76f74e88c27dfe3021
upload failed: - to s3://rust-lang-ci-sccache2/docker/f6fe2abde95d04eb3063dc7a10581b22c9acd1ac716232aeb58e198d801e1d61a8ecf396a3345e3f0ccfd4f5d16368c6dcefc74d7c944f76f74e88c27dfe3021 Unable to locate credentials
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
