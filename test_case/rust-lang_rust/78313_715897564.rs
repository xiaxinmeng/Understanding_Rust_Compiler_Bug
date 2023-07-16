plain
   Compiling chalk-ir v0.32.0
   Compiling rustc_index v0.0.0 (/checkout/compiler/rustc_index)
   Compiling tracing-tree v0.1.6
   Compiling rustc_data_structures v0.0.0 (/checkout/compiler/rustc_data_structures)
error: struct is never constructed: `MapIdVec`
  --> compiler/rustc_data_structures/src/functor.rs:36:8
   |
36 | struct MapIdVec<T> {
   |
   |
   = note: `-D dead-code` implied by `-D warnings`
error: aborting due to previous error

error: could not compile `rustc_data_structures`


To learn more, run the command again with --verbose.
warning: build failed, waiting for other jobs to finish...
error: build failed
command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "16" "--release" "--locked" "--color" "always" "--features" "jemalloc llvm max_level_info" "--manifest-path" "/checkout/compiler/rustc/Cargo.toml" "--message-format" "json-render-diagnostics"
failed to run: /checkout/obj/build/bootstrap/debug/bootstrap dist --host x86_64-unknown-linux-gnu --target x86_64-unknown-linux-gnu --include-default-paths src/tools/build-manifest
Build completed unsuccessfully in 0:08:51
== clock drift check ==
  local time: Sat Oct 24 10:53:32 UTC 2020
  local time: Sat Oct 24 10:53:32 UTC 2020
  network time: Fri, 23 Oct 2020 23:31:30 GMT
== end clock drift check ==
##[error]Process completed with exit code 1.
Terminate orphan process: pid (4555) (python)
