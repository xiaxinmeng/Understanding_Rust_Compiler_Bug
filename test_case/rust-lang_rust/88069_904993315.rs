plain
 ---> dd1bd56da04f
Successfully built dd1bd56da04f
Successfully tagged rust-ci:latest
Built container sha256:dd1bd56da04f7b0d82690f84c9aea9a25c8afbd260c600ffe3239cf9e2f0c334
Uploading finished image to https://ci-caches.rust-lang.org/docker/60c5e17bd4a03c85922034446f11124eedc5158f620bedca5c8e5e94a27a032b243405c87ab9ddabf03298397d94953ae7cfe263eb4add8e55730ab349bcfaa2
upload failed: - to s3://rust-lang-ci-sccache2/docker/60c5e17bd4a03c85922034446f11124eedc5158f620bedca5c8e5e94a27a032b243405c87ab9ddabf03298397d94953ae7cfe263eb4add8e55730ab349bcfaa2 Unable to locate credentials
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
    Finished release [optimized] target(s) in 11.09s
tidy check
tidy: Skipping binary file check, read-only filesystem
Checking which error codes lack tests...
tidy error: /checkout/src/ci/docker/scripts/sccache.sh:6: line longer than 100 chars
* highest error code: E0784
Found 500 error codes
Found 0 error codes with no tests
Done!
Done!
* 347 features
some tidy checks failed


command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/rust-tidy" "/checkout" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "/checkout/obj/build" "16"


Build completed unsuccessfully in 0:00:13
