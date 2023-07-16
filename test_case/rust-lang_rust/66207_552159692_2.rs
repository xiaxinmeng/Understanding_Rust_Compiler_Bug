\n\nYou need to link your code to the relevant crate in order to be able to use it\n(through Cargo or the `-L` option of rustc example). Plugins are crates as\nwell, and you link to them the same way.\n"},"level":"error","spans":[{"file_name":"tests/ui/serde.rs","byte_start":57,"byte_end":76,"line_start":4,"line_end":4,"column_start":1,"column_end":20,"is_primary":true,"text":[{"text":"extern crate serde;","highlight_start":1,"highlight_end":20}],"label":"can't find crate","suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error[E0463]: can't find crate for `serde`\n  --> tests/ui/serde.rs:4:1\n   |\nLL | extern crate serde;\n   | ^^^^^^^^^^^^^^^^^^^ can't find crate\n\n"}
2019-11-10T03:15:22.2690224Z {"message":"For more information about this error, try `rustc --explain E0463`.","code":null,"level":"failure-note","spans":[],"children":[],"rendered":"For more information about this error, try `rustc --explain E0463`.\n"}
2019-11-10T03:15:22.2690263Z 
2019-11-10T03:15:22.2690438Z ------------------------------------------
2019-11-10T03:15:22.2690488Z 
---
2019-11-10T03:47:43.9390387Z Verifying status of rustfmt...
2019-11-10T03:47:43.9402132Z Verifying status of clippy-driver...
2019-11-10T03:47:43.9413834Z This PR updated 'src/tools/clippy', verifying if status is 'test-pass'...
2019-11-10T03:47:43.9423662Z 
2019-11-10T03:47:43.9424296Z ⚠️ We detected that this PR updated 'clippy-driver', but its tests failed.
2019-11-10T03:47:43.9424378Z 
2019-11-10T03:47:43.9424631Z If you do intend to update 'clippy-driver', please check the error messages above and
2019-11-10T03:47:43.9424848Z commit another update.
2019-11-10T03:47:43.9424899Z 
2019-11-10T03:47:43.9425117Z If you do NOT intend to update 'clippy-driver', please ensure you did not accidentally
2019-11-10T03:47:43.9425329Z change the submodule at 'src/tools/clippy'. You may ask your reviewer for the
2019-11-10T03:47:43.9425389Z proper steps.
2019-11-10T03:47:43.9449372Z   local time: Sun Nov 10 03:47:43 UTC 2019
2019-11-10T03:47:44.2270600Z   network time: Sun, 10 Nov 2019 03:47:44 GMT
2019-11-10T03:47:44.2275196Z == end clock drift check ==
2019-11-10T03:47:44.7732579Z 
2019-11-10T03:47:44.7732579Z 
2019-11-10T03:47:44.7842104Z ##[error]Bash exited with code '3'.
2019-11-10T03:47:44.7870092Z ##[section]Starting: Checkout
2019-11-10T03:47:44.7871799Z ==============================================================================
2019-11-10T03:47:44.7871990Z Task         : Get sources
2019-11-10T03:47:44.7872029Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
