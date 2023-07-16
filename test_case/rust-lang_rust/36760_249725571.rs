
 Compiling rustc_driver v0.0.0 (file:///build/src/librustc_driver)

error: expected one of `;` or `as`, found `extern`

  --> src/librustc_driver/lib.rs:58:1

   |

58 | extern crate rustc_llvm as llvm;

   | ^^^^^^

error: aborting due to previous error

error: Could not compile `rustc_driver`.

To learn more, run the command again with --verbose.

command did not execute successfully: "/build/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "-j" "2" "--target" "x86_64-unknown-linux-gnu" "--release" "--features" " jemalloc" "--manifest-path" "/build/src/rustc/Cargo.toml"

expected success, got: exit code: 101
