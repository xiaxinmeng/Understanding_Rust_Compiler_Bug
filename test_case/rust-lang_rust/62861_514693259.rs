plain
2019-07-24T15:34:10.7062232Z    Compiling cargo_metadata v0.8.0
2019-07-24T15:34:16.7122479Z [RUSTC-TIMING] toml test:false 17.872
2019-07-24T15:34:32.9154256Z [RUSTC-TIMING] cargo_metadata test:false 22.203
2019-07-24T15:34:32.9200385Z    Compiling clippy_lints v0.0.212 (/checkout/src/tools/clippy/clippy_lints)
2019-07-24T15:34:39.4247780Z error[E0599]: no method named `as_place_ref` found for type `&rustc::mir::Place<'tcx>` in the current scope
2019-07-24T15:34:39.4266122Z     |
2019-07-24T15:34:39.4266122Z     |
2019-07-24T15:34:39.4273068Z 296 |     } = place.as_place_ref();
2019-07-24T15:34:39.4284359Z 
2019-07-24T15:34:39.7678368Z [RUSTC-TIMING] cargo_metadata test:false 23.050
2019-07-24T15:34:40.0541407Z error: aborting due to previous error
2019-07-24T15:34:40.0541533Z 
2019-07-24T15:34:40.0541533Z 
2019-07-24T15:34:40.0577903Z For more information about this error, try `rustc --explain E0599`.
2019-07-24T15:34:40.1238788Z [RUSTC-TIMING] clippy_lints test:false 7.198
2019-07-24T15:34:40.1260968Z error: Could not compile `clippy_lints`.
2019-07-24T15:34:40.1272775Z warning: build failed, waiting for other jobs to finish...
2019-07-24T15:34:46.1533684Z error[E0599]: no method named `as_place_ref` found for type `&rustc::mir::Place<'tcx>` in the current scope
2019-07-24T15:34:46.1535195Z     |
2019-07-24T15:34:46.1535195Z     |
2019-07-24T15:34:46.1535648Z 296 |     } = place.as_place_ref();
2019-07-24T15:34:46.1536568Z 
2019-07-24T15:34:46.7614066Z error: aborting due to previous error
2019-07-24T15:34:46.7614175Z 
2019-07-24T15:34:46.7614457Z For more information about this error, try `rustc --explain E0599`.
---
2019-07-24T15:56:50.1345664Z Verifying status of rustfmt...
2019-07-24T15:56:50.1356836Z Verifying status of clippy-driver...
2019-07-24T15:56:50.1385156Z This PR updated 'src/tools/clippy', verifying if status is 'test-pass'...
2019-07-24T15:56:50.1385277Z 
2019-07-24T15:56:50.1385619Z ⚠️ We detected that this PR updated 'clippy-driver', but its tests failed.
2019-07-24T15:56:50.1385675Z 
2019-07-24T15:56:50.1385960Z If you do intend to update 'clippy-driver', please check the error messages above and
2019-07-24T15:56:50.1386038Z commit another update.
2019-07-24T15:56:50.1386073Z 
2019-07-24T15:56:50.1386340Z If you do NOT intend to update 'clippy-driver', please ensure you did not accidentally
2019-07-24T15:56:50.1386597Z change the submodule at 'src/tools/clippy'. You may ask your reviewer for the
2019-07-24T15:56:50.1386685Z proper steps.
2019-07-24T15:56:51.1519684Z ##[error]Bash exited with code '3'.
2019-07-24T15:56:51.1554147Z ##[section]Starting: Upload CPU usage statistics
2019-07-24T15:56:51.1563242Z ==============================================================================
2019-07-24T15:56:51.1563340Z Task         : Bash
2019-07-24T15:56:51.1563431Z Description  : Run a Bash script on macOS, Linux, or Windows
