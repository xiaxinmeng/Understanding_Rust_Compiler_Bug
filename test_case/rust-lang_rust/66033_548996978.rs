plain
2019-11-02T01:16:43.9449516Z test [ui] ui/parser/macros-no-semicolon.rs ... ok
2019-11-02T01:16:43.9449965Z test [ui] ui/parser/macros-no-semicolon-items.rs ... ok
2019-11-02T01:16:43.9865634Z test [ui] ui/parser/match-arrows-block-then-binop.rs ... ok
2019-11-02T01:16:43.9866080Z test [ui] ui/parser/match-refactor-to-expr.rs ... ok
2019-11-02T01:16:44.0340728Z test [ui] ui/parser/mismatched-braces/missing-close-brace-in-impl-trait.rs ... ok
2019-11-02T01:16:44.0827451Z test [ui] ui/parser/mismatched-braces/missing-close-brace-in-struct.rs ... ok
2019-11-02T01:16:44.1048112Z test [ui] ui/parser/match-vec-invalid.rs ... ok
2019-11-02T01:16:44.1257648Z test [ui] ui/parser/mismatched-braces/missing-close-brace-in-trait.rs ... ok
2019-11-02T01:16:44.1288163Z test [ui] ui/parser/mod_file_not_exist_windows.rs ... ignored
2019-11-02T01:16:44.1514391Z test [ui] ui/parser/mod_file_not_exist.rs ... ok
2019-11-02T01:16:44.1545849Z test [ui] ui/parser/mod_file_with_path_attr.rs ... ok
2019-11-02T01:16:44.1755592Z test [ui] ui/parser/multitrait.rs ... ok
---
2019-11-02T01:21:57.6507317Z failures:
2019-11-02T01:21:57.6529873Z 
2019-11-02T01:21:57.6531100Z ---- [ui] ui/never_type/adjust_never.rs stdout ----
2019-11-02T01:21:57.6531476Z 
2019-11-02T01:21:57.6531995Z error: error pattern 'explicit' not found!
2019-11-02T01:21:57.6532349Z status: exit code: 101
2019-11-02T01:21:57.6533017Z command: "/node-v9.2.0-linux-x64/bin/node" "/checkout/src/etc/wasm32-shim.js" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/never_type/adjust_never/a.wasm"
2019-11-02T01:21:57.6534384Z ------------------------------------------
2019-11-02T01:21:57.6534813Z 
2019-11-02T01:21:57.6535276Z ------------------------------------------
2019-11-02T01:21:57.6535586Z stderr:
2019-11-02T01:21:57.6535586Z stderr:
2019-11-02T01:21:57.6536046Z ------------------------------------------
2019-11-02T01:21:57.6536331Z RuntimeError: unreachable
2019-11-02T01:21:57.6536817Z     at __rust_start_panic (wasm-function[71]:1)
2019-11-02T01:21:57.6537334Z     at rust_panic (wasm-function[66]:39)
2019-11-02T01:21:57.6537894Z     at _ZN3std9panicking20rust_panic_with_hook17h4bf016bb8f901ef3E (wasm-function[61]:327)
2019-11-02T01:21:57.6538481Z     at _ZN3std9panicking11begin_panic17he4feea82f5ca82e0E (wasm-function[0]:49)
2019-11-02T01:21:57.6539050Z     at _ZN12adjust_never4main17h6d2f738735ef194fE (wasm-function[10]:15)
2019-11-02T01:21:57.6539632Z     at _ZN3std2rt10lang_start28_$u7b$$u7b$closure$u7d$$u7d$17h816ca1e94677d36cE (wasm-function[7]:25)
2019-11-02T01:21:57.6541121Z     at _ZN3std10sys_common9backtrace28__rust_begin_short_backtrace17hd40c27b3687892a9E (wasm-function[49]:8)
2019-11-02T01:21:57.6541906Z     at _ZN3std9panicking3try7do_call17h8a23e8e96f92da42E (wasm-function[58]:20)
2019-11-02T01:21:57.6542708Z     at __rust_maybe_catch_panic (wasm-function[70]:5)
2019-11-02T01:21:57.6543376Z     at _ZN3std2rt19lang_start_internal17hb1bffde6f861f36fE (wasm-function[67]:250)
2019-11-02T01:21:57.6543935Z     at main (wasm-function[11]:46)
2019-11-02T01:21:57.6544539Z     at Object.<anonymous> (/checkout/src/etc/wasm32-shim.js:20:20)
2019-11-02T01:21:57.6544887Z     at Module._compile (module.js:641:30)
2019-11-02T01:21:57.6545741Z     at Object.Module._extensions..js (module.js:652:10)
2019-11-02T01:21:57.6546186Z     at Module.load (module.js:560:32)
2019-11-02T01:21:57.6546544Z     at tryModuleLoad (module.js:503:12)
2019-11-02T01:21:57.6546709Z     at Function.Module._load (module.js:495:3)
2019-11-02T01:21:57.6546894Z     at Function.Module.runMain (module.js:682:10)
2019-11-02T01:21:57.6547070Z     at startup (bootstrap_node.js:191:16)
2019-11-02T01:21:57.6547252Z     at bootstrap_node.js:613:3
2019-11-02T01:21:57.6549717Z ------------------------------------------
2019-11-02T01:21:57.6549999Z 
2019-11-02T01:21:57.6550935Z 
2019-11-02T01:21:57.6551702Z ---- [ui] ui/never_type/call-fn-never-arg.rs stdout ----
2019-11-02T01:21:57.6551702Z ---- [ui] ui/never_type/call-fn-never-arg.rs stdout ----
2019-11-02T01:21:57.6551943Z 
2019-11-02T01:21:57.6552337Z error: error pattern 'wowzers!' not found!
2019-11-02T01:21:57.6552571Z status: exit code: 101
2019-11-02T01:21:57.6553132Z command: "/node-v9.2.0-linux-x64/bin/node" "/checkout/src/etc/wasm32-shim.js" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/never_type/call-fn-never-arg/a.wasm"
2019-11-02T01:21:57.6553841Z ------------------------------------------
2019-11-02T01:21:57.6554191Z 
2019-11-02T01:21:57.6554550Z ------------------------------------------
2019-11-02T01:21:57.6554745Z stderr:
2019-11-02T01:21:57.6554745Z stderr:
2019-11-02T01:21:57.6555105Z ------------------------------------------
2019-11-02T01:21:57.6555327Z RuntimeError: unreachable
2019-11-02T01:21:57.6555702Z     at __rust_start_panic (wasm-function[71]:1)
2019-11-02T01:21:57.6556356Z     at rust_panic (wasm-function[66]:39)
2019-11-02T01:21:57.6557148Z     at _ZN3std9panicking20rust_panic_with_hook17h4bf016bb8f901ef3E (wasm-function[61]:327)
2019-11-02T01:21:57.6557643Z     at _ZN3std9panicking11begin_panic17ha1cdba62719ebe21E (wasm-function[2]:49)
2019-11-02T01:21:57.6558324Z     at _ZN17call_fn_never_arg4main17h5384709b6cad65deE (wasm-function[0]:15)
2019-11-02T01:21:57.6558801Z     at _ZN3std2rt10lang_start28_$u7b$$u7b$closure$u7d$$u7d$17h3602d2fe93b3cb75E (wasm-function[7]:25)
2019-11-02T01:21:57.6559310Z     at _ZN3std10sys_common9backtrace28__rust_begin_short_backtrace17hd40c27b3687892a9E (wasm-function[49]:8)
2019-11-02T01:21:57.6560343Z     at _ZN3std9panicking3try7do_call17h8a23e8e96f92da42E (wasm-function[58]:20)
2019-11-02T01:21:57.6566973Z     at __rust_maybe_catch_panic (wasm-function[70]:5)
2019-11-02T01:21:57.6567544Z     at _ZN3std2rt19lang_start_internal17hb1bffde6f861f36fE (wasm-function[67]:250)
2019-11-02T01:21:57.6567842Z     at main (wasm-function[1]:46)
2019-11-02T01:21:57.6568127Z     at Object.<anonymous> (/checkout/src/etc/wasm32-shim.js:20:20)
2019-11-02T01:21:57.6568246Z     at Module._compile (module.js:641:30)
2019-11-02T01:21:57.6568349Z     at Object.Module._extensions..js (module.js:652:10)
2019-11-02T01:21:57.6568429Z     at Module.load (module.js:560:32)
2019-11-02T01:21:57.6568519Z     at tryModuleLoad (module.js:503:12)
2019-11-02T01:21:57.6568981Z     at Function.Module._load (module.js:495:3)
2019-11-02T01:21:57.6569087Z     at Function.Module.runMain (module.js:682:10)
2019-11-02T01:21:57.6569339Z     at startup (bootstrap_node.js:191:16)
2019-11-02T01:21:57.6569435Z     at bootstrap_node.js:613:3
2019-11-02T01:21:57.6570477Z ------------------------------------------
2019-11-02T01:21:57.6570534Z 
2019-11-02T01:21:57.6570572Z 
2019-11-02T01:21:57.6571011Z ---- [ui] ui/never_type/cast-never.rs stdout ----
2019-11-02T01:21:57.6571011Z ---- [ui] ui/never_type/cast-never.rs stdout ----
2019-11-02T01:21:57.6571077Z 
2019-11-02T01:21:57.6571381Z error: error pattern 'explicit' not found!
2019-11-02T01:21:57.6571460Z status: exit code: 101
2019-11-02T01:21:57.6571995Z command: "/node-v9.2.0-linux-x64/bin/node" "/checkout/src/etc/wasm32-shim.js" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/never_type/cast-never/a.wasm"
2019-11-02T01:21:57.6572373Z ------------------------------------------
2019-11-02T01:21:57.6572423Z 
2019-11-02T01:21:57.6572684Z ------------------------------------------
2019-11-02T01:21:57.6572775Z stderr:
2019-11-02T01:21:57.6572775Z stderr:
2019-11-02T01:21:57.6573011Z ------------------------------------------
2019-11-02T01:21:57.6573105Z RuntimeError: unreachable
2019-11-02T01:21:57.6573526Z     at __rust_start_panic (wasm-function[71]:1)
2019-11-02T01:21:57.6573778Z     at rust_panic (wasm-function[66]:39)
2019-11-02T01:21:57.6576269Z     at _ZN3std9panicking20rust_panic_with_hook17h4bf016bb8f901ef3E (wasm-function[61]:327)
2019-11-02T01:21:57.6576781Z     at _ZN3std9panicking11begin_panic17ha2996a84a4a8d472E (wasm-function[0]:49)
2019-11-02T01:21:57.6577122Z     at _ZN10cast_never4main17hac0b415d9f8ca7faE (wasm-function[5]:15)
2019-11-02T01:21:57.6577485Z     at _ZN3std2rt10lang_start28_$u7b$$u7b$closure$u7d$$u7d$17h8cca5c89d8e7be7dE (wasm-function[9]:25)
2019-11-02T01:21:57.6577860Z     at _ZN3std10sys_common9backtrace28__rust_begin_short_backtrace17hd40c27b3687892a9E (wasm-function[49]:8)
2019-11-02T01:21:57.6578189Z     at _ZN3std9panicking3try7do_call17h8a23e8e96f92da42E (wasm-function[58]:20)
2019-11-02T01:21:57.6578487Z     at __rust_maybe_catch_panic (wasm-function[70]:5)
2019-11-02T01:21:57.6578796Z     at _ZN3std2rt19lang_start_internal17hb1bffde6f861f36fE (wasm-function[67]:250)
2019-11-02T01:21:57.6579076Z     at main (wasm-function[6]:46)
2019-11-02T01:21:57.6579374Z     at Object.<anonymous> (/checkout/src/etc/wasm32-shim.js:20:20)
2019-11-02T01:21:57.6579461Z     at Module._compile (module.js:641:30)
2019-11-02T01:21:57.6579565Z     at Object.Module._extensions..js (module.js:652:10)
2019-11-02T01:21:57.6579645Z     at Module.load (module.js:560:32)
2019-11-02T01:21:57.6579733Z     at tryModuleLoad (module.js:503:12)
2019-11-02T01:21:57.6579819Z     at Function.Module._load (module.js:495:3)
2019-11-02T01:21:57.6579915Z     at Function.Module.runMain (module.js:682:10)
2019-11-02T01:21:57.6580009Z     at startup (bootstrap_node.js:191:16)
2019-11-02T01:21:57.6580083Z     at bootstrap_node.js:613:3
2019-11-02T01:21:57.6580850Z ------------------------------------------
2019-11-02T01:21:57.6580905Z 
2019-11-02T01:21:57.6580960Z 
2019-11-02T01:21:57.6581224Z ---- [ui] ui/never_type/never-associated-type.rs stdout ----
2019-11-02T01:21:57.6581224Z ---- [ui] ui/never_type/never-associated-type.rs stdout ----
2019-11-02T01:21:57.6581280Z 
2019-11-02T01:21:57.6581532Z error: error pattern 'kapow!' not found!
2019-11-02T01:21:57.6581626Z status: exit code: 101
2019-11-02T01:21:57.6582048Z command: "/node-v9.2.0-linux-x64/bin/node" "/checkout/src/etc/wasm32-shim.js" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/never_type/never-associated-type/a.wasm"
2019-11-02T01:21:57.6582430Z ------------------------------------------
2019-11-02T01:21:57.6582507Z 
2019-11-02T01:21:57.6582745Z ------------------------------------------
2019-11-02T01:21:57.6582839Z stderr:
2019-11-02T01:21:57.6582839Z stderr:
2019-11-02T01:21:57.6583073Z ------------------------------------------
2019-11-02T01:21:57.6583169Z RuntimeError: unreachable
2019-11-02T01:21:57.6583427Z     at __rust_start_panic (wasm-function[72]:1)
2019-11-02T01:21:57.6583858Z     at rust_panic (wasm-function[67]:39)
2019-11-02T01:21:57.6584317Z     at _ZN3std9panicking20rust_panic_with_hook17h4bf016bb8f901ef3E (wasm-function[62]:327)
2019-11-02T01:21:57.6584621Z     at _ZN3std9panicking11begin_panic17ha08bf4c06a1dcf59E (wasm-function[5]:49)
2019-11-02T01:21:57.6584983Z     at _ZN74_$LT$never_associated_type..Blah$u20$as$u20$never_associated_type..Foo$GT$4smeg17h8ece16706c6bd11fE (wasm-function[0]:15)
2019-11-02T01:21:57.6585413Z     at _ZN21never_associated_type4main17h3765ccaae976c2b3E (wasm-function[1]:1)
2019-11-02T01:21:57.6585794Z     at _ZN3std2rt10lang_start28_$u7b$$u7b$closure$u7d$$u7d$17h2e53d26935bed1d5E (wasm-function[10]:25)
2019-11-02T01:21:57.6586436Z     at _ZN3std10sys_common9backtrace28__rust_begin_short_backtrace17hd40c27b3687892a9E (wasm-function[50]:8)
2019-11-02T01:21:57.6586744Z     at _ZN3std9panicking3try7do_call17h8a23e8e96f92da42E (wasm-function[59]:20)
2019-11-02T01:21:57.6587021Z     at __rust_maybe_catch_panic (wasm-function[71]:5)
2019-11-02T01:21:57.6587309Z     at _ZN3std2rt19lang_start_internal17hb1bffde6f861f36fE (wasm-function[68]:250)
2019-11-02T01:21:57.6587566Z     at main (wasm-function[2]:46)
2019-11-02T01:21:57.6587986Z     at Object.<anonymous> (/checkout/src/etc/wasm32-shim.js:20:20)
2019-11-02T01:21:57.6588082Z     at Module._compile (module.js:641:30)
2019-11-02T01:21:57.6588173Z     at Object.Module._extensions..js (module.js:652:10)
2019-11-02T01:21:57.6588244Z     at Module.load (module.js:560:32)
2019-11-02T01:21:57.6588336Z     at tryModuleLoad (module.js:503:12)
2019-11-02T01:21:57.6588407Z     at Function.Module._load (module.js:495:3)
2019-11-02T01:21:57.6588496Z     at Function.Module.runMain (module.js:682:10)
2019-11-02T01:21:57.6588572Z     at startup (bootstrap_node.js:191:16)
2019-11-02T01:21:57.6588859Z ------------------------------------------
2019-11-02T01:21:57.6588924Z 
2019-11-02T01:21:57.6588958Z 
2019-11-02T01:21:57.6589186Z ---- [ui] ui/never_type/never-type-arg.rs stdout ----
2019-11-02T01:21:57.6589186Z ---- [ui] ui/never_type/never-type-arg.rs stdout ----
2019-11-02T01:21:57.6589252Z 
2019-11-02T01:21:57.6589464Z error: error pattern 'oh no!' not found!
2019-11-02T01:21:57.6589548Z status: exit code: 101
2019-11-02T01:21:57.6589908Z command: "/node-v9.2.0-linux-x64/bin/node" "/checkout/src/etc/wasm32-shim.js" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/never_type/never-type-arg/a.wasm"
2019-11-02T01:21:57.6590782Z ------------------------------------------
2019-11-02T01:21:57.6590839Z 
2019-11-02T01:21:57.6591091Z ------------------------------------------
2019-11-02T01:21:57.6591185Z stderr:
2019-11-02T01:21:57.6591185Z stderr:
2019-11-02T01:21:57.6591422Z ------------------------------------------
2019-11-02T01:21:57.6591525Z RuntimeError: unreachable
2019-11-02T01:21:57.6591799Z     at __rust_start_panic (wasm-function[71]:1)
2019-11-02T01:21:57.6592050Z     at rust_panic (wasm-function[66]:39)
2019-11-02T01:21:57.6592379Z     at _ZN3std9panicking20rust_panic_with_hook17h4bf016bb8f901ef3E (wasm-function[61]:327)
2019-11-02T01:21:57.6592695Z     at _ZN3std9panicking11begin_panic17h59d3fbd47e6d450bE (wasm-function[4]:49)
2019-11-02T01:21:57.6593022Z     at _ZN14never_type_arg4main17h84add72622d7bf33E (wasm-function[0]:15)
2019-11-02T01:21:57.6593362Z     at _ZN3std2rt10lang_start28_$u7b$$u7b$closure$u7d$$u7d$17h9b1e6a20c60c2dc1E (wasm-function[9]:25)
2019-11-02T01:21:57.6593741Z     at _ZN3std10sys_common9backtrace28__rust_begin_short_backtrace17hd40c27b3687892a9E (wasm-function[49]:8)
2019-11-02T01:21:57.6594233Z     at _ZN3std9panicking3try7do_call17h8a23e8e96f92da42E (wasm-function[58]:20)
2019-11-02T01:21:57.6594496Z     at __rust_maybe_catch_panic (wasm-function[70]:5)
2019-11-02T01:21:57.6594810Z     at _ZN3std2rt19lang_start_internal17hb1bffde6f861f36fE (wasm-function[67]:250)
2019-11-02T01:21:57.6595047Z     at main (wasm-function[1]:46)
2019-11-02T01:21:57.6595324Z     at Object.<anonymous> (/checkout/src/etc/wasm32-shim.js:20:20)
2019-11-02T01:21:57.6595422Z     at Module._compile (module.js:641:30)
2019-11-02T01:21:57.6595499Z     at Object.Module._extensions..js (module.js:652:10)
2019-11-02T01:21:57.6595587Z     at Module.load (module.js:560:32)
2019-11-02T01:21:57.6595657Z     at tryModuleLoad (module.js:503:12)
2019-11-02T01:21:57.6595747Z     at Function.Module._load (module.js:495:3)
2019-11-02T01:21:57.6595822Z     at Function.Module.runMain (module.js:682:10)
2019-11-02T01:21:57.6596090Z     at startup (bootstrap_node.js:191:16)
2019-11-02T01:21:57.6596169Z     at bootstrap_node.js:613:3
2019-11-02T01:21:57.6596605Z ------------------------------------------
2019-11-02T01:21:57.6596684Z 
2019-11-02T01:21:57.6596717Z 
2019-11-02T01:21:57.6596750Z 
---
2019-11-02T01:21:57.6598645Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:537:22
2019-11-02T01:21:57.6598762Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
2019-11-02T01:21:57.6598818Z 
2019-11-02T01:21:57.6598868Z 
2019-11-02T01:21:57.6601224Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-unknown/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-wasm32-unknown-unknown" "--mode" "ui" "--target" "wasm32-unknown-unknown" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/checkout/obj/build/x86_64-unknown-linux-gnu/llvm/build/bin/FileCheck" "--nodejs" "/node-v9.2.0-linux-x64/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/wasm32-unknown-unknown/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--llvm-version" "9.0.0-rust-1.40.0-dev\n" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2019-11-02T01:21:57.6601938Z 
2019-11-02T01:21:57.6601997Z 
2019-11-02T01:21:57.6602525Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test --target wasm32-unknown-unknown src/test/run-make src/test/ui src/test/compile-fail src/test/mir-opt src/test/codegen-units src/libcore
2019-11-02T01:21:57.6602674Z Build completed unsuccessfully in 1:26:28
2019-11-02T01:21:57.6602674Z Build completed unsuccessfully in 1:26:28
2019-11-02T01:21:57.6644659Z == clock drift check ==
2019-11-02T01:21:57.6671705Z   local time: Sat Nov  2 01:21:57 UTC 2019
2019-11-02T01:21:58.0338494Z   network time: Sat, 02 Nov 2019 01:21:58 GMT
2019-11-02T01:21:58.0345269Z == end clock drift check ==
2019-11-02T01:21:58.8663833Z 
2019-11-02T01:21:58.8743508Z ##[error]Bash exited with code '1'.
2019-11-02T01:21:58.8777907Z ##[section]Starting: Checkout
2019-11-02T01:21:58.8779581Z ==============================================================================
2019-11-02T01:21:58.8779658Z Task         : Get sources
2019-11-02T01:21:58.8779745Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
