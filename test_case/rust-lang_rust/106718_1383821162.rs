plain
Successfully built 043d9489659d
Successfully tagged rust-ci:latest
Built container sha256:043d9489659d3b517bcb7aca3aa7384c27e8baa63c9f8761fc2ddf0f7e247ba3
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
Highest error code: `E0791`
* 394 features
fmt check
skip untracked path cpu-usage.csv during rustfmt invocations
Diff in /checkout/compiler/rustc_trait_selection/src/solve/cache.rs at line 12:
 use super::{CanonicalGoal, Certainty, MaybeCause, Response};
 use super::{EvalCtxt, QueryResult};
 use rustc_data_structures::fx::FxHashMap;
+use rustc_index::vec::IndexVec;
 use rustc_infer::infer::canonical::{Canonical, CanonicalVarKind, CanonicalVarValues};
 use rustc_middle::ty::{self, TyCtxt};
Diff in /checkout/compiler/rustc_trait_selection/src/solve/cache.rs at line 18:
-use rustc_index::vec::IndexVec;
 
 rustc_index::newtype_index! {
 rustc_index::newtype_index! {
     pub struct StackDepth {}
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2021" "--unstable-features" "--skip-children" "--check" "/checkout/compiler/rustc_trait_selection/src/traits/misc.rs" "/checkout/compiler/rustc_trait_selection/src/traits/relationships.rs" "/checkout/compiler/rustc_trait_selection/src/solve/infcx_ext.rs" "/checkout/compiler/rustc_trait_selection/src/traits/coherence.rs" "/checkout/compiler/rustc_trait_selection/src/solve/cache.rs" "/checkout/compiler/rustc_trait_selection/src/solve/project_goals.rs" "/checkout/compiler/rustc_middle/src/util/common/tests.rs" "/checkout/compiler/rustc_trait_selection/src/solve/mod.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
