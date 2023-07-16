plain
   Compiling toml v0.5.7
error[E0277]: `PathBuf` doesn't implement `std::fmt::Display`
  --> src/bootstrap/sanity.rs:99:71
   |
99 |                     eprintln!("env var CMAKE = {cmake_from_env:?} != {explicit_name} from config.toml")
   |                                                                       ^^^^^^^^^^^^^ `PathBuf` cannot be formatted with the default formatter; call `.display()` on it
   = help: the trait `std::fmt::Display` is not implemented for `PathBuf`
   = help: the trait `std::fmt::Display` is not implemented for `PathBuf`
   = note: call `.display()` or `.to_string_lossy()` to safely print paths, as they may contain non-Unicode data
   = note: this error originates in the macro `$crate::format_args_nl` (in Nightly builds, run with -Z macro-backtrace for more info)
For more information about this error, try `rustc --explain E0277`.
error: could not compile `bootstrap` due to previous error
failed to run: /checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo build --manifest-path /checkout/src/bootstrap/Cargo.toml
Build completed unsuccessfully in 0:01:18
