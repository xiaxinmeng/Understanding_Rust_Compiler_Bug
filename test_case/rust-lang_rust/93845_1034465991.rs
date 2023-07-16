plain
Successfully built 3fbaa1220beb
Successfully tagged rust-ci:latest
Built container sha256:3fbaa1220beb086a8f129d9717a7bee7dcd7d58a88061d27a67f20e8a6ac75ac
Uploading finished image to https://ci-caches.rust-lang.org/docker/9567406ca397026e6d08b99ab2979859301d063de191c2500807ab84c19e827d07128981858917062997c4fe33329efa040c86f471172809b1d85076deabb2ef
upload failed: - to s3://rust-lang-ci-sccache2/docker/9567406ca397026e6d08b99ab2979859301d063de191c2500807ab84c19e827d07128981858917062997c4fe33329efa040c86f471172809b1d85076deabb2ef Unable to locate credentials
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
<Nothing changed>
+ /scripts/validate-error-codes.sh
Check if an error code explanation was removed...
Error code explanations should never be removed!
Take a look at E0001 to see how to handle it.
