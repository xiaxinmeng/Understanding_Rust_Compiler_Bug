\n\nYou need to link your code to the relevant crate in order to be able to use it\n(through Cargo or the `-L` option of rustc example). Plugins are crates as\nwell, and you link to them the same way.\n"},"level":"error","spans":[{"file_name":"tests/ui/useless_attribute.rs","byte_start":327,"byte_end":353,"line_start":14,"line_end":14,"column_start":1,"column_end":27,"is_primary":true,"text":[{"text":"extern crate clippy_lints;","highlight_start":1,"highlight_end":27}],"label":"can't find crate","suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error[E0463]: can't find crate for `clippy_lints`\n  --> tests/ui/useless_attribute.rs:14:1\n   |\nLL | extern crate clippy_lints;\n   | ^^^^^^^^^^^^^^^^^^^^^^^^^^ can't find crate\n\n"}
2019-11-06T16:48:44.8402282Z {"message":"For more information about this error, try `rustc --explain E0463`.","code":null,"level":"failure-note","spans":[],"children":[],"rendered":"For more information about this error, try `rustc --explain E0463`.\n"}
2019-11-06T16:48:44.8402338Z 
2019-11-06T16:48:44.8402571Z ------------------------------------------
2019-11-06T16:48:44.8402620Z 
---
2019-11-06T17:26:57.9740551Z Verifying status of rustfmt...
2019-11-06T17:26:57.9755782Z Verifying status of clippy-driver...
2019-11-06T17:26:57.9770087Z This PR updated 'src/tools/clippy', verifying if status is 'test-pass'...
2019-11-06T17:26:57.9781586Z 
2019-11-06T17:26:57.9782196Z ⚠️ We detected that this PR updated 'clippy-driver', but its tests failed.
2019-11-06T17:26:57.9782506Z 
2019-11-06T17:26:57.9790841Z If you do intend to update 'clippy-driver', please check the error messages above and
2019-11-06T17:26:57.9790938Z commit another update.
2019-11-06T17:26:57.9790979Z 
2019-11-06T17:26:57.9791570Z If you do NOT intend to update 'clippy-driver', please ensure you did not accidentally
2019-11-06T17:26:57.9794313Z change the submodule at 'src/tools/clippy'. You may ask your reviewer for the
2019-11-06T17:26:57.9797344Z proper steps.
2019-11-06T17:26:57.9875323Z   local time: Wed Nov  6 17:26:57 UTC 2019
2019-11-06T17:26:58.0967673Z   network time: Wed, 06 Nov 2019 17:26:58 GMT
2019-11-06T17:26:58.0968511Z == end clock drift check ==
2019-11-06T17:26:59.4349844Z 
2019-11-06T17:26:59.4349844Z 
2019-11-06T17:26:59.4477004Z ##[error]Bash exited with code '3'.
2019-11-06T17:26:59.4553969Z ##[section]Starting: Checkout
2019-11-06T17:26:59.4556296Z ==============================================================================
2019-11-06T17:26:59.4556364Z Task         : Get sources
2019-11-06T17:26:59.4556432Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
