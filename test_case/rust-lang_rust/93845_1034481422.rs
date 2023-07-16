plain
Successfully built c1e19356d3fe
Successfully tagged rust-ci:latest
Built container sha256:c1e19356d3fe2c12f119f0457f849361ea1d5bb12688a543d0dc8029f6550ece
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
* highest error code: E0787
Found 501 error codes
Found 0 error codes with no tests
Done!
tidy error: /checkout/src/test/ui/lifetimes/shadow.rs: too many trailing newlines (2)
tidy error: /checkout/src/test/ui/lifetimes/missing-lifetime-in-alias.rs: too many trailing newlines (2)
tidy error: /checkout/src/test/ui/lifetimes/nested.rs: missing trailing newline
some tidy checks failed
Build completed unsuccessfully in 0:00:12
