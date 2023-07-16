plain
   Compiling globset v0.4.8
   Compiling ignore v0.4.18
   Compiling xz2 v0.1.6
   Compiling toml v0.5.9
error[E0609]: no field `test_stage` on type `&mut Build`
   --> sanity.rs:144:14
    |
144 |     if build.test_stage.is_some() {
    |
    |
    = note: available fields are: `config`, `version`, `src`, `out`, `bootstrap_out` ... and 30 others
For more information about this error, try `rustc --explain E0609`.
error: could not compile `bootstrap` due to previous error
failed to run: /checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo build --manifest-path /checkout/src/bootstrap/Cargo.toml
Build completed unsuccessfully in 0:00:41
