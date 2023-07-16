plain
Successfully built 1013c55bd866
Successfully tagged rust-ci:latest
Built container sha256:1013c55bd866513c20961a72c96c8d3cbc7f0ba286d6344fdcbbdfdbb1a0d285
Uploading finished image to https://ci-caches.rust-lang.org/docker/80afe504501370b4d310121e20e04a989f302196b07831c4375b96e05bc067556c2046e20ab2062b28a9dc9b2ae132b37d419cc55a065dfcd25501527e829ab9
upload failed: - to s3://rust-lang-ci-sccache2/docker/80afe504501370b4d310121e20e04a989f302196b07831c4375b96e05bc067556c2046e20ab2062b28a9dc9b2ae132b37d419cc55a065dfcd25501527e829ab9 Unable to locate credentials
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
Checking which error codes lack tests...
Found 435 error codes
Found 0 error codes with no tests
Done!
tidy error: /checkout/src/test/ui/const-generics/const_evaluatable_checked/nested_uneval_unification-2.rs:28: line longer than 100 chars
tidy error: /checkout/src/test/ui/const-generics/const_evaluatable_checked/nested_uneval_unification-1.rs:34: line longer than 100 chars
tidy error: /checkout/src/test/ui/const-generics/const_evaluatable_checked/nested-abstract-consts-2.rs:34: line longer than 100 chars
some tidy checks failed

command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "/checkout/obj/build"
expected success, got: exit code: 1

