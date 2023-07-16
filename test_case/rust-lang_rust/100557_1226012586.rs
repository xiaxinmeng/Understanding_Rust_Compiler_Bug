plain
   Compiling toml v0.5.9
error[E0061]: this function takes 2 arguments but 1 argument was supplied
   --> native.rs:204:24
    |
204 |         let llvm_sha = detect_llvm_sha(config);
    |
note: function defined here
   --> native.rs:118:15
    |
    |
118 | pub(crate) fn detect_llvm_sha(config: &crate::config::Config, is_git: bool) -> String {
help: provide the argument
    |
    |
204 |         let llvm_sha = detect_llvm_sha(config, /* bool */);

error[E0061]: this function takes 2 arguments but 1 argument was supplied
   --> native.rs:225:20
    |
    |
225 |     let llvm_sha = detect_llvm_sha(&config);
    |
note: function defined here
   --> native.rs:118:15
    |
    |
118 | pub(crate) fn detect_llvm_sha(config: &crate::config::Config, is_git: bool) -> String {
help: provide the argument
    |
    |
225 |     let llvm_sha = detect_llvm_sha(&config, /* bool */);

For more information about this error, try `rustc --explain E0061`.
error: could not compile `bootstrap` due to 2 previous errors
failed to run: /checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo build --manifest-path /checkout/src/bootstrap/Cargo.toml
