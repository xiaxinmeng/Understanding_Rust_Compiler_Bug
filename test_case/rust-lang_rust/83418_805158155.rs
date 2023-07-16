plain
  SCCACHE_BUCKET: rust-lang-ci-sccache2
  TOOLSTATE_REPO: https://github.com/rust-lang-nursery/rust-toolstate
  CACHE_DOMAIN: ci-caches.rust-lang.org
  EXTRA_VARIABLES: {
 "CI_ONLY_WHEN_SUBMODULES_CHANGED": 1
##[endgroup]
adding extra environment variable CI_ONLY_WHEN_SUBMODULES_CHANGED
linux builder detected, using docker to run the build
##[group]Run src/ci/scripts/should-skip-this.sh
---

error[E0432]: unresolved import `cargo::util::ProcessBuilder`
  --> src/tools/rls/rls/src/build/cargo.rs:19:18
   |
19 |     ConfigValue, ProcessBuilder,
   |                  ^^^^^^^^^^^^^^ no `ProcessBuilder` in `util`
error[E0432]: unresolved import `cargo::util::ProcessBuilder`
  --> src/tools/rls/rls/src/build/cargo_plan.rs:25:5
   |
25 | use cargo::util::ProcessBuilder;
25 | use cargo::util::ProcessBuilder;
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^ no `ProcessBuilder` in `util`

error[E0432]: unresolved imports `cargo::util::process`, `cargo::util::ProcessBuilder`
  --> src/tools/rls/rls/src/build/external.rs:26:19
   |
26 | use cargo::util::{process, ProcessBuilder};
   |                   ^^^^^^^  ^^^^^^^^^^^^^^ no `ProcessBuilder` in `util`
   |                   |
   |                   no `process` in `util`
   |                   help: a similar name exists in the module: `progress`
error[E0432]: unresolved import `cargo::util::ProcessBuilder`
  --> src/tools/rls/rls/src/build/plan.rs:15:5
   |
15 | use cargo::util::ProcessBuilder;
15 | use cargo::util::ProcessBuilder;
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^ no `ProcessBuilder` in `util`

error[E0433]: failed to resolve: could not find `paths` in `util`
    |
    |
269 |     cargo::util::paths::create_dir_all_excluded_from_backups_atomic(
    |                  ^^^^^ could not find `paths` in `util`

error[E0560]: struct `CompileOptions` has no field named `features`
    |
    |
232 |         features: opts.features,
    |         ^^^^^^^^ `CompileOptions` does not have this field
    |
    = note: available fields are: `build_config`, `cli_features`, `spec`, `filter`, `target_rustdoc_args` ... and 4 others
error[E0560]: struct `CompileOptions` has no field named `all_features`
   --> src/tools/rls/rls/src/build/cargo.rs:233:9
    |
233 |         all_features: opts.all_features,
233 |         all_features: opts.all_features,
    |         ^^^^^^^^^^^^ help: a field with a similar name exists: `cli_features`

error[E0560]: struct `CompileOptions` has no field named `no_default_features`
    |
    |
234 |         no_default_features: opts.no_default_features,
    |         ^^^^^^^^^^^^^^^^^^^ `CompileOptions` does not have this field
    |
    = note: available fields are: `build_config`, `cli_features`, `spec`, `filter`, `target_rustdoc_args` ... and 4 others

error[E0271]: type mismatch resolving `<std::collections::btree_map::Iter<'_, std::string::String, std::option::Option<OsString>> as Iterator>::Item == (_, std::option::Option<_>)`
    |
    |
491 |         for (k, v) in &envs {
    |
    |
    = note: expected tuple `(&std::string::String, &std::option::Option<OsString>)`
               found tuple `(_, std::option::Option<_>)`
error[E0283]: type annotations needed
   --> src/tools/rls/rls/src/build/cargo_plan.rs:53:5
    |
44  |   #[derive(Debug, Default)]
44  |   #[derive(Debug, Default)]
    |                   ------- in this macro invocation
...
53  |       pub(crate) compiler_jobs: HashMap<UnitKey, ProcessBuilder>,
    | 
   ::: /checkout/library/core/src/default.rs:167:1
    |
    |
167 | / pub macro Default($item:item) {
169 | | }
    | |_- in this expansion of `#[derive(Default)]`
    |
    |
    = note: cannot satisfy `_: std::default::Default`
    = note: required by `std::default::Default::default`
error[E0061]: this function takes 8 arguments but 7 arguments were supplied
   --> src/tools/rls/rls/src/project_model.rs:220:5
    |
    |
220 |     ops::resolve_with_previous(registry, ws, &ResolveOpts::everything(), prev, None, &[], true)
    |     |
    |     expected 8 arguments
    |
note: function defined here
note: function defined here
   --> /checkout/src/tools/cargo/src/cargo/ops/resolve.rs:206:8
    |
206 | pub fn resolve_with_previous<'cfg>(

error: aborting due to 11 previous errors

Some errors have detailed explanations: E0061, E0271, E0283, E0432, E0433, E0560.
---
Verifying status of rustfmt...
Verifying status of miri...
Verifying status of embedded-book...
Cloning into 'rust-toolstate'...
error: Tool `rls` has regressed from test-pass to build-fail during beta week.
{"rust-by-example":"test-pass","rustfmt":"test-pass","nomicon":"test-pass","rustbook":"test-fail","edition-guide":"test-pass","rls":"build-fail","book":"test-pass","reference":"test-pass","cargo-miri":"test-fail","miri":"test-pass","embedded-book":"test-pass"}failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test --stage 2 check-tools
