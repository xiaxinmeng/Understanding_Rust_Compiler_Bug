plain
Successfully built 3b6c825b9799
Successfully tagged rust-ci:latest
Built container sha256:3b6c825b9799bc30f348d39c6ca4570b9feb1f8611b2087d36eab3c6fc2a1baa
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
skip untracked path cpu-usage.csv during rustfmt invocations
skip untracked path src/doc/book/ during rustfmt invocations
skip untracked path src/doc/rust-by-example/ during rustfmt invocations
skip untracked path src/llvm-project/ during rustfmt invocations
Diff in /checkout/library/std/src/sys/unix/process/process_unix.rs at line 80:
                     libc::_exit(1)
                 })) {
                     Err(_) => crate::process::abort(),
+                },
                 n => n,
             }
         };
         };
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2018" "--unstable-features" "--skip-children" "--check" "/checkout/library/std/src/sys/unix/process/process_unix.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
Build completed unsuccessfully in 0:00:22
