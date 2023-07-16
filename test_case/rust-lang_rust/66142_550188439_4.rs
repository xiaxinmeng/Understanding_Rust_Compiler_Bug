\n\nYou need to link your code to the relevant crate in order to be able to use it\n(through Cargo or the `-L` option of rustc example). Plugins are crates as\nwell, and you link to them the same way.\n"},"level":"error","spans":[{"file_name":"tests/ui/useless_attribute.rs","byte_start":327,"byte_end":353,"line_start":14,"line_end":14,"column_start":1,"column_end":27,"is_primary":true,"text":[{"text":"extern crate clippy_lints;","highlight_start":1,"highlight_end":27}],"label":"can't find crate","suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error[E0463]: can't find crate for `clippy_lints`\n  --> tests/ui/useless_attribute.rs:14:1\n   |\nLL | extern crate clippy_lints;\n   | ^^^^^^^^^^^^^^^^^^^^^^^^^^ can't find crate\n\n"}
2019-11-06T07:13:13.5335825Z {"message":"For more information about this error, try `rustc --explain E0463`.","code":null,"level":"failure-note","spans":[],"children":[],"rendered":"For more information about this error, try `rustc --explain E0463`.\n"}
2019-11-06T07:13:13.5335880Z 
2019-11-06T07:13:13.5336112Z ------------------------------------------
2019-11-06T07:13:13.5336145Z 
---
2019-11-06T07:48:57.5582521Z Verifying status of rustfmt...
2019-11-06T07:48:57.5594325Z Verifying status of clippy-driver...
2019-11-06T07:48:57.5606411Z This PR updated 'src/tools/clippy', verifying if status is 'test-pass'...
2019-11-06T07:48:57.5617449Z 
2019-11-06T07:48:57.5618448Z ⚠️ We detected that this PR updated 'clippy-driver', but its tests failed.
2019-11-06T07:48:57.5618554Z 
2019-11-06T07:48:57.5618892Z If you do intend to update 'clippy-driver', please check the error messages above and
2019-11-06T07:48:57.5618977Z commit another update.
2019-11-06T07:48:57.5619007Z 
2019-11-06T07:48:57.5619270Z If you do NOT intend to update 'clippy-driver', please ensure you did not accidentally
2019-11-06T07:48:57.5619552Z change the submodule at 'src/tools/clippy'. You may ask your reviewer for the
2019-11-06T07:48:57.5619628Z proper steps.
2019-11-06T07:48:57.5640706Z   local time: Wed Nov  6 07:48:57 UTC 2019
2019-11-06T07:48:57.7224434Z   network time: Wed, 06 Nov 2019 07:48:57 GMT
2019-11-06T07:48:57.7224762Z == end clock drift check ==
2019-11-06T07:48:58.2803356Z 
2019-11-06T07:48:58.2803356Z 
2019-11-06T07:48:58.2921997Z ##[error]Bash exited with code '3'.
2019-11-06T07:48:58.2958125Z ##[section]Starting: Checkout
2019-11-06T07:48:58.2960021Z ==============================================================================
2019-11-06T07:48:58.2960084Z Task         : Get sources
2019-11-06T07:48:58.2960155Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
