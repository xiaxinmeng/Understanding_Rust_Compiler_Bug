plain
   Compiling fd-lock v3.0.11
   Compiling toml v0.5.9
   Compiling xz2 v0.1.6
   Compiling sysinfo v0.26.7
error[E0277]: the trait bound `BuildMetrics: Clone` is not satisfied
    |
193 | #[derive(Clone)]
    |          ----- in this derive macro expansion
...
...
244 |     metrics: metrics::BuildMetrics,
    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ the trait `Clone` is not implemented for `BuildMetrics`
    = note: this error originates in the derive macro `Clone` (in Nightly builds, run with -Z macro-backtrace for more info)

For more information about this error, try `rustc --explain E0277`.
error: could not compile `bootstrap` due to previous error
error: could not compile `bootstrap` due to previous error
failed to run: /checkout/obj/build/aarch64-unknown-linux-gnu/stage0/bin/cargo build --manifest-path /checkout/src/bootstrap/Cargo.toml --locked --features build-metrics
Build completed unsuccessfully in 0:00:32
make: *** [Makefile:58: prepare] Error 1
Command failed. Attempt 2/5:
Building bootstrap
   Compiling bootstrap v0.0.0 (/checkout/src/bootstrap)
error[E0277]: the trait bound `BuildMetrics: Clone` is not satisfied
    |
193 | #[derive(Clone)]
    |          ----- in this derive macro expansion
...
...
244 |     metrics: metrics::BuildMetrics,
    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ the trait `Clone` is not implemented for `BuildMetrics`
    = note: this error originates in the derive macro `Clone` (in Nightly builds, run with -Z macro-backtrace for more info)

For more information about this error, try `rustc --explain E0277`.
error: could not compile `bootstrap` due to previous error
error: could not compile `bootstrap` due to previous error
failed to run: /checkout/obj/build/aarch64-unknown-linux-gnu/stage0/bin/cargo build --manifest-path /checkout/src/bootstrap/Cargo.toml --locked --features build-metrics
Build completed unsuccessfully in 0:00:02
make: *** [Makefile:58: prepare] Error 1
Command failed. Attempt 3/5:
Building bootstrap
   Compiling bootstrap v0.0.0 (/checkout/src/bootstrap)
error[E0277]: the trait bound `BuildMetrics: Clone` is not satisfied
    |
193 | #[derive(Clone)]
    |          ----- in this derive macro expansion
...
...
244 |     metrics: metrics::BuildMetrics,
    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ the trait `Clone` is not implemented for `BuildMetrics`
    = note: this error originates in the derive macro `Clone` (in Nightly builds, run with -Z macro-backtrace for more info)

For more information about this error, try `rustc --explain E0277`.
error: could not compile `bootstrap` due to previous error
error: could not compile `bootstrap` due to previous error
failed to run: /checkout/obj/build/aarch64-unknown-linux-gnu/stage0/bin/cargo build --manifest-path /checkout/src/bootstrap/Cargo.toml --locked --features build-metrics
Build completed unsuccessfully in 0:00:02
make: *** [Makefile:58: prepare] Error 1
Command failed. Attempt 4/5:
Building bootstrap
   Compiling bootstrap v0.0.0 (/checkout/src/bootstrap)
error[E0277]: the trait bound `BuildMetrics: Clone` is not satisfied
    |
193 | #[derive(Clone)]
    |          ----- in this derive macro expansion
...
...
244 |     metrics: metrics::BuildMetrics,
    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ the trait `Clone` is not implemented for `BuildMetrics`
    = note: this error originates in the derive macro `Clone` (in Nightly builds, run with -Z macro-backtrace for more info)

For more information about this error, try `rustc --explain E0277`.
error: could not compile `bootstrap` due to previous error
error: could not compile `bootstrap` due to previous error
failed to run: /checkout/obj/build/aarch64-unknown-linux-gnu/stage0/bin/cargo build --manifest-path /checkout/src/bootstrap/Cargo.toml --locked --features build-metrics
Build completed unsuccessfully in 0:00:02
make: *** [Makefile:58: prepare] Error 1
Command failed. Attempt 5/5:
Building bootstrap
   Compiling bootstrap v0.0.0 (/checkout/src/bootstrap)
error[E0277]: the trait bound `BuildMetrics: Clone` is not satisfied
    |
193 | #[derive(Clone)]
    |          ----- in this derive macro expansion
...
...
244 |     metrics: metrics::BuildMetrics,
    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ the trait `Clone` is not implemented for `BuildMetrics`
    = note: this error originates in the derive macro `Clone` (in Nightly builds, run with -Z macro-backtrace for more info)

For more information about this error, try `rustc --explain E0277`.
error: could not compile `bootstrap` due to previous error
