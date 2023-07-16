
error[E0432]: unresolved import `cargo::core::enable_nightly_features`
  --> src/tools/rls/rls/src/build/cargo.rs:16:5
   |
16 |     enable_nightly_features, PackageId, Shell, Target, TargetKind, Verbosity, Workspace,
   |     ^^^^^^^^^^^^^^^^^^^^^^^ no `enable_nightly_features` in `core`

error[E0425]: cannot find function `enable_nightly_features` in module `cargo::core`
  --> src/tools/rls/rls/src/project_model.rs:48:22
   |
48 |         cargo::core::enable_nightly_features();
   |                      ^^^^^^^^^^^^^^^^^^^^^^^ not found in `cargo::core`

error: aborting due to 2 previous errors
