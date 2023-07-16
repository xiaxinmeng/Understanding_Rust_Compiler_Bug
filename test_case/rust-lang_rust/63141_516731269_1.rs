\n\nDelete the offending feature attribute.\n"},"level":"error","spans":[{"file_name":"tests/run-pass/async-fn.rs","byte_start":33,"byte_end":44,"line_start":3,"line_end":3,"column_start":5,"column_end":16,"is_primary":true,"text":[{"text":"    await_macro,","highlight_start":5,"highlight_end":16}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"subsumed by `.await` syntax","code":null,"level":"note","spans":[{"file_name":"tests/run-pass/async-fn.rs","byte_start":33,"byte_end":44,"line_start":3,"line_end":3,"column_start":5,"column_end":16,"is_primary":true,"text":[{"text":"    await_macro,","highlight_start":5,"highlight_end":16}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":null}],"rendered":"error[E0557]: feature has been removed\n --> tests/run-pass/async-fn.rs:3:5\n  |\n3 |     await_macro,\n  |     ^^^^^^^^^^^\n  |\nnote: subsumed by `.await` syntax\n --> tests/run-pass/async-fn.rs:3:5\n  |\n3 |     await_macro,\n  |     ^^^^^^^^^^^\n\n"}
2019-07-31T07:23:27.2479720Z {"message":"For more information about this error, try `rustc --explain E0557`.","code":null,"level":"","spans":[],"children":[],"rendered":"For more information about this error, try `rustc --explain E0557`.\n"}
2019-07-31T07:23:27.2479830Z 
2019-07-31T07:23:27.2480087Z ------------------------------------------
2019-07-31T07:23:27.2480163Z 
---
2019-07-31T07:23:52.8198580Z Verifying status of clippy-driver...
2019-07-31T07:23:52.8211846Z Verifying status of miri...
2019-07-31T07:23:52.8224932Z This PR updated 'src/tools/miri', verifying if status is 'test-pass'...
2019-07-31T07:23:52.8235332Z 
2019-07-31T07:23:52.8236649Z ⚠️ We detected that this PR updated 'miri', but its tests failed.
2019-07-31T07:23:52.8237337Z 
2019-07-31T07:23:52.8238026Z If you do intend to update 'miri', please check the error messages above and
2019-07-31T07:23:52.8238606Z commit another update.
2019-07-31T07:23:52.8239408Z 
2019-07-31T07:23:52.8239928Z If you do NOT intend to update 'miri', please ensure you did not accidentally
2019-07-31T07:23:52.8240536Z change the submodule at 'src/tools/miri'. You may ask your reviewer for the
2019-07-31T07:23:52.8241058Z proper steps.
2019-07-31T07:23:53.5598716Z ##[error]Bash exited with code '3'.
2019-07-31T07:23:53.5630763Z ##[section]Starting: Upload CPU usage statistics
2019-07-31T07:23:53.5638644Z ==============================================================================
2019-07-31T07:23:53.5638721Z Task         : Bash
2019-07-31T07:23:53.5638796Z Description  : Run a Bash script on macOS, Linux, or Windows
