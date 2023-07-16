plain
Successfully built c338c87183f0
Successfully tagged rust-ci:latest
Built container sha256:c338c87183f0d02a182e40c4d762dce949092cc01843b02e97671d7b19a524ed
Uploading finished image to https://ci-caches.rust-lang.org/docker/98bfad37442d569a24c9084bf97eaf36aa1186a9185c511020b4ff69ac5fb998b55f873ebe10e96fca9c8deb900cc60699d79e2bee9fa81dfb50723663e93560
upload failed: - to s3://rust-lang-ci-sccache2/docker/98bfad37442d569a24c9084bf97eaf36aa1186a9185c511020b4ff69ac5fb998b55f873ebe10e96fca9c8deb900cc60699d79e2bee9fa81dfb50723663e93560 Unable to locate credentials
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
Build completed successfully in 0:00:35
+ /scripts/validate-toolstate.sh
Cloning into 'rust-toolstate'...
<Nothing changed>
+ /scripts/validate-error-codes.sh
Check if an error code explanation was removed...
Error code explanations should never be removed!
Take a look at E0001 to see how to handle it.
