plain
Successfully built 3a70db8da316
Successfully tagged rust-ci:latest
Built container sha256:3a70db8da316f0afe1356960a3806807b39c53dd23adf3fd08e9f957dce42237
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
tidy: Skipping binary file check, read-only filesystem
Checking which error codes lack tests...
* 633 error codes
* highest error code: E0791
thread '<unnamed>' panicked at 'Encountered error while testing Git status: "failed to execute git log command: fatal: ambiguous argument ':!src/tools/miri': unknown revision or path not in the working tree.\nUse '--' to separate paths from revisions, like this:\n'git <command> [<revision>...] -- [<file>...]'\n"', src/tools/tidy/src/no_merge.rs:23:25
Found 507 error codes
Found 0 error(s) in error codes
Done!
thread 'main' panicked at 'called `Result::unwrap()` on an `Err` value: Any { .. }', src/tools/tidy/src/main.rs:42:61
