
error[E0432]: unresolved import `cargo::util::ProcessBuilder`
  --> src/tools/rls/rls/src/build/cargo.rs:19:18
   |
19 |     ConfigValue, ProcessBuilder,
   |                  ^^^^^^^^^^^^^^ no `ProcessBuilder` in `util`

error[E0432]: unresolved import `cargo::util::ProcessBuilder`
  --> src/tools/rls/rls/src/build/cargo_plan.rs:25:5
   |
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
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^ no `ProcessBuilder` in `util`

error[E0433]: failed to resolve: could not find `paths` in `util`
   --> src/tools/rls/rls/src/build/cargo.rs:269:18
    |
269 |     cargo::util::paths::create_dir_all_excluded_from_backups_atomic(
    |                  ^^^^^ could not find `paths` in `util`

error[E0560]: struct `CompileOptions` has no field named `features`
   --> src/tools/rls/rls/src/build/cargo.rs:232:9
    |
232 |         features: opts.features,
    |         ^^^^^^^^ `CompileOptions` does not have this field
    |
    = note: available fields are: `build_config`, `cli_features`, `spec`, `filter`, `target_rustdoc_args` ... and 4 others

error[E0560]: struct `CompileOptions` has no field named `all_features`
   --> src/tools/rls/rls/src/build/cargo.rs:233:9
    |
233 |         all_features: opts.all_features,
    |         ^^^^^^^^^^^^ help: a field with a similar name exists: `cli_features`

error[E0560]: struct `CompileOptions` has no field named `no_default_features`
   --> src/tools/rls/rls/src/build/cargo.rs:234:9
    |
234 |         no_default_features: opts.no_default_features,
    |         ^^^^^^^^^^^^^^^^^^^ `CompileOptions` does not have this field
    |
    = note: available fields are: `build_config`, `cli_features`, `spec`, `filter`, `target_rustdoc_args` ... and 4 others

error[E0271]: type mismatch resolving `<std::collections::btree_map::Iter<'_, std::string::String, std::option::Option<OsString>> as Iterator>::Item == (_, std::option::Option<_>)`
   --> src/tools/rls/rls/src/build/cargo.rs:491:23
    |
491 |         for (k, v) in &envs {
    |                       ^^^^^ expected reference, found enum `std::option::Option`
    |
    = note: expected tuple `(&std::string::String, &std::option::Option<OsString>)`
               found tuple `(_, std::option::Option<_>)`

error[E0283]: type annotations needed
   --> src/tools/rls/rls/src/build/cargo_plan.rs:53:5
    |
44  |   #[derive(Debug, Default)]
    |                   ------- in this macro invocation
...
53  |       pub(crate) compiler_jobs: HashMap<UnitKey, ProcessBuilder>,
    |       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ cannot infer type
    | 
   ::: /checkout/library/core/src/default.rs:167:1
    |
167 | / pub macro Default($item:item) {
168 | |     /* compiler built-in */
169 | | }
    | |_- in this expansion of `#[derive(Default)]`
    |
    = note: cannot satisfy `_: std::default::Default`
    = note: required by `std::default::Default::default`

error[E0061]: this function takes 8 arguments but 7 arguments were supplied
   --> src/tools/rls/rls/src/project_model.rs:220:5
    |
220 |     ops::resolve_with_previous(registry, ws, &ResolveOpts::everything(), prev, None, &[], true)
    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^ --------  --  --------------------------  ----  ----  ---  ---- supplied 7 arguments
    |     |
    |     expected 8 arguments
    |
note: function defined here
   --> /checkout/src/tools/cargo/src/cargo/ops/resolve.rs:206:8
    |
206 | pub fn resolve_with_previous<'cfg>(
    |        ^^^^^^^^^^^^^^^^^^^^^

error: aborting due to 11 previous errors

Some errors have detailed explanations: E0061, E0271, E0283, E0432, E0433, E0560.
For more information about an error, try `rustc --explain E0061`.
error: could not compile `rls`

To learn more, run the command again with --verbose.
warning: build failed, waiting for other jobs to finish...
error: build failed
command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "16" "--release" "--locked" "--color" "always" "--manifest-path" "/checkout/src/tools/rls/Cargo.toml" "--features" "clippy, rustc-workspace-hack/all-static" "--message-format" "json-render-diagnostics"
expected success, got: exit code: 101
failed to test rls: could not build
