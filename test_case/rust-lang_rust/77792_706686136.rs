plain
   Compiling rustc_trait_selection v0.0.0 (/checkout/compiler/rustc_trait_selection)
error: expected expression, found `?`
   --> compiler/rustc_trait_selection/src/traits/fulfill.rs:346:38
    |
346 |         debug!("process_obligation", ?obligation, ?obligation.cause);
    |                                      ^ expected expression
error: aborting due to previous error

error: could not compile `rustc_trait_selection`


To learn more, run the command again with --verbose.
warning: build failed, waiting for other jobs to finish...
error: build failed
command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "16" "--release" "--locked" "--color" "always" "--features" "jemalloc llvm max_level_info" "--manifest-path" "/checkout/compiler/rustc/Cargo.toml" "--message-format" "json-render-diagnostics"
failed to run: /checkout/obj/build/bootstrap/debug/bootstrap dist --host x86_64-unknown-linux-gnu --target x86_64-unknown-linux-gnu
Build completed unsuccessfully in 0:11:36
== clock drift check ==
  local time: Sun Oct 11 10:53:11 UTC 2020
  local time: Sun Oct 11 10:53:11 UTC 2020
  network time: Sun, 11 Oct 2020 10:53:12 GMT
== end clock drift check ==
##[error]Process completed with exit code 1.
Terminate orphan process: pid (4653) (python)
