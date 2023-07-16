plain
   Compiling toml v0.5.7
error[E0599]: no method named `success` found for struct `std::process::Output` in the current scope
   --> src/bootstrap/setup.rs:117:75
    |
117 |     let rustup_installed = rustup_installed.map_or(false, |status| status.success());
    |                                                                           ^^^^^^^ method not found in `std::process::Output`
For more information about this error, try `rustc --explain E0599`.
error: could not compile `bootstrap` due to previous error
failed to run: /checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo build --manifest-path /checkout/src/bootstrap/Cargo.toml
Build completed unsuccessfully in 0:00:48
