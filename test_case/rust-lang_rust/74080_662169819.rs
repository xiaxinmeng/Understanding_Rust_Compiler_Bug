
error[E0635]: unknown feature `const_transmute`
  --> /home/swarming/.cargo/registry/src/github.com-1ecc6299db9ec823/rustc-ap-rustc_ast-664.0.0/lib.rs:13:12
   |
13 | #![feature(const_transmute)]
   |            ^^^^^^^^^^^^^^^

error: aborting due to previous error

For more information about this error, try `rustc --explain E0635`.
error: could not compile `rustc-ap-rustc_ast`.

To learn more, run the command again with --verbose.
warning: build failed, waiting for other jobs to finish...
error: build failed
command did not execute successfully: "/b/s/w/ir/k/recipe_cleanup/rustLjFjLE/build/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "32" "--release" "--manifest-path" "/b/s/w/ir/k/rust/src/tools/rls/Cargo.toml" "--features" "clippy, rustc-workspace-hack/all-static" "--message-format" "json-render-diagnostics"
expected success, got: exit code: 101
thread 'main' panicked at 'Unable to build RLS', src/bootstrap/dist.rs:66:9
