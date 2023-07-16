
[RUSTC-TIMING] cargo test:false 205.713
error[E0432]: unresolved import `cargo::ops::CompileMode`
  --> tools\rls\src\build\cargo.rs:13:52
   |
13 | use cargo::ops::{compile_with_exec, CompileFilter, CompileMode, CompileOptions, Packages};
   |                                                    ^^^^^^^^^^^ no `CompileMode` in `ops`
error[E0560]: struct `cargo::ops::CompileOptions<'_>` has no field named `target`
   --> tools\rls\src\build\cargo.rs:192:9
    |
192 |         target: opts.target,
    |         ^^^^^^ `cargo::ops::CompileOptions<'_>` does not have this field
    |
    = note: available fields are: `config`, `build_config`, `features`, `all_features`, `no_default_features` ... and 5 others
error[E0560]: struct `cargo::ops::CompileOptions<'_>` has no field named `jobs`
   --> tools\rls\src\build\cargo.rs:210:9
    |
210 |         jobs: opts.jobs,
    |         ^^^^ `cargo::ops::CompileOptions<'_>` does not have this field
    |
    = note: available fields are: `config`, `build_config`, `features`, `all_features`, `no_default_features` ... and 5 others
error[E0599]: no function or associated item named `default` found for type `cargo::ops::CompileOptions<'_>` in the current scope
   --> tools\rls\src\build\cargo.rs:211:11
    |
211 |         ..CompileOptions::default(&config, CompileMode::Check { test: cfg_test })
    |           ^^^^^^^^^^^^^^^^^^^^^^^ function or associated item not found in `cargo::ops::CompileOptions<'_>`
error: aborting due to 4 previous errors
