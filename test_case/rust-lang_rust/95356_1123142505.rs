plain
..........................................................iiiiii........................ 1144/1198
......................i...............................
failures:

---- src/process.rs - process::ExitCode::exit_process (line 1747) stdout ----
error[E0599]: no variant or associated item named `GenericError` found for enum `UhOhError` in the current scope
   |
   |
6  | enum UhOhError { GenericProblem, Specific, WithCode { exit_code: ExitCode, _x: () } }
   | -------------- variant or associated item `GenericError` not found here
...
15 |         UhOhError::GenericError => ExitCode::FAILURE,
   |                    ^^^^^^^^^^^^ variant or associated item not found in `UhOhError`
error[E0658]: use of unstable library feature 'exitcode_exit_method'
  --> src/process.rs:1763:10
   |
19 |     code.exit_process()
19 |     code.exit_process()
   |          ^^^^^^^^^^^^
   |
   = help: add `#![feature(exitcode_exit_method)]` to the crate attributes to enable
error: aborting due to 2 previous errors

Some errors have detailed explanations: E0599, E0658.
For more information about an error, try `rustc --explain E0599`.
