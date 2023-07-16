\n"
2020-03-11T13:17:18.2064935Z   },
2020-03-11T13:17:18.2065102Z   "level": "error",
2020-03-11T13:17:18.2065299Z   "spans": [
2020-03-11T13:17:18.2065703Z       "file_name": "/checkout/src/test/ui/lint/use_suggestion_json.rs",
2020-03-11T13:17:18.2066172Z       "byte_start": 471,
2020-03-11T13:17:18.2066374Z       "byte_end": 475,
2020-03-11T13:17:18.2066572Z       "line_start": 12,
2020-03-11T13:17:18.2066572Z       "line_start": 12,
2020-03-11T13:17:18.2066789Z       "line_end": 12,
2020-03-11T13:17:18.2066989Z       "column_start": 12,
2020-03-11T13:17:18.2067194Z       "column_end": 16,
2020-03-11T13:17:18.2067412Z       "is_primary": true,
2020-03-11T13:17:18.2067601Z       "text": [
2020-03-11T13:17:18.2067754Z         {
2020-03-11T13:17:18.2067953Z           "text": "    let x: Iter;",
2020-03-11T13:17:18.2068220Z           "highlight_start": 12,
2020-03-11T13:17:18.2068451Z           "highlight_end": 16
2020-03-11T13:17:18.2068782Z       ],
2020-03-11T13:17:18.2068979Z       "label": "not found in this scope",
2020-03-11T13:17:18.2069234Z       "suggested_replacement": null,
2020-03-11T13:17:18.2069508Z       "suggestion_applicability": null,
2020-03-11T13:17:18.2069508Z       "suggestion_applicability": null,
2020-03-11T13:17:18.2069738Z       "expansion": null
2020-03-11T13:17:18.2069899Z     }
2020-03-11T13:17:18.2070036Z   ],
2020-03-11T13:17:18.2070181Z   "children": [
2020-03-11T13:17:18.2070327Z     {
2020-03-11T13:17:18.2070612Z       "message": "possible candidates are found in other modules, you can import them into scope",
2020-03-11T13:17:18.2070950Z       "code": null,
2020-03-11T13:17:18.2071141Z       "level": "help",
2020-03-11T13:17:18.2071344Z       "spans": [
2020-03-11T13:17:18.2071756Z           "file_name": "/checkout/src/test/ui/lint/use_suggestion_json.rs",
2020-03-11T13:17:18.2072053Z           "byte_start": 448,
2020-03-11T13:17:18.2072286Z           "byte_end": 448,
2020-03-11T13:17:18.2072501Z           "line_start": 11,
---
2020-03-11T13:17:18.2139610Z       ],
2020-03-11T13:17:18.2139778Z       "children": [],
2020-03-11T13:17:18.2139997Z       "rendered": null
2020-03-11T13:17:18.2140169Z     }
2020-03-11T13:17:18.2140295Z   ],
2020-03-11T13:17:18.2146968Z   "rendered": "\u001b[0m\u001b[1m\u001b[38;5;9merror[E0412]\u001b[0m\u001b[0m\u001b[1m: cannot find type `Iter` in this scope\u001b[0m\n\u001b[0m  \u001b[0m\u001b[0m\u001b[1m\u001b[38;5;12m--> \u001b[0m\u001b[0m/checkout/src/test/ui/lint/use_suggestion_json.rs:12:12\u001b[0m\n\u001b[0m   \u001b[0m\u001b[0m\u001b[1m\u001b[38;5;12m|\u001b[0m\n\u001b[0m\u001b[1m\u001b[38;5;12mLL\u001b[0m\u001b[0m \u001b[0m\u001b[0m\u001b[1m\u001b[38;5;12m| \u001b[0m\u001b[0m    let x: Iter;\u001b[0m\n\u001b[0m   \u001b[0m\u001b[0m\u001b[1m\u001b[38;5;12m| \u001b[0m\u001b[0m           \u001b[0m\u001b[0m\u001b[1m\u001b[38;5;9m^^^^\u001b[0m\u001b[0m \u001b[0m\u001b[0m\u001b[1m\u001b[38;5;9mnot found in this scope\u001b[0m\n\u001b[0m   \u001b[0m\u001b[0m\u001b[1m\u001b[38;5;12m|\u001b[0m\n\u001b[0m\u001b[1m\u001b[38;5;14mhelp\u001b[0m\u001b[0m: possible candidates are found in other modules, you can import them into scope\u001b[0m\n\u001b[0m   \u001b[0m\u001b[0m\u001b[1m\u001b[38;5;12m|\u001b[0m\n\u001b[0m\u001b[1m\u001b[38;5;12mLL\u001b[0m\u001b[0m \u001b[0m\u001b[0m\u001b[1m\u001b[38;5;12m| \u001b[0m\u001b[0muse std::collections::binary_heap::Iter;\u001b[0m\n\u001b[0m   \u001b[0m\u001b[0m\u001b[1m\u001b[38;5;12m|\u001b[0m\n\u001b[0m\u001b[1m\u001b[38;5;12mLL\u001b[0m\u001b[0m \u001b[0m\u001b[0m\u001b[1m\u001b[38;5;12m| \u001b[0m\u001b[0muse std::collections::btree_map::Iter;\u001b[0m\n\u001b[0m   \u001b[0m\u001b[0m\u001b[1m\u001b[38;5;12m|\u001b[0m\n\u001b[0m\u001b[1m\u001b[38;5;12mLL\u001b[0m\u001b[0m \u001b[0m\u001b[0m\u001b[1m\u001b[38;5;12m| \u001b[0m\u001b[0muse std::collections::btree_set::Iter;\u001b[0m\n\u001b[0m   \u001b[0m\u001b[0m\u001b[1m\u001b[38;5;12m|\u001b[0m\n\u001b[0m\u001b[1m\u001b[38;5;12mLL\u001b[0m\u001b[0m \u001b[0m\u001b[0m\u001b[1m\u001b[38;5;12m| \u001b[0m\u001b[0muse std::collections::hash_map::Iter;\u001b[0m\n\u001b[0m   \u001b[0m\u001b[0m\u001b[1m\u001b[38;5;12m|\u001b[0m\n\u001b[0m     and 8 other candidates\u001b[0m\n\n"
2020-03-11T13:17:18.2151545Z {
2020-03-11T13:17:18.2151736Z   "message": "aborting due to previous error",
2020-03-11T13:17:18.2151973Z   "code": null,
2020-03-11T13:17:18.2151973Z   "code": null,
2020-03-11T13:17:18.2152150Z   "level": "error",
2020-03-11T13:17:18.2152322Z   "spans": [],
2020-03-11T13:17:18.2152509Z   "children": [],
2020-03-11T13:17:18.2153139Z   "rendered": "\u001b[0m\u001b[1m\u001b[38;5;9merror\u001b[0m\u001b[0m\u001b[1m: aborting due to previous error\u001b[0m\n\n"
2020-03-11T13:17:18.2158806Z {
2020-03-11T13:17:18.2159553Z   "message": "For more information about this error, try `rustc --explain E0412`.",
2020-03-11T13:17:18.2159852Z   "code": null,
2020-03-11T13:17:18.2160225Z   "level": "failure-note",
2020-03-11T13:17:18.2160225Z   "level": "failure-note",
2020-03-11T13:17:18.2160411Z   "spans": [],
2020-03-11T13:17:18.2160582Z   "children": [],
2020-03-11T13:17:18.2161156Z   "rendered": "\u001b[0m\u001b[1mFor more information about this error, try `rustc --explain E0412`.\u001b[0m\n"
2020-03-11T13:17:18.2162075Z thread '[ui] ui/lint/use_suggestion_json.rs' panicked at 'explicit panic', src/tools/compiletest/src/json.rs:87:21
2020-03-11T13:17:18.2162984Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
2020-03-11T13:17:18.2163225Z 
2020-03-11T13:17:18.2164008Z 
---
2020-03-11T13:17:18.2165865Z 
2020-03-11T13:17:18.2166343Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:348:22
2020-03-11T13:17:18.2166634Z 
2020-03-11T13:17:18.2166725Z 
2020-03-11T13:17:18.2175239Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-7/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "7.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2020-03-11T13:17:18.2191687Z 
2020-03-11T13:17:18.2191783Z 
2020-03-11T13:17:18.2192032Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
2020-03-11T13:17:18.2192341Z Build completed unsuccessfully in 1:03:02
2020-03-11T13:17:18.2192341Z Build completed unsuccessfully in 1:03:02
2020-03-11T13:17:18.2192812Z == clock drift check ==
2020-03-11T13:17:18.2193105Z   local time: Wed Mar 11 13:17:18 UTC 2020
2020-03-11T13:17:18.4899500Z   network time: Wed, 11 Mar 2020 13:17:18 GMT
2020-03-11T13:17:18.4899802Z == end clock drift check ==
2020-03-11T13:17:18.9342749Z 
2020-03-11T13:17:18.9425883Z ##[error]Bash exited with code '1'.
2020-03-11T13:17:18.9441237Z ##[section]Finishing: Run build
2020-03-11T13:17:18.9499370Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/69917/merge to s
2020-03-11T13:17:18.9504420Z Task         : Get sources
2020-03-11T13:17:18.9504764Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-03-11T13:17:18.9505082Z Version      : 1.0.0
2020-03-11T13:17:18.9505288Z Author       : Microsoft
2020-03-11T13:17:18.9505288Z Author       : Microsoft
2020-03-11T13:17:18.9505618Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-03-11T13:17:18.9506020Z ==============================================================================
2020-03-11T13:17:19.2974452Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-03-11T13:17:19.3039386Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/69917/merge to s
2020-03-11T13:17:19.3129866Z Cleaning up task key
2020-03-11T13:17:19.3131059Z Start cleaning up orphan processes.
2020-03-11T13:17:19.3494641Z Terminate orphan process: pid (5519) (python)
2020-03-11T13:17:19.3550060Z ##[section]Finishing: Finalize Job
