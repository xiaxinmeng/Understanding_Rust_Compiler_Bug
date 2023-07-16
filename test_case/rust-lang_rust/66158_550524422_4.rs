\n\nYou need to link your code to the relevant crate in order to be able to use it\n(through Cargo or the `-L` option of rustc example). Plugins are crates as\nwell, and you link to them the same way.\n"},"level":"error","spans":[{"file_name":"tests/ui/useless_attribute.rs","byte_start":327,"byte_end":353,"line_start":14,"line_end":14,"column_start":1,"column_end":27,"is_primary":true,"text":[{"text":"extern crate clippy_lints;","highlight_start":1,"highlight_end":27}],"label":"can't find crate","suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error[E0463]: can't find crate for `clippy_lints`\n  --> tests/ui/useless_attribute.rs:14:1\n   |\nLL | extern crate clippy_lints;\n   | ^^^^^^^^^^^^^^^^^^^^^^^^^^ can't find crate\n\n"}
2019-11-06T21:36:06.7180481Z {"message":"For more information about this error, try `rustc --explain E0463`.","code":null,"level":"failure-note","spans":[],"children":[],"rendered":"For more information about this error, try `rustc --explain E0463`.\n"}
2019-11-06T21:36:06.7180704Z 
2019-11-06T21:36:06.7180871Z ------------------------------------------
2019-11-06T21:36:06.7180896Z 
---
2019-11-06T22:12:04.2745029Z Verifying status of rustfmt...
2019-11-06T22:12:04.2761706Z Verifying status of clippy-driver...
2019-11-06T22:12:04.2779639Z This PR updated 'src/tools/clippy', verifying if status is 'test-pass'...
2019-11-06T22:12:04.2789871Z 
2019-11-06T22:12:04.2790557Z ⚠️ We detected that this PR updated 'clippy-driver', but its tests failed.
2019-11-06T22:12:04.2790934Z 
2019-11-06T22:12:04.2791985Z If you do intend to update 'clippy-driver', please check the error messages above and
2019-11-06T22:12:04.2792176Z commit another update.
2019-11-06T22:12:04.2792324Z 
2019-11-06T22:12:04.2792866Z If you do NOT intend to update 'clippy-driver', please ensure you did not accidentally
2019-11-06T22:12:04.2793257Z change the submodule at 'src/tools/clippy'. You may ask your reviewer for the
2019-11-06T22:12:04.2793462Z proper steps.
2019-11-06T22:12:04.2816882Z   local time: Wed Nov  6 22:12:04 UTC 2019
2019-11-06T22:12:04.3786441Z   network time: Wed, 06 Nov 2019 22:12:04 GMT
2019-11-06T22:12:04.3790491Z == end clock drift check ==
2019-11-06T22:12:04.9186581Z 
2019-11-06T22:12:04.9186581Z 
2019-11-06T22:12:04.9290796Z ##[error]Bash exited with code '3'.
2019-11-06T22:12:04.9321068Z ##[section]Starting: Checkout
2019-11-06T22:12:04.9322721Z ==============================================================================
2019-11-06T22:12:04.9322768Z Task         : Get sources
2019-11-06T22:12:04.9322826Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
