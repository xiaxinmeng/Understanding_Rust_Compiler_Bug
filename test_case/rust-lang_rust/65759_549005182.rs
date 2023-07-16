plain
2019-11-02T03:11:15.7843600Z failures:
2019-11-02T03:11:15.7863280Z 
2019-11-02T03:11:15.7863957Z ---- [ui] ui/never_type/adjust_never.rs stdout ----
2019-11-02T03:11:15.7864190Z 
2019-11-02T03:11:15.7864747Z error: error pattern 'explicit' not found!
2019-11-02T03:11:15.7865006Z status: exit code: 101
2019-11-02T03:11:15.7865704Z command: "/node-v9.2.0-linux-x64/bin/node" "/checkout/src/etc/wasm32-shim.js" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/never_type/adjust_never/a.wasm"
2019-11-02T03:11:15.7866351Z ------------------------------------------
2019-11-02T03:11:15.7866509Z 
2019-11-02T03:11:15.7866866Z ------------------------------------------
2019-11-02T03:11:15.7867050Z stderr:
2019-11-02T03:11:15.7867050Z stderr:
2019-11-02T03:11:15.7867402Z ------------------------------------------
2019-11-02T03:11:15.7867610Z RuntimeError: unreachable
2019-11-02T03:11:15.7867969Z     at __rust_start_panic (wasm-function[71]:1)
2019-11-02T03:11:15.7868360Z     at rust_panic (wasm-function[66]:39)
2019-11-02T03:11:15.7868794Z     at _ZN3std9panicking20rust_panic_with_hook17h4bf016bb8f901ef3E (wasm-function[61]:327)
2019-11-02T03:11:15.7869310Z     at _ZN3std9panicking11begin_panic17he4feea82f5ca82e0E (wasm-function[0]:49)
2019-11-02T03:11:15.7869974Z     at _ZN12adjust_never4main17h6d2f738735ef194fE (wasm-function[10]:15)
2019-11-02T03:11:15.7870556Z     at _ZN3std2rt10lang_start28_$u7b$$u7b$closure$u7d$$u7d$17h816ca1e94677d36cE (wasm-function[7]:25)
2019-11-02T03:11:15.7872068Z     at _ZN3std10sys_common9backtrace28__rust_begin_short_backtrace17hd40c27b3687892a9E (wasm-function[49]:8)
2019-11-02T03:11:15.7872662Z     at _ZN3std9panicking3try7do_call17h8a23e8e96f92da42E (wasm-function[58]:20)
2019-11-02T03:11:15.7873119Z     at __rust_maybe_catch_panic (wasm-function[70]:5)
2019-11-02T03:11:15.7873618Z     at _ZN3std2rt19lang_start_internal17hb1bffde6f861f36fE (wasm-function[67]:250)
2019-11-02T03:11:15.7874037Z     at main (wasm-function[11]:46)
2019-11-02T03:11:15.7874504Z     at Object.<anonymous> (/checkout/src/etc/wasm32-shim.js:20:20)
2019-11-02T03:11:15.7874936Z     at Module._compile (module.js:641:30)
2019-11-02T03:11:15.7875115Z     at Object.Module._extensions..js (module.js:652:10)
2019-11-02T03:11:15.7875301Z     at Module.load (module.js:560:32)
2019-11-02T03:11:15.7875497Z     at tryModuleLoad (module.js:503:12)
2019-11-02T03:11:15.7875681Z     at Function.Module._load (module.js:495:3)
2019-11-02T03:11:15.7875871Z     at Function.Module.runMain (module.js:682:10)
2019-11-02T03:11:15.7876231Z     at startup (bootstrap_node.js:191:16)
2019-11-02T03:11:15.7876410Z     at bootstrap_node.js:613:3
2019-11-02T03:11:15.7877106Z ------------------------------------------
2019-11-02T03:11:15.7877266Z 
2019-11-02T03:11:15.7877403Z 
2019-11-02T03:11:15.7877751Z ---- [ui] ui/never_type/cast-never.rs stdout ----
2019-11-02T03:11:15.7877751Z ---- [ui] ui/never_type/cast-never.rs stdout ----
2019-11-02T03:11:15.7877927Z 
2019-11-02T03:11:15.7878266Z error: error pattern 'explicit' not found!
2019-11-02T03:11:15.7878468Z status: exit code: 101
2019-11-02T03:11:15.7878951Z command: "/node-v9.2.0-linux-x64/bin/node" "/checkout/src/etc/wasm32-shim.js" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/never_type/cast-never/a.wasm"
2019-11-02T03:11:15.7879559Z ------------------------------------------
2019-11-02T03:11:15.7880091Z 
2019-11-02T03:11:15.7880492Z ------------------------------------------
2019-11-02T03:11:15.7880692Z stderr:
2019-11-02T03:11:15.7880692Z stderr:
2019-11-02T03:11:15.7881536Z ------------------------------------------
2019-11-02T03:11:15.7881765Z RuntimeError: unreachable
2019-11-02T03:11:15.7882188Z     at __rust_start_panic (wasm-function[71]:1)
2019-11-02T03:11:15.7882624Z     at rust_panic (wasm-function[66]:39)
2019-11-02T03:11:15.7883102Z     at _ZN3std9panicking20rust_panic_with_hook17h4bf016bb8f901ef3E (wasm-function[61]:327)
2019-11-02T03:11:15.7884856Z     at _ZN3std9panicking11begin_panic17ha2996a84a4a8d472E (wasm-function[0]:49)
2019-11-02T03:11:15.7885182Z     at _ZN10cast_never4main17hac0b415d9f8ca7faE (wasm-function[5]:15)
2019-11-02T03:11:15.7885522Z     at _ZN3std2rt10lang_start28_$u7b$$u7b$closure$u7d$$u7d$17h8cca5c89d8e7be7dE (wasm-function[9]:25)
2019-11-02T03:11:15.7885863Z     at _ZN3std10sys_common9backtrace28__rust_begin_short_backtrace17hd40c27b3687892a9E (wasm-function[49]:8)
2019-11-02T03:11:15.7886219Z     at _ZN3std9panicking3try7do_call17h8a23e8e96f92da42E (wasm-function[58]:20)
2019-11-02T03:11:15.7886494Z     at __rust_maybe_catch_panic (wasm-function[70]:5)
2019-11-02T03:11:15.7886805Z     at _ZN3std2rt19lang_start_internal17hb1bffde6f861f36fE (wasm-function[67]:250)
2019-11-02T03:11:15.7887068Z     at main (wasm-function[6]:46)
2019-11-02T03:11:15.7887335Z     at Object.<anonymous> (/checkout/src/etc/wasm32-shim.js:20:20)
2019-11-02T03:11:15.7887438Z     at Module._compile (module.js:641:30)
2019-11-02T03:11:15.7887514Z     at Object.Module._extensions..js (module.js:652:10)
2019-11-02T03:11:15.7887605Z     at Module.load (module.js:560:32)
2019-11-02T03:11:15.7887674Z     at tryModuleLoad (module.js:503:12)
2019-11-02T03:11:15.7887764Z     at Function.Module._load (module.js:495:3)
2019-11-02T03:11:15.7887844Z     at Function.Module.runMain (module.js:682:10)
2019-11-02T03:11:15.7887933Z     at startup (bootstrap_node.js:191:16)
2019-11-02T03:11:15.7888136Z     at bootstrap_node.js:613:3
2019-11-02T03:11:15.7888460Z ------------------------------------------
2019-11-02T03:11:15.7888530Z 
2019-11-02T03:11:15.7888565Z 
2019-11-02T03:11:15.7888826Z ---- [ui] ui/never_type/call-fn-never-arg.rs stdout ----
2019-11-02T03:11:15.7888826Z ---- [ui] ui/never_type/call-fn-never-arg.rs stdout ----
2019-11-02T03:11:15.7888879Z 
2019-11-02T03:11:15.7889102Z error: error pattern 'wowzers!' not found!
2019-11-02T03:11:15.7889194Z status: exit code: 101
2019-11-02T03:11:15.7889594Z command: "/node-v9.2.0-linux-x64/bin/node" "/checkout/src/etc/wasm32-shim.js" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/never_type/call-fn-never-arg/a.wasm"
2019-11-02T03:11:15.7889950Z ------------------------------------------
2019-11-02T03:11:15.7889999Z 
2019-11-02T03:11:15.7890241Z ------------------------------------------
2019-11-02T03:11:15.7890312Z stderr:
2019-11-02T03:11:15.7890312Z stderr:
2019-11-02T03:11:15.7890552Z ------------------------------------------
2019-11-02T03:11:15.7890624Z RuntimeError: unreachable
2019-11-02T03:11:15.7890888Z     at __rust_start_panic (wasm-function[71]:1)
2019-11-02T03:11:15.7891591Z     at rust_panic (wasm-function[66]:39)
2019-11-02T03:11:15.7895038Z     at _ZN3std9panicking20rust_panic_with_hook17h4bf016bb8f901ef3E (wasm-function[61]:327)
2019-11-02T03:11:15.7895546Z     at _ZN3std9panicking11begin_panic17ha1cdba62719ebe21E (wasm-function[2]:49)
2019-11-02T03:11:15.7895847Z     at _ZN17call_fn_never_arg4main17h5384709b6cad65deE (wasm-function[0]:15)
2019-11-02T03:11:15.7896183Z     at _ZN3std2rt10lang_start28_$u7b$$u7b$closure$u7d$$u7d$17h3602d2fe93b3cb75E (wasm-function[7]:25)
2019-11-02T03:11:15.7896512Z     at _ZN3std10sys_common9backtrace28__rust_begin_short_backtrace17hd40c27b3687892a9E (wasm-function[49]:8)
2019-11-02T03:11:15.7896831Z     at _ZN3std9panicking3try7do_call17h8a23e8e96f92da42E (wasm-function[58]:20)
2019-11-02T03:11:15.7897090Z     at __rust_maybe_catch_panic (wasm-function[70]:5)
2019-11-02T03:11:15.7897396Z     at _ZN3std2rt19lang_start_internal17hb1bffde6f861f36fE (wasm-function[67]:250)
2019-11-02T03:11:15.7897846Z     at main (wasm-function[1]:46)
2019-11-02T03:11:15.7898117Z     at Object.<anonymous> (/checkout/src/etc/wasm32-shim.js:20:20)
2019-11-02T03:11:15.7898218Z     at Module._compile (module.js:641:30)
2019-11-02T03:11:15.7898293Z     at Object.Module._extensions..js (module.js:652:10)
2019-11-02T03:11:15.7898381Z     at Module.load (module.js:560:32)
2019-11-02T03:11:15.7898447Z     at tryModuleLoad (module.js:503:12)
2019-11-02T03:11:15.7898538Z     at Function.Module._load (module.js:495:3)
2019-11-02T03:11:15.7898629Z     at Function.Module.runMain (module.js:682:10)
2019-11-02T03:11:15.7898699Z     at startup (bootstrap_node.js:191:16)
2019-11-02T03:11:15.7898781Z     at bootstrap_node.js:613:3
2019-11-02T03:11:15.7899065Z ------------------------------------------
2019-11-02T03:11:15.7899114Z 
2019-11-02T03:11:15.7899147Z 
2019-11-02T03:11:15.7899403Z ---- [ui] ui/never_type/never-associated-type.rs stdout ----
2019-11-02T03:11:15.7899403Z ---- [ui] ui/never_type/never-associated-type.rs stdout ----
2019-11-02T03:11:15.7899463Z 
2019-11-02T03:11:15.7899750Z error: error pattern 'kapow!' not found!
2019-11-02T03:11:15.7899826Z status: exit code: 101
2019-11-02T03:11:15.7900241Z command: "/node-v9.2.0-linux-x64/bin/node" "/checkout/src/etc/wasm32-shim.js" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/never_type/never-associated-type/a.wasm"
2019-11-02T03:11:15.7900608Z ------------------------------------------
2019-11-02T03:11:15.7900660Z 
2019-11-02T03:11:15.7901359Z ------------------------------------------
2019-11-02T03:11:15.7901444Z stderr:
2019-11-02T03:11:15.7901444Z stderr:
2019-11-02T03:11:15.7902116Z ------------------------------------------
2019-11-02T03:11:15.7902202Z RuntimeError: unreachable
2019-11-02T03:11:15.7902488Z     at __rust_start_panic (wasm-function[72]:1)
2019-11-02T03:11:15.7902764Z     at rust_panic (wasm-function[67]:39)
2019-11-02T03:11:15.7903090Z     at _ZN3std9panicking20rust_panic_with_hook17h4bf016bb8f901ef3E (wasm-function[62]:327)
2019-11-02T03:11:15.7903581Z     at _ZN3std9panicking11begin_panic17ha08bf4c06a1dcf59E (wasm-function[5]:49)
2019-11-02T03:11:15.7904052Z     at _ZN74_$LT$never_associated_type..Blah$u20$as$u20$never_associated_type..Foo$GT$4smeg17h8ece16706c6bd11fE (wasm-function[0]:15)
2019-11-02T03:11:15.7904582Z     at _ZN21never_associated_type4main17h3765ccaae976c2b3E (wasm-function[1]:1)
2019-11-02T03:11:15.7905162Z     at _ZN3std2rt10lang_start28_$u7b$$u7b$closure$u7d$$u7d$17h2e53d26935bed1d5E (wasm-function[10]:25)
2019-11-02T03:11:15.7909104Z     at _ZN3std10sys_common9backtrace28__rust_begin_short_backtrace17hd40c27b3687892a9E (wasm-function[50]:8)
2019-11-02T03:11:15.7909462Z     at _ZN3std9panicking3try7do_call17h8a23e8e96f92da42E (wasm-function[59]:20)
2019-11-02T03:11:15.7909729Z     at __rust_maybe_catch_panic (wasm-function[71]:5)
2019-11-02T03:11:15.7910034Z     at _ZN3std2rt19lang_start_internal17hb1bffde6f861f36fE (wasm-function[68]:250)
2019-11-02T03:11:15.7910270Z     at main (wasm-function[2]:46)
2019-11-02T03:11:15.7910563Z     at Object.<anonymous> (/checkout/src/etc/wasm32-shim.js:20:20)
2019-11-02T03:11:15.7910672Z     at Module._compile (module.js:641:30)
2019-11-02T03:11:15.7910747Z     at Object.Module._extensions..js (module.js:652:10)
2019-11-02T03:11:15.7911293Z     at Module.load (module.js:560:32)
2019-11-02T03:11:15.7911378Z     at tryModuleLoad (module.js:503:12)
2019-11-02T03:11:15.7911478Z     at Function.Module._load (module.js:495:3)
2019-11-02T03:11:15.7911559Z     at Function.Module.runMain (module.js:682:10)
2019-11-02T03:11:15.7911656Z     at startup (bootstrap_node.js:191:16)
2019-11-02T03:11:15.7912022Z ------------------------------------------
2019-11-02T03:11:15.7912075Z 
2019-11-02T03:11:15.7912111Z 
2019-11-02T03:11:15.7912392Z ---- [ui] ui/never_type/never-type-arg.rs stdout ----
2019-11-02T03:11:15.7912392Z ---- [ui] ui/never_type/never-type-arg.rs stdout ----
2019-11-02T03:11:15.7912449Z 
2019-11-02T03:11:15.7912954Z error: error pattern 'oh no!' not found!
2019-11-02T03:11:15.7913038Z status: exit code: 101
2019-11-02T03:11:15.7913479Z command: "/node-v9.2.0-linux-x64/bin/node" "/checkout/src/etc/wasm32-shim.js" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/never_type/never-type-arg/a.wasm"
2019-11-02T03:11:15.7914049Z ------------------------------------------
2019-11-02T03:11:15.7914102Z 
2019-11-02T03:11:15.7914363Z ------------------------------------------
2019-11-02T03:11:15.7914620Z stderr:
2019-11-02T03:11:15.7914620Z stderr:
2019-11-02T03:11:15.7914849Z ------------------------------------------
2019-11-02T03:11:15.7915096Z RuntimeError: unreachable
2019-11-02T03:11:15.7915333Z     at __rust_start_panic (wasm-function[71]:1)
2019-11-02T03:11:15.7915587Z     at rust_panic (wasm-function[66]:39)
2019-11-02T03:11:15.7915878Z     at _ZN3std9panicking20rust_panic_with_hook17h4bf016bb8f901ef3E (wasm-function[61]:327)
2019-11-02T03:11:15.7916189Z     at _ZN3std9panicking11begin_panic17h59d3fbd47e6d450bE (wasm-function[4]:49)
2019-11-02T03:11:15.7916475Z     at _ZN14never_type_arg4main17h84add72622d7bf33E (wasm-function[0]:15)
2019-11-02T03:11:15.7916818Z     at _ZN3std2rt10lang_start28_$u7b$$u7b$closure$u7d$$u7d$17h9b1e6a20c60c2dc1E (wasm-function[9]:25)
2019-11-02T03:11:15.7917177Z     at _ZN3std10sys_common9backtrace28__rust_begin_short_backtrace17hd40c27b3687892a9E (wasm-function[49]:8)
2019-11-02T03:11:15.7917476Z     at _ZN3std9panicking3try7do_call17h8a23e8e96f92da42E (wasm-function[58]:20)
2019-11-02T03:11:15.7917750Z     at __rust_maybe_catch_panic (wasm-function[70]:5)
2019-11-02T03:11:15.7918033Z     at _ZN3std2rt19lang_start_internal17hb1bffde6f861f36fE (wasm-function[67]:250)
2019-11-02T03:11:15.7918287Z     at main (wasm-function[1]:46)
2019-11-02T03:11:15.7918563Z     at Object.<anonymous> (/checkout/src/etc/wasm32-shim.js:20:20)
2019-11-02T03:11:15.7918644Z     at Module._compile (module.js:641:30)
2019-11-02T03:11:15.7918735Z     at Object.Module._extensions..js (module.js:652:10)
2019-11-02T03:11:15.7918804Z     at Module.load (module.js:560:32)
2019-11-02T03:11:15.7918887Z     at tryModuleLoad (module.js:503:12)
2019-11-02T03:11:15.7919028Z     at Function.Module._load (module.js:495:3)
2019-11-02T03:11:15.7919135Z     at Function.Module.runMain (module.js:682:10)
2019-11-02T03:11:15.7919222Z     at startup (bootstrap_node.js:191:16)
2019-11-02T03:11:15.7919288Z     at bootstrap_node.js:613:3
2019-11-02T03:11:15.7919595Z ------------------------------------------
2019-11-02T03:11:15.7919642Z 
2019-11-02T03:11:15.7919693Z 
2019-11-02T03:11:15.7919725Z 
---
2019-11-02T03:11:15.7922998Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:537:22
2019-11-02T03:11:15.7923301Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
2019-11-02T03:11:15.7923452Z 
2019-11-02T03:11:15.7923492Z 
2019-11-02T03:11:15.7925572Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-unknown/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-wasm32-unknown-unknown" "--mode" "ui" "--target" "wasm32-unknown-unknown" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/checkout/obj/build/x86_64-unknown-linux-gnu/llvm/build/bin/FileCheck" "--nodejs" "/node-v9.2.0-linux-x64/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/wasm32-unknown-unknown/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--llvm-version" "9.0.0-rust-1.40.0-dev\n" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2019-11-02T03:11:15.7926482Z 
2019-11-02T03:11:15.7926532Z 
2019-11-02T03:11:15.7976666Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test --target wasm32-unknown-unknown src/test/run-make src/test/ui src/test/compile-fail src/test/mir-opt src/test/codegen-units src/libcore
2019-11-02T03:11:15.7977426Z Build completed unsuccessfully in 1:22:31
2019-11-02T03:11:15.7977426Z Build completed unsuccessfully in 1:22:31
2019-11-02T03:11:15.7998027Z == clock drift check ==
2019-11-02T03:11:15.8016960Z   local time: Sat Nov  2 03:11:15 UTC 2019
2019-11-02T03:11:16.0962335Z   network time: Sat, 02 Nov 2019 03:11:16 GMT
2019-11-02T03:11:16.0964487Z == end clock drift check ==
2019-11-02T03:11:17.3513870Z 
2019-11-02T03:11:17.3629941Z ##[error]Bash exited with code '1'.
2019-11-02T03:11:17.3669602Z ##[section]Starting: Checkout
2019-11-02T03:11:17.3672012Z ==============================================================================
2019-11-02T03:11:17.3672112Z Task         : Get sources
2019-11-02T03:11:17.3672218Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
