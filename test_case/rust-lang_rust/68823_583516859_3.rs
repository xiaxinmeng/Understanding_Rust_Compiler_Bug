\n\nMust always be called with exactly two arguments, e.g., `f(2, \"test\")`.\n\nNote that Rust does not have a notion of optional function arguments or\nvariadic functions (except for its C-FFI).\n"},"level":"error","spans":[{"file_name":"tests/run-pass/generator.rs","byte_start":380,"byte_end":386,"line_start":12,"line_end":12,"column_start":26,"column_end":32,"is_primary":true,"text":[{"text":"        match t.as_mut().resume() {","highlight_start":26,"highlight_end":32}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"expected the unit value `()`; create it with empty parentheses","code":null,"level":"help","spans":[{"file_name":"tests/run-pass/generator.rs","byte_start":387,"byte_end":387,"line_start":12,"line_end":12,"column_start":33,"column_end":33,"is_primary":true,"text":[{"text":"        match t.as_mut().resume() {","highlight_start":33,"highlight_end":33}],"label":null,"suggested_replacement":"()","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error[E0061]: this function takes 1 parameter but 0 parameters were supplied\n  --> tests/run-pass/generator.rs:12:26\n   |\n12 |         match t.as_mut().resume() {\n   |                          ^^^^^^\n   |\nhelp: expected the unit value `()`; create it with empty parentheses\n   |\n12 |         match t.as_mut().resume(()) {\n   |                                 ^^\n\n"}
2020-02-07T17:30:48.9835819Z {"message":"For more information about this error, try `rustc --explain E0061`.","code":null,"level":"failure-note","spans":[],"children":[],"rendered":"For more information about this error, try `rustc --explain E0061`.\n"}
2020-02-07T17:30:48.9835870Z 
2020-02-07T17:30:48.9836085Z ------------------------------------------
2020-02-07T17:30:48.9836142Z 
---
2020-02-07T17:31:06.3978863Z This PR updated 'src/tools/clippy', verifying if status is 'test-pass'...
2020-02-07T17:31:06.3978907Z 
2020-02-07T17:31:06.3979205Z We detected that this PR updated 'clippy-driver', but its tests failed.
2020-02-07T17:31:06.3979246Z 
2020-02-07T17:31:06.3979543Z If you do intend to update 'clippy-driver', please check the error messages above and
2020-02-07T17:31:06.3979617Z commit another update.
2020-02-07T17:31:06.3979668Z 
2020-02-07T17:31:06.3979966Z If you do NOT intend to update 'clippy-driver', please ensure you did not accidentally
2020-02-07T17:31:06.3980259Z change the submodule at 'src/tools/clippy'. You may ask your reviewer for the
2020-02-07T17:31:06.3980334Z proper steps.
2020-02-07T17:31:06.3989716Z Build completed unsuccessfully in 0:00:01
2020-02-07T17:31:06.4043853Z == clock drift check ==
2020-02-07T17:31:06.4059426Z   local time: Fri Feb  7 17:31:06 UTC 2020
2020-02-07T17:31:06.9555402Z   network time: Fri, 07 Feb 2020 17:31:06 GMT
2020-02-07T17:31:06.9555402Z   network time: Fri, 07 Feb 2020 17:31:06 GMT
2020-02-07T17:31:06.9556129Z == end clock drift check ==
2020-02-07T17:31:07.8365331Z 
2020-02-07T17:31:07.8489723Z ##[error]Bash exited with code '1'.
2020-02-07T17:31:07.8503193Z ##[section]Finishing: Run build
2020-02-07T17:31:07.8527289Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/68823/merge to s
2020-02-07T17:31:07.8529504Z Task         : Get sources
2020-02-07T17:31:07.8529545Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-02-07T17:31:07.8529608Z Version      : 1.0.0
2020-02-07T17:31:07.8529645Z Author       : Microsoft
2020-02-07T17:31:07.8529645Z Author       : Microsoft
2020-02-07T17:31:07.8529687Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-02-07T17:31:07.8529731Z ==============================================================================
2020-02-07T17:31:08.2984700Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-02-07T17:31:08.3028044Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/68823/merge to s
2020-02-07T17:31:08.3152469Z Cleaning up task key
2020-02-07T17:31:08.3153286Z Start cleaning up orphan processes.
2020-02-07T17:31:08.3272963Z Terminate orphan process: pid (3905) (python)
2020-02-07T17:31:08.3553150Z ##[section]Finishing: Finalize Job
