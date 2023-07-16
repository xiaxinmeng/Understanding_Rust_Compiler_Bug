plain
Successfully built 340bac53c1d3
Successfully tagged rust-ci:latest
Built container sha256:340bac53c1d314aa993546e432d01d64a0e6992532adb5709a9030dee95e547e
Uploading finished image to https://ci-caches.rust-lang.org/docker/b4c226ec1d684c71a4d91fb0cd1a640c97d897266a00433471b3dedeefdb8d321b1df2c8c36c10bd4d6087df0c2b9b507f88d10da95406d6ce945bef9db881e3
upload failed: - to s3://rust-lang-ci-sccache2/docker/b4c226ec1d684c71a4d91fb0cd1a640c97d897266a00433471b3dedeefdb8d321b1df2c8c36c10bd4d6087df0c2b9b507f88d10da95406d6ce945bef9db881e3 Unable to locate credentials
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
* 340 features
some tidy checks failed


command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/rust-tidy" "/checkout" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "/checkout/obj/build" "16"


Build completed unsuccessfully in 0:00:13
