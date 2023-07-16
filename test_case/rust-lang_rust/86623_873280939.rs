plain
Successfully built e0e3d492ea8c
Successfully tagged rust-ci:latest
Built container sha256:e0e3d492ea8c2ad755437433807c6cfaeac8bab7ec72ea756572b7ebf17c53a5
Uploading finished image to https://ci-caches.rust-lang.org/docker/acba89d1f437fd486f9ebdfda8bae298d27da1cd52b8ff1e079ad7f0bfaa7d729d0c120e2b3d00b0e6eed03e2e090df28e0708af4adadb6da0e363ba602d1fe1
upload failed: - to s3://rust-lang-ci-sccache2/docker/acba89d1f437fd486f9ebdfda8bae298d27da1cd52b8ff1e079ad7f0bfaa7d729d0c120e2b3d00b0e6eed03e2e090df28e0708af4adadb6da0e363ba602d1fe1 Unable to locate credentials
 * branch              master     -> FETCH_HEAD
[CI_JOB_NAME=mingw-check]
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
---
Build completed successfully in 0:00:37
+ /scripts/validate-toolstate.sh
Cloning into 'rust-toolstate'...
<Nothing changed>
+ /scripts/validate-error-codes.sh
/scripts/validate-error-codes.sh: line 6: DEPLOY_ENV: unbound variable
