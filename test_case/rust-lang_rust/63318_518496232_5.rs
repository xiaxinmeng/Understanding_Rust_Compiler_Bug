\n\nIf the item you are importing is not defined in some super-module of the\ncurrent module, then it must also be declared as public (e.g., `pub fn`).\n"},"level":"error","message":"cannot find function `metabuild` in crate `mb`","spans":[{"byte_end":51,"byte_start":42,"column_end":18,"column_start":9,"expansion":null,"file_name":"target/.metabuild/metabuild-foo-11ea4648a08a0492.rs","is_primary":true,"label":"not found in `mb`","line_end":4,"line_start":4,"suggested_replacement":null,"suggestion_applicability":null,"text":[{"highlight_end":18,"highlight_start":9,"text":"    mb::metabuild();"}]}]}}
2019-08-06T04:41:43.7165724Z {"reason":"compiler-message","package_id":"foo 0.0.1 (path+file:///checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/cit/t932/foo)","target":{"kind":["custom-build"],"crate_types":["bin"],"name":"metabuild-foo","src_path":null,"edition":"2018","doctest":false},"message":{"rendered":"error: aborting due to previous error\n\n","children":[],"code":null,"level":"error","message":"aborting due to previous error","spans":[]}}
2019-08-06T04:41:43.7166562Z {"reason":"compiler-message","package_id":"foo 0.0.1 (path+file:///checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/cit/t932/foo)","target":{"kind":["custom-build"],"crate_types":["bin"],"name":"metabuild-foo","src_path":null,"edition":"2018","doctest":false},"message":{"rendered":"For more information about this error, try `rustc --explain E0425`.\n","children":[],"code":null,"level":"","message":"For more information about this error, try `rustc --explain E0425`.","spans":[]}}
2019-08-06T04:41:43.7166947Z ', src/tools/cargo/tests/testsuite/support/mod.rs:843:13
2019-08-06T04:41:43.7167051Z 
2019-08-06T04:41:43.7167118Z failures:
2019-08-06T04:41:43.7167332Z     check::rustc_check_err
2019-08-06T04:41:43.7167816Z     metabuild::metabuild_failed_build_json
---
2019-08-06T04:41:43.7217525Z 
2019-08-06T04:41:43.7217556Z 
2019-08-06T04:41:43.7227110Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/test/run-fail/pretty src/test/run-pass-valgrind/pretty src/tools/cargo src/tools/cargotest
2019-08-06T04:41:43.7227447Z Build completed unsuccessfully in 1:55:40
2019-08-06T04:41:43.7282554Z Makefile:50: recipe for target 'check-aux' failed
2019-08-06T04:41:43.7282796Z make: *** [check-aux] Error 1
2019-08-06T04:41:50.1992933Z ##[error]Bash exited with code '2'.
2019-08-06T04:41:50.2038016Z ##[section]Starting: Upload CPU usage statistics
2019-08-06T04:41:50.2055751Z ==============================================================================
2019-08-06T04:41:50.2055832Z Task         : Bash
2019-08-06T04:41:50.2055906Z Description  : Run a Bash script on macOS, Linux, or Windows
