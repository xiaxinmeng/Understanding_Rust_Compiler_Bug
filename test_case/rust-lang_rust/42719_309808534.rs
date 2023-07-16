
...
DEBUG:cargo::core::registry: load/match    registry https://github.com/rust-lang/crates.io-index
DEBUG:cargo::core::registry: load/match    registry https://github.com/rust-lang/crates.io-index
DEBUG:cargo::core::registry: load/match    registry https://github.com/rust-lang/crates.io-index
DEBUG:cargo::core::registry: load/missing  https://github.com/rust-lang/cargo#50b1c24d
DEBUG:cargo::sources::config: loading: https://github.com/rust-lang/cargo#50b1c24d
    Updating git repository `https://github.com/rust-lang/cargo`
DEBUG:cargo: exit_with_error; err=CliError { error: Some(CargoError(Msg("failed to load source for a dependency on `cargo`"), State { next_error:
 Some(CargoError(Msg("Unable to update https://github.com/rust-lang/cargo#50b1c24d"), State { next_error: Some(CargoError(Msg("failed to fetch into /data/semarie/build-rust/install_dir/crates/git/db/cargo-e7ff1db891893a9e"), State { next_error: Some(CargoError(Msg("attempting to update a git repository, but --frozen was specified"), State { next_error: None, backtrace: None })), backtrace: None })), backtrace: None })), backtrace:
None })), unknown: false, exit_code: 101 }
error: failed to load source for a dependency on `cargo`

Caused by:
  Unable to update https://github.com/rust-lang/cargo#50b1c24d

Caused by:
  failed to fetch into /data/semarie/build-rust/install_dir/crates/git/db/cargo-e7ff1db891893a9e

Caused by:
  attempting to update a git repository, but --frozen was specified
Traceback (most recent call last):
  File "/data/semarie/build-rust/build_dir/rustc/rustc-nightly-src/src/bootstrap/bootstrap.py", line 694, in <module>
    main()
  File "/data/semarie/build-rust/build_dir/rustc/rustc-nightly-src/src/bootstrap/bootstrap.py", line 678, in main
    bootstrap()
  File "/data/semarie/build-rust/build_dir/rustc/rustc-nightly-src/src/bootstrap/bootstrap.py", line 660, in bootstrap
    rb.build_bootstrap()
  File "/data/semarie/build-rust/build_dir/rustc/rustc-nightly-src/src/bootstrap/bootstrap.py", line 413, in build_bootstrap
    run(args, env=env, verbose=self.verbose)
  File "/data/semarie/build-rust/build_dir/rustc/rustc-nightly-src/src/bootstrap/bootstrap.py", line 142, in run
    raise RuntimeError(err)
RuntimeError: failed to run: /data/semarie/build-rust/install_dir/beta/bin/cargo build --manifest-path /data/semarie/build-rust/build_dir/rustc/rustc-nightly-src/src/bootstrap/Cargo.toml --verbose --frozen
