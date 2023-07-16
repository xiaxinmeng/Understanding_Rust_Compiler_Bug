
2020-07-02T01:23:34.6745917Z failures:
2020-07-02T01:23:34.6776793Z 
2020-07-02T01:23:34.6777741Z ---- [ui] ui/fmt/format-args-capture.rs stdout ----
2020-07-02T01:23:34.6777869Z 
2020-07-02T01:23:34.6778019Z error: test run failed!
2020-07-02T01:23:34.6778161Z status: exit code: 101
2020-07-02T01:23:34.6778651Z command: "/node-v9.2.0-linux-x64/bin/node" "/checkout/src/etc/wasm32-shim.js" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/fmt/format-args-capture/a.wasm"
2020-07-02T01:23:34.6778892Z stdout:
2020-07-02T01:23:34.6779197Z ------------------------------------------
2020-07-02T01:23:34.6779281Z 
2020-07-02T01:23:34.6779718Z ------------------------------------------
2020-07-02T01:23:34.6780143Z stderr:
2020-07-02T01:23:34.6780484Z ------------------------------------------
2020-07-02T01:23:34.6780632Z RuntimeError: unreachable
2020-07-02T01:23:34.6780937Z     at __rust_start_panic (wasm-function[71]:1)
2020-07-02T01:23:34.6784824Z     at rust_panic (wasm-function[67]:39)
2020-07-02T01:23:34.6785686Z     at _ZN3std9panicking20rust_panic_with_hook17h0f0f5f15fb4d96a5E (wasm-function[62]:300)
2020-07-02T01:23:34.6786146Z     at _ZN3std9panicking11begin_panic17hf152d23817f4c89eE.llvm.3685695550328072697 (wasm-function[3]:63)
2020-07-02T01:23:34.6786668Z     at _ZN19format_args_capture49panic_with_single_argument_does_not_get_formatted28_$u7b$$u7b$closure$u7d$$u7d$17hd89a552a94891368E.llvm.3685695550328072697 (wasm-function[5]:1)
2020-07-02T01:23:34.6787056Z     at _ZN3std9panicking3try17h6bf86b1b1823b488E (wasm-function[4]:1)
2020-07-02T01:23:34.6787438Z     at _ZN19format_args_capture4main17h64e7f2babf424377E (wasm-function[1]:639)
2020-07-02T01:23:34.6787869Z     at _ZN3std2rt10lang_start28_$u7b$$u7b$closure$u7d$$u7d$17h9d63bb236469fc68E (wasm-function[9]:25)
2020-07-02T01:23:34.6788295Z     at _ZN3std10sys_common9backtrace28__rust_begin_short_backtrace17h1d43db6386b866d9E (wasm-function[53]:8)
2020-07-02T01:23:34.6788695Z     at _ZN3std2rt19lang_start_internal17h682f4d41ea4333a0E (wasm-function[68]:229)
2020-07-02T01:23:34.6789027Z     at main (wasm-function[2]:46)
2020-07-02T01:23:34.6789368Z     at Object.<anonymous> (/checkout/src/etc/wasm32-shim.js:20:20)
2020-07-02T01:23:34.6789570Z     at Module._compile (module.js:641:30)
2020-07-02T01:23:34.6789733Z     at Object.Module._extensions..js (module.js:652:10)
2020-07-02T01:23:34.6789893Z     at Module.load (module.js:560:32)
2020-07-02T01:23:34.6790045Z     at tryModuleLoad (module.js:503:12)
2020-07-02T01:23:34.6790200Z     at Function.Module._load (module.js:495:3)
2020-07-02T01:23:34.6790356Z     at Function.Module.runMain (module.js:682:10)
2020-07-02T01:23:34.6790510Z     at startup (bootstrap_node.js:191:16)
2020-07-02T01:23:34.6790664Z     at bootstrap_node.js:613:3
2020-07-02T01:23:34.6790757Z 
2020-07-02T01:23:34.6791402Z ------------------------------------------
2020-07-02T01:23:34.6791462Z 
2020-07-02T01:23:34.6791531Z 
2020-07-02T01:23:34.6791593Z 
2020-07-02T01:23:34.6791695Z failures:
2020-07-02T01:23:34.6791923Z     [ui] ui/fmt/format-args-capture.rs
2020-07-02T01:23:34.6791984Z 
2020-07-02T01:23:34.6792268Z test result: [31mFAILED(B[m. 9876 passed; 1 failed; 545 ignored; 0 measured; 0 filtered out
2020-07-02T01:23:34.6792360Z 
2020-07-02T01:23:34.6806229Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:344:22
2020-07-02T01:23:34.6806658Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
2020-07-02T01:23:34.6825558Z 
