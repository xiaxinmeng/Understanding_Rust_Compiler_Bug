plain
Successfully built 9e69c9a0f6bc
Successfully tagged rust-ci:latest
Built container sha256:9e69c9a0f6bcea25fbee85812b36112a048cf31e5c09ffa3864e118a7eea6fe5
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
skip untracked path cpu-usage.csv during rustfmt invocations
skip untracked path src/doc/book/ during rustfmt invocations
skip untracked path src/doc/rust-by-example/ during rustfmt invocations
skip untracked path src/llvm-project/ during rustfmt invocations
Diff in /checkout/compiler/rustc_monomorphize/src/partitioning/mod.rs at line 109:
 use rustc_middle::ty::print::with_no_trimmed_paths;
 use rustc_middle::ty::TyCtxt;
 use rustc_middle::ty::TyCtxt;
-use rustc_session::config::{SwitchWithOptPath, DumpMonoStatsFormat};
+use rustc_session::config::{DumpMonoStatsFormat, SwitchWithOptPath};
 
 use crate::collector::InliningMap;
 use crate::collector::InliningMap;
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2021" "--unstable-features" "--skip-children" "--check" "/checkout/compiler/rustc_mir_dataflow/src/un_derefer.rs" "/checkout/compiler/rustc_monomorphize/src/partitioning/merging.rs" "/checkout/compiler/rustc_monomorphize/src/partitioning/default.rs" "/checkout/compiler/rustc_monomorphize/src/partitioning/mod.rs" "/checkout/compiler/rustc_mir_dataflow/src/errors.rs" "/checkout/compiler/rustc_mir_dataflow/src/rustc_peek.rs" "/checkout/compiler/rustc_mir_dataflow/src/impls/storage_liveness.rs" "/checkout/compiler/rustc_monomorphize/src/errors.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
