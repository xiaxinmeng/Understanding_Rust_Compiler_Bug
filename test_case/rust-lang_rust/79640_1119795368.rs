
root@ae1e55809c68:/rust-project# cargo +nightly test --verbose
   Compiling rust-project v0.1.0 (/rust-project)
     Running `rustc --crate-name rust_project --edition=2018 src/main.rs --error-format=json --json=diagnostic-rendered-ansi,artifacts,future-incompat --emit=dep-info,link -C embed-bitcode=no -C debuginfo=2 --test -C metadata=0d091f013ad5ec96 -C extra-filename=-0d091f013ad5ec96 --out-dir /rust-project/target/debug/deps -L dependency=/rust-project/target/debug/deps -Zprofile -Ccodegen-units=1 -Copt-level=0 -Clink-dead-code -Coverflow-checks=off -Zpanic_abort_tests -Cpanic=abort`
error[E0463]: can't find crate for `profiler_builtins`
  |
  = note: the compiler may have been built without the profiler runtime

For more information about this error, try `rustc --explain E0463`.
error: could not compile `rust-project` due to previous error

Caused by:
  process didn't exit successfully: `rustc --crate-name rust_project --edition=2018 src/main.rs --error-format=json --json=diagnostic-rendered-ansi,artifacts,future-incompat --emit=dep-info,link -C embed-bitcode=no -C debuginfo=2 --test -C metadata=0d091f013ad5ec96 -C extra-filename=-0d091f013ad5ec96 --out-dir /rust-project/target/debug/deps -L dependency=/rust-project/target/debug/deps -Zprofile -Ccodegen-units=1 -Copt-level=0 -Clink-dead-code -Coverflow-checks=off -Zpanic_abort_tests -Cpanic=abort` (exit status: 1)
root@ae1e55809c68:/rust-project#
