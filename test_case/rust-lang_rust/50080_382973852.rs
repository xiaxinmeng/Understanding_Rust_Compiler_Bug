plain
[00:35:13]    Compiling bitflags v0.9.1
[00:35:16]    Compiling rand v0.4.2
[00:35:22]    Compiling tempdir v0.3.7
[00:35:22]    Compiling rustdoc v0.0.0 (file:///checkout/src/librustdoc)
[00:35:32] error[E0560]: struct `rustc::session::config::DebuggingOptions` has no field named `edition`
[00:35:32]     |
[00:35:32] 158 |             edition,
[00:35:32] 158 |             edition,
[00:35:32]     |             ^^^^^^^ `rustc::session::config::DebuggingOptions` does not have this field
[00:35:32]     |
[00:35:32]     = note: available fields are: `codegen_backend`, `verbose`, `span_free_formats`, `identify_regions`, `emit_end_regions` ... and 96 others
[00:35:32] 
[00:35:38] error[E0560]: struct `rustc::session::config::DebuggingOptions` has no field named `edition`
[00:35:38]    |
[00:35:38] 83 |             edition,
[00:35:38] 83 |             edition,
[00:35:38]    |             ^^^^^^^ `rustc::session::config::DebuggingOptions` does not have this field
[00:35:38]    |
[00:35:38]    = note: available fields are: `codegen_backend`, `verbose`, `span_free_formats`, `identify_regions`, `emit_end_regions` ... and 96 others
[00:35:38] 
[00:35:38] error[E0560]: struct `rustc::session::config::DebuggingOptions` has no field named `edition`
[00:35:38]     |
[00:35:38] 226 |             edition,
[00:35:38] 226 |             edition,
[00:35:38]     |             ^^^^^^^ `rustc::session::config::DebuggingOptions` does not have this field
[00:35:38]     |
[00:35:38]     = note: available fields are: `codegen_backend`, `verbose`, `span_free_formats`, `identify_regions`, `emit_end_regions` ... and 96 others
[00:35:38] error: aborting due to 3 previous errors
[00:35:38] 
[00:35:38] For more information about this error, try `rustc --explain E0560`.
[00:35:38] error: Could not compile `rustdoc`.
---
[00:35:38] 
[00:35:38] 
[00:35:38] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap build
[00:35:38] Build completed unsuccessfully in 0:30:59
[00:35:39] make: *** [all] Error 1
[00:35:39] Makefile:28: recipe for target 'all' failed

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:09ef5eb4
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
