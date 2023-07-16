\n"},"level":"error","spans":[{"file_name":"tests/ui/builtin-type-shadow.rs","byte_start":104,"byte_end":106,"line_start":5,"line_end":5,"column_start":5,"column_end":7,"is_primary":true,"text":[{"text":"    42","highlight_start":5,"highlight_end":7}],"label":"expected type parameter, found integer","suggested_replacement":null,"suggestion_applicability":null,"expansion":null},{"file_name":"tests/ui/builtin-type-shadow.rs","byte_start":94,"byte_end":97,"line_start":4,"line_end":4,"column_start":24,"column_end":27,"is_primary":false,"text":[{"text":"fn foo<u32>(a: u32) -> u32 {","highlight_start":24,"highlight_end":27}],"label":"expected `u32` because of return type","suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"expected type `u32`\n   found type `{integer}`","code":null,"level":"note","spans":[],"children":[],"rendered":null},{"message":"type parameters must be constrained to match other types","code":null,"level":"help","spans":[],"children":[],"rendered":null},{"message":"for more information, visit https://doc.rust-lang.org/book/ch10-02-traits.html#traits-as-parameters","code":null,"level":"note","spans":[],"children":[],"rendered":null}],"rendered":"error[E0308]: mismatched types\n  --> tests/ui/builtin-type-shadow.rs:5:5\n   |\nLL | fn foo<u32>(a: u32) -> u32 {\n   |                        --- expected `u32` because of return type\nLL |     42\n   |     ^^ expected type parameter, found integer\n   |\n   = note: expected type `u32`\n              found type `{integer}`\n   = help: type parameters must be constrained to match other types\n   = note: for more information, visit https://doc.rust-lang.org/book/ch10-02-traits.html#traits-as-parameters\n\n"}
2019-09-22T13:42:08.2166032Z {"message":"For more information about this error, try `rustc --explain E0308`.","code":null,"level":"failure-note","spans":[],"children":[],"rendered":"For more information about this error, try `rustc --explain E0308`.\n"}
2019-09-22T13:42:08.2166191Z 
2019-09-22T13:42:08.2166491Z ------------------------------------------
2019-09-22T13:42:08.2166564Z 
---
2019-09-22T14:32:35.8484795Z Verifying status of rustfmt...
2019-09-22T14:32:35.8499929Z Verifying status of clippy-driver...
2019-09-22T14:32:35.8514873Z This PR updated 'src/tools/clippy', verifying if status is 'test-pass'...
2019-09-22T14:32:35.8525930Z 
2019-09-22T14:32:35.8526988Z ⚠️ We detected that this PR updated 'clippy-driver', but its tests failed.
2019-09-22T14:32:35.8527438Z 
2019-09-22T14:32:35.8528665Z If you do intend to update 'clippy-driver', please check the error messages above and
2019-09-22T14:32:35.8528797Z commit another update.
2019-09-22T14:32:35.8528867Z 
2019-09-22T14:32:35.8529244Z If you do NOT intend to update 'clippy-driver', please ensure you did not accidentally
2019-09-22T14:32:35.8529618Z change the submodule at 'src/tools/clippy'. You may ask your reviewer for the
2019-09-22T14:32:35.8529714Z proper steps.
2019-09-22T14:32:35.8547303Z   local time: Sun Sep 22 14:32:35 UTC 2019
2019-09-22T14:32:35.9370367Z   network time: Sun, 22 Sep 2019 14:32:35 GMT
2019-09-22T14:32:35.9370531Z == end clock drift check ==
2019-09-22T14:32:35.9370531Z == end clock drift check ==
2019-09-22T14:32:36.5337373Z ##[error]Bash exited with code '3'.
2019-09-22T14:32:36.5374953Z ##[section]Starting: Upload CPU usage statistics
2019-09-22T14:32:36.5382077Z ==============================================================================
2019-09-22T14:32:36.5382178Z Task         : Bash
2019-09-22T14:32:36.5382267Z Description  : Run a Bash script on macOS, Linux, or Windows
