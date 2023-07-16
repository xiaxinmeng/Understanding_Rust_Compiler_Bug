\n\nYou need to link your code to the relevant crate in order to be able to use it\n(through Cargo or the `-L` option of rustc example). Plugins are crates as\nwell, and you link to them the same way.\n"},"level":"error","spans":[{"file_name":"tests/ui/useless_attribute.rs","byte_start":327,"byte_end":353,"line_start":14,"line_end":14,"column_start":1,"column_end":27,"is_primary":true,"text":[{"text":"extern crate clippy_lints;","highlight_start":1,"highlight_end":27}],"label":"can't find crate","suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error[E0463]: can't find crate for `clippy_lints`\n  --> tests/ui/useless_attribute.rs:14:1\n   |\nLL | extern crate clippy_lints;\n   | ^^^^^^^^^^^^^^^^^^^^^^^^^^ can't find crate\n\n"}
2019-11-06T17:25:15.9738077Z {"message":"For more information about this error, try `rustc --explain E0463`.","code":null,"level":"failure-note","spans":[],"children":[],"rendered":"For more information about this error, try `rustc --explain E0463`.\n"}
2019-11-06T17:25:15.9738153Z 
2019-11-06T17:25:15.9738378Z ------------------------------------------
2019-11-06T17:25:15.9738411Z 
---
2019-11-06T18:01:57.5487742Z Verifying status of rustfmt...
2019-11-06T18:01:57.5501678Z Verifying status of clippy-driver...
2019-11-06T18:01:57.5516344Z This PR updated 'src/tools/clippy', verifying if status is 'test-pass'...
2019-11-06T18:01:57.5526898Z 
2019-11-06T18:01:57.5528948Z ⚠️ We detected that this PR updated 'clippy-driver', but its tests failed.
2019-11-06T18:01:57.5529069Z 
2019-11-06T18:01:57.5529402Z If you do intend to update 'clippy-driver', please check the error messages above and
2019-11-06T18:01:57.5529464Z commit another update.
2019-11-06T18:01:57.5529496Z 
2019-11-06T18:01:57.5530061Z If you do NOT intend to update 'clippy-driver', please ensure you did not accidentally
2019-11-06T18:01:57.5530402Z change the submodule at 'src/tools/clippy'. You may ask your reviewer for the
2019-11-06T18:01:57.5530461Z proper steps.
2019-11-06T18:01:57.5554615Z   local time: Wed Nov  6 18:01:57 UTC 2019
2019-11-06T18:01:57.8499583Z   network time: Wed, 06 Nov 2019 18:01:57 GMT
2019-11-06T18:01:57.8503410Z == end clock drift check ==
2019-11-06T18:01:58.4063292Z 
2019-11-06T18:01:58.4063292Z 
2019-11-06T18:01:58.4169609Z ##[error]Bash exited with code '3'.
2019-11-06T18:01:58.4216238Z ##[section]Starting: Checkout
2019-11-06T18:01:58.4218103Z ==============================================================================
2019-11-06T18:01:58.4218165Z Task         : Get sources
2019-11-06T18:01:58.4218217Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
