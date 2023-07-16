plain
   Compiling rustc_index v0.0.0 (/checkout/compiler/rustc_index)
   Compiling rustc_data_structures v0.0.0 (/checkout/compiler/rustc_data_structures)
   Compiling rustc_arena v0.0.0 (/checkout/compiler/rustc_arena)
   Compiling rustc_span v0.0.0 (/checkout/compiler/rustc_span)
error[E0507]: cannot move out of `range_to_update` which is behind a shared reference
   --> compiler/rustc_span/src/hygiene.rs:397:9
    |
397 |         range_to_update.zip(names.into_iter()).for_each(|(idx, name)| {
    |         ^^^^^^^^^^^^^^^ move occurs because `range_to_update` has type `std::ops::Range<usize>`, which does not implement the `Copy` trait
error: aborting due to previous error

For more information about this error, try `rustc --explain E0507`.
error: could not compile `rustc_span`.
error: could not compile `rustc_span`.

To learn more, run the command again with --verbose.
warning: build failed, waiting for other jobs to finish...
error: build failed
command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "16" "--release" "--locked" "--color" "always" "--features" "jemalloc llvm max_level_info" "--manifest-path" "/checkout/compiler/rustc/Cargo.toml" "--message-format" "json-render-diagnostics"
failed to run: /checkout/obj/build/bootstrap/debug/bootstrap dist --host x86_64-unknown-linux-gnu --target x86_64-unknown-linux-gnu
Build completed unsuccessfully in 0:15:01
== clock drift check ==
  local time: Fri Sep 25 11:59:02 UTC 2020
  local time: Fri Sep 25 11:59:02 UTC 2020
  network time: Fri, 25 Sep 2020 11:59:02 GMT
== end clock drift check ==
##[error]Process completed with exit code 1.
Terminate orphan process: pid (5533) (python)
