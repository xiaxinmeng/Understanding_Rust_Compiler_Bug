plain
Successfully built 8c0b3a0beb15
Successfully tagged rust-ci:latest
Built container sha256:8c0b3a0beb156b7a74d630d8ec71b9325062ed0475f267fd610c6df65d8a0b16
Uploading finished image to https://ci-caches.rust-lang.org/docker/58bd69d3781a382eb5941028d9596b84692bd50bf9ab53a2d16b350022deca32f83037b6d64da3bfdf69e2b275e0db2ddd3212390c690549c0b3382192828b45
upload failed: - to s3://rust-lang-ci-sccache2/docker/58bd69d3781a382eb5941028d9596b84692bd50bf9ab53a2d16b350022deca32f83037b6d64da3bfdf69e2b275e0db2ddd3212390c690549c0b3382192828b45 Unable to locate credentials
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
Found 507 error codes
Found 0 error(s) in error codes
Done!
* 392 features
tidy error: /checkout/src/bootstrap/bootstrap_test.py:79: line longer than 100 chars
tidy error: /checkout/src/bootstrap/bootstrap.py:635: line longer than 100 chars
some tidy checks failed
