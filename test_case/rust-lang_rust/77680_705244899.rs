plain
   Compiling rustc_mir v0.0.0 (/checkout/compiler/rustc_mir)
error: unused variable: `tcx`
  --> compiler/rustc_mir/src/transform/unreachable_prop.rs:15:30
   |
15 |     fn run_pass<'tcx>(&self, tcx: TyCtxt<'tcx>, body: &mut Body<'tcx>) {
   |                              ^^^ help: if this is intentional, prefix it with an underscore: `_tcx`
   |
   = note: `-D unused-variables` implied by `-D warnings`
error: aborting due to previous error

error: could not compile `rustc_mir`.


To learn more, run the command again with --verbose.
warning: build failed, waiting for other jobs to finish...
error: build failed
command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "16" "--release" "--locked" "--color" "always" "--features" "jemalloc llvm max_level_info" "--manifest-path" "/checkout/compiler/rustc/Cargo.toml" "--message-format" "json-render-diagnostics"
failed to run: /checkout/obj/build/bootstrap/debug/bootstrap dist --host x86_64-unknown-linux-gnu --target x86_64-unknown-linux-gnu
Build completed unsuccessfully in 0:12:13
== clock drift check ==
  local time: Wed Oct  7 23:25:53 UTC 2020
  local time: Wed Oct  7 23:25:53 UTC 2020
  network time: Wed, 07 Oct 2020 23:25:53 GMT
== end clock drift check ==
##[error]Process completed with exit code 1.
Terminate orphan process: pid (4408) (python)
