plain
Successfully built 3e9eebdf47e6
Successfully tagged rust-ci:latest
Built container sha256:3e9eebdf47e6266d32ab29b2aa7bbda2d1c7ee659dbf23b09a25ea4371d05b00
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
tidy: Skipping binary file check, read-only filesystem
Checking which error codes lack tests...
* 630 error codes
* highest error code: E0788
tidy error: /checkout/src/test/ui/asm/naked-functions-target-feature.rs: too many trailing newlines (2)
Found 0 error codes with no tests
Done!
* 363 features
some tidy checks failed
