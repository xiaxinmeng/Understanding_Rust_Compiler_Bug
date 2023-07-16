\n"
2020-03-13T18:36:20.9626098Z   },
2020-03-13T18:36:20.9626340Z   "level": "error",
2020-03-13T18:36:20.9626562Z   "spans": [
2020-03-13T18:36:20.9627125Z       "file_name": "/checkout/src/test/ui/lint/use_suggestion_json.rs",
2020-03-13T18:36:20.9627535Z       "byte_start": 471,
2020-03-13T18:36:20.9627921Z       "byte_end": 475,
2020-03-13T18:36:20.9628262Z       "line_start": 12,
2020-03-13T18:36:20.9628262Z       "line_start": 12,
2020-03-13T18:36:20.9628573Z       "line_end": 12,
2020-03-13T18:36:20.9628851Z       "column_start": 12,
2020-03-13T18:36:20.9629177Z       "column_end": 16,
2020-03-13T18:36:20.9629489Z       "is_primary": true,
2020-03-13T18:36:20.9629785Z       "text": [
2020-03-13T18:36:20.9630350Z         {
2020-03-13T18:36:20.9630664Z           "text": "    let x: Iter;",
2020-03-13T18:36:20.9631035Z           "highlight_start": 12,
2020-03-13T18:36:20.9631394Z           "highlight_end": 16
2020-03-13T18:36:20.9631871Z       ],
2020-03-13T18:36:20.9632081Z       "label": "not found in this scope",
2020-03-13T18:36:20.9632351Z       "suggested_replacement": null,
2020-03-13T18:36:20.9632740Z       "suggestion_applicability": null,
2020-03-13T18:36:20.9632740Z       "suggestion_applicability": null,
2020-03-13T18:36:20.9633082Z       "expansion": null
2020-03-13T18:36:20.9633352Z     }
2020-03-13T18:36:20.9633521Z   ],
2020-03-13T18:36:20.9633772Z   "children": [
2020-03-13T18:36:20.9633953Z     {
2020-03-13T18:36:20.9634392Z       "message": "possible candidates are found in other modules, you can import them into scope",
2020-03-13T18:36:20.9634821Z       "code": null,
2020-03-13T18:36:20.9635148Z       "level": "help",
2020-03-13T18:36:20.9635442Z       "spans": [
2020-03-13T18:36:20.9636022Z           "file_name": "/checkout/src/test/ui/lint/use_suggestion_json.rs",
2020-03-13T18:36:20.9636433Z           "byte_start": 448,
2020-03-13T18:36:20.9636757Z           "byte_end": 448,
2020-03-13T18:36:20.9637025Z           "line_start": 11,
---
2020-03-13T18:36:21.6015617Z       ],
2020-03-13T18:36:21.6015793Z       "children": [],
2020-03-13T18:36:21.6016014Z       "rendered": null
2020-03-13T18:36:21.6016183Z     }
2020-03-13T18:36:21.6016311Z   ],
2020-03-13T18:36:21.6080212Z   "rendered": "\u001b[0m\u001b[1m\u001b[38;5;9merror[E0412]\u001b[0m\u001b[0m\u001b[1m: cannot find type `Iter` in this scope\u001b[0m\n\u001b[0m  \u001b[0m\u001b[0m\u001b[1m\u001b[38;5;12m--> \u001b[0m\u001b[0m/checkout/src/test/ui/lint/use_suggestion_json.rs:12:12\u001b[0m\n\u001b[0m   \u001b[0m\u001b[0m\u001b[1m\u001b[38;5;12m|\u001b[0m\n\u001b[0m\u001b[1m\u001b[38;5;12mLL\u001b[0m\u001b[0m \u001b[0m\u001b[0m\u001b[1m\u001b[38;5;12m| \u001b[0m\u001b[0m    let x: Iter;\u001b[0m\n\u001b[0m   \u001b[0m\u001b[0m\u001b[1m\u001b[38;5;12m| \u001b[0m\u001b[0m           \u001b[0m\u001b[0m\u001b[1m\u001b[38;5;9m^^^^\u001b[0m\u001b[0m \u001b[0m\u001b[0m\u001b[1m\u001b[38;5;9mnot found in this scope\u001b[0m\n\u001b[0m   \u001b[0m\u001b[0m\u001b[1m\u001b[38;5;12m|\u001b[0m\n\u001b[0m\u001b[1m\u001b[38;5;14mhelp\u001b[0m\u001b[0m: possible candidates are found in other modules, you can import them into scope\u001b[0m\n\u001b[0m   \u001b[0m\u001b[0m\u001b[1m\u001b[38;5;12m|\u001b[0m\n\u001b[0m\u001b[1m\u001b[38;5;12mLL\u001b[0m\u001b[0m \u001b[0m\u001b[0m\u001b[1m\u001b[38;5;12m| \u001b[0m\u001b[0muse std::collections::binary_heap::Iter;\u001b[0m\n\u001b[0m   \u001b[0m\u001b[0m\u001b[1m\u001b[38;5;12m|\u001b[0m\n\u001b[0m\u001b[1m\u001b[38;5;12mLL\u001b[0m\u001b[0m \u001b[0m\u001b[0m\u001b[1m\u001b[38;5;12m| \u001b[0m\u001b[0muse std::collections::btree_map::Iter;\u001b[0m\n\u001b[0m   \u001b[0m\u001b[0m\u001b[1m\u001b[38;5;12m|\u001b[0m\n\u001b[0m\u001b[1m\u001b[38;5;12mLL\u001b[0m\u001b[0m \u001b[0m\u001b[0m\u001b[1m\u001b[38;5;12m| \u001b[0m\u001b[0muse std::collections::btree_set::Iter;\u001b[0m\n\u001b[0m   \u001b[0m\u001b[0m\u001b[1m\u001b[38;5;12m|\u001b[0m\n\u001b[0m\u001b[1m\u001b[38;5;12mLL\u001b[0m\u001b[0m \u001b[0m\u001b[0m\u001b[1m\u001b[38;5;12m| \u001b[0m\u001b[0muse std::collections::hash_map::Iter;\u001b[0m\n\u001b[0m   \u001b[0m\u001b[0m\u001b[1m\u001b[38;5;12m|\u001b[0m\n\u001b[0m     and 8 other candidates\u001b[0m\n\n"
2020-03-13T18:36:21.6084776Z {
2020-03-13T18:36:21.6084983Z   "message": "aborting due to previous error",
2020-03-13T18:36:21.6085234Z   "code": null,
2020-03-13T18:36:21.6085234Z   "code": null,
2020-03-13T18:36:21.6085537Z   "level": "error",
2020-03-13T18:36:21.6085713Z   "spans": [],
2020-03-13T18:36:21.6085897Z   "children": [],
2020-03-13T18:36:21.6086273Z   "rendered": "\u001b[0m\u001b[1m\u001b[38;5;9merror\u001b[0m\u001b[0m\u001b[1m: aborting due to previous error\u001b[0m\n\n"
2020-03-13T18:36:21.6086737Z {
2020-03-13T18:36:21.6087405Z   "message": "For more information about this error, try `rustc --explain E0412`.",
2020-03-13T18:36:21.6087683Z   "code": null,
2020-03-13T18:36:21.6088068Z   "level": "failure-note",
2020-03-13T18:36:21.6088068Z   "level": "failure-note",
2020-03-13T18:36:21.6088255Z   "spans": [],
2020-03-13T18:36:21.6088423Z   "children": [],
2020-03-13T18:36:21.6088989Z   "rendered": "\u001b[0m\u001b[1mFor more information about this error, try `rustc --explain E0412`.\u001b[0m\n"
2020-03-13T18:36:21.6089795Z thread '[ui] ui/lint/use_suggestion_json.rs' panicked at 'explicit panic', src/tools/compiletest/src/json.rs:87:21
2020-03-13T18:36:21.6090424Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
2020-03-13T18:36:21.6090661Z 
2020-03-13T18:36:21.6090763Z 
2020-03-13T18:36:21.6090763Z 
2020-03-13T18:36:21.6090892Z failures:
2020-03-13T18:36:21.6091214Z     [ui] ui/lint/use_suggestion_json.rs
2020-03-13T18:36:21.6091367Z 
2020-03-13T18:36:21.6091830Z test result: FAILED. 9713 passed; 1 failed; 57 ignored; 0 measured; 0 filtered out
2020-03-13T18:36:21.6092074Z 
2020-03-13T18:36:21.6092163Z 
2020-03-13T18:36:21.6092269Z 
2020-03-13T18:36:21.6095650Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-7/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "7.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2020-03-13T18:36:21.6104076Z 
2020-03-13T18:36:21.6104172Z 
2020-03-13T18:36:21.6106852Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:348:22
2020-03-13T18:36:21.6107259Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
2020-03-13T18:36:21.6107259Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
2020-03-13T18:36:21.6107584Z Build completed unsuccessfully in 1:00:59
2020-03-13T18:36:21.6107814Z == clock drift check ==
2020-03-13T18:36:21.6108057Z   local time: Fri Mar 13 18:36:20 UTC 2020
2020-03-13T18:36:21.6108338Z   network time: Fri, 13 Mar 2020 18:36:21 GMT
2020-03-13T18:36:21.6108580Z == end clock drift check ==
2020-03-13T18:36:21.8443303Z 
2020-03-13T18:36:21.8520089Z ##[error]Bash exited with code '1'.
2020-03-13T18:36:21.8534143Z ##[section]Finishing: Run build
2020-03-13T18:36:21.8582023Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/69979/merge to s
2020-03-13T18:36:21.8587148Z Task         : Get sources
2020-03-13T18:36:21.8587487Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-03-13T18:36:21.8587822Z Version      : 1.0.0
2020-03-13T18:36:21.8588045Z Author       : Microsoft
2020-03-13T18:36:21.8588045Z Author       : Microsoft
2020-03-13T18:36:21.8588398Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-03-13T18:36:21.8588826Z ==============================================================================
2020-03-13T18:36:22.1792059Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-03-13T18:36:22.1843179Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/69979/merge to s
2020-03-13T18:36:22.1929834Z Cleaning up task key
2020-03-13T18:36:22.1931108Z Start cleaning up orphan processes.
2020-03-13T18:36:22.2312307Z Terminate orphan process: pid (3717) (python)
2020-03-13T18:36:22.2366717Z ##[section]Finishing: Finalize Job
