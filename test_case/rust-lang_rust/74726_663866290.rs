plain
Building stage1 tool error_index_generator (x86_64-unknown-linux-gnu)
   Compiling same-file v1.0.4
   Compiling walkdir v2.2.7
   Compiling error_index_generator v0.0.0 (/checkout/src/tools/error_index_generator)
error[E0463]: can't find crate for `env_logger`
  |
3 | extern crate env_logger;
  | ^^^^^^^^^^^^^^^^^^^^^^^^ can't find crate


error: aborting due to previous error

For more information about this error, try `rustc --explain E0463`.
error: could not compile `error_index_generator`.
To learn more, run the command again with --verbose.
command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "16" "--release" "--locked" "--color" "always" "--manifest-path" "/checkout/src/tools/error_index_generator/Cargo.toml" "--message-format" "json-render-diagnostics"
expected success, got: exit code: 101
failed to run: /checkout/obj/build/bootstrap/debug/bootstrap dist --host x86_64-unknown-linux-gnu --target x86_64-unknown-linux-gnu
failed to run: /checkout/obj/build/bootstrap/debug/bootstrap dist --host x86_64-unknown-linux-gnu --target x86_64-unknown-linux-gnu
Build completed unsuccessfully in 0:22:23
== clock drift check ==
  local time: Sat Jul 25 15:11:39 UTC 2020
  network time: Sat, 25 Jul 2020 15:11:40 GMT
== end clock drift check ==
##[error]Process completed with exit code 1.
Terminate orphan process: pid (3404) (python)
