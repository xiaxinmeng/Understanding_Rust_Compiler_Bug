plain
Successfully built 31453ee7c394
Successfully tagged rust-ci:latest
Built container sha256:31453ee7c394238099b623e41b2b6021e2a9567c08efed9bb7be83e9faa40cad
Uploading finished image to https://ci-caches.rust-lang.org/docker/d33e4deadd55ac310bf6d14046967e2ae76df1d0cbcfb3cbdfd57a515110e5c43bea0fc68be40129a2ca62a8c3c0cddee210b82b000006fe1b3d251a4634baf3
upload failed: - to s3://rust-lang-ci-sccache2/docker/d33e4deadd55ac310bf6d14046967e2ae76df1d0cbcfb3cbdfd57a515110e5c43bea0fc68be40129a2ca62a8c3c0cddee210b82b000006fe1b3d251a4634baf3 Unable to locate credentials
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
Highest error code: `E0792`
* 394 features
fmt check
skip untracked path cpu-usage.csv during rustfmt invocations
Diff in /checkout/library/core/src/future/join.rs at line 4:
 use crate::future::{poll_fn, Future};
 use crate::mem;
 use crate::pin::Pin;
-use crate::task::{Context, Poll, ready};
+use crate::task::{ready, Context, Poll};
 
 /// Polls multiple futures simultaneously, returning a tuple
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2021" "--unstable-features" "--skip-children" "--check" "/checkout/library/core/src/borrow.rs" "/checkout/library/core/src/future/ready.rs" "/checkout/library/core/src/future/join.rs" "/checkout/library/core/src/future/poll_fn.rs" "/checkout/library/core/src/future/future.rs" "/checkout/library/core/src/future/pending.rs" "/checkout/library/core/src/future/mod.rs" "/checkout/library/core/src/future/into_future.rs"` failed.
 /// of all results once complete.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
