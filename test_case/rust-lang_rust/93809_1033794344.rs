plain
a different dependency.

Run `npm audit` for details.
npm notice 
npm notice New major version of npm available! 7.21.1 -> 8.4.1
npm notice Changelog: <https://github.com/npm/cli/releases/tag/v8.4.1>
npm notice Run `npm install -g npm@8.4.1` to update!
Removing intermediate container 9d2c8a907b8c
 ---> b8b758ff971c
Step 6/12 : RUN npm install eslint@8.6.0 -g
 ---> Running in 157eaa91a13f
---
Successfully built 3dd5ba0bd0c0
Successfully tagged rust-ci:latest
Built container sha256:3dd5ba0bd0c0df1459a8be1a9f75cf8db73e6f998d2dbc37d6f267b3eb2a34cb
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
* 632 error codes
* highest error code: E0790
tidy error: /checkout/src/test/ui/asm/naked-functions-target-feature.rs: too many trailing newlines (2)
`/checkout/compiler/rustc_error_codes/src/error_codes/E0789.md` doesn't use its own error code in compile_fail example
Done!
* 363 features
some tidy checks failed
Build completed unsuccessfully in 0:00:12
