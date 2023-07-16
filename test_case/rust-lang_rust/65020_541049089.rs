plain
2019-10-11T12:43:27.5472124Z failures:
2019-10-11T12:43:27.5472369Z 
2019-10-11T12:43:27.5474520Z ---- [ui] ui/extern/issue-64655-allow-unwind-when-calling-panic-directly.rs#fat stdout ----
2019-10-11T12:43:27.5474680Z 
2019-10-11T12:43:27.5474762Z error in revision `fat`: test run failed!
2019-10-11T12:43:27.5474866Z status: exit code: 101
2019-10-11T12:43:27.5476774Z command: "/emsdk-portable/node/8.9.1_64bit/bin/node" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/extern/issue-64655-allow-unwind-when-calling-panic-directly.fat/a.js"
2019-10-11T12:43:27.5479469Z ------------------------------------------
2019-10-11T12:43:27.5479552Z 
2019-10-11T12:43:27.5481661Z ------------------------------------------
2019-10-11T12:43:27.5481846Z stderr:
2019-10-11T12:43:27.5481846Z stderr:
2019-10-11T12:43:27.5482950Z ------------------------------------------
2019-10-11T12:43:27.5483064Z 
2019-10-11T12:43:27.5483912Z ------------------------------------------
2019-10-11T12:43:27.5485333Z 
2019-10-11T12:43:27.5485404Z 
2019-10-11T12:43:27.5485865Z ---- [ui] ui/extern/issue-64655-allow-unwind-when-calling-panic-directly.rs#no stdout ----
2019-10-11T12:43:27.5485968Z 
2019-10-11T12:43:27.5486041Z error in revision `no`: test run failed!
2019-10-11T12:43:27.5486144Z status: exit code: 101
2019-10-11T12:43:27.5486640Z command: "/emsdk-portable/node/8.9.1_64bit/bin/node" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/extern/issue-64655-allow-unwind-when-calling-panic-directly.no/a.js"
2019-10-11T12:43:27.5487097Z ------------------------------------------
2019-10-11T12:43:27.5487154Z 
2019-10-11T12:43:27.5487866Z ------------------------------------------
2019-10-11T12:43:27.5487960Z stderr:
2019-10-11T12:43:27.5487960Z stderr:
2019-10-11T12:43:27.5488265Z ------------------------------------------
2019-10-11T12:43:27.5488336Z 
2019-10-11T12:43:27.5488611Z ------------------------------------------
2019-10-11T12:43:27.5488688Z 
2019-10-11T12:43:27.5488727Z 
2019-10-11T12:43:27.5489100Z ---- [ui] ui/extern/issue-64655-allow-unwind-when-calling-panic-directly.rs#thin stdout ----
2019-10-11T12:43:27.5489173Z 
2019-10-11T12:43:27.5489264Z error in revision `thin`: test run failed!
2019-10-11T12:43:27.5489345Z status: exit code: 101
2019-10-11T12:43:27.5489837Z command: "/emsdk-portable/node/8.9.1_64bit/bin/node" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/extern/issue-64655-allow-unwind-when-calling-panic-directly.thin/a.js"
2019-10-11T12:43:27.5490270Z ------------------------------------------
2019-10-11T12:43:27.5490326Z 
2019-10-11T12:43:27.5490669Z ------------------------------------------
2019-10-11T12:43:27.5490752Z stderr:
---
2019-10-11T12:43:27.5574392Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:537:22
2019-10-11T12:43:27.5574544Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
2019-10-11T12:43:27.5597493Z 
2019-10-11T12:43:27.5598745Z 
2019-10-11T12:43:27.5606000Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/asmjs-unknown-emscripten/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-asmjs-unknown-emscripten" "--mode" "ui" "--target" "asmjs-unknown-emscripten" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/checkout/obj/build/x86_64-unknown-linux-gnu/llvm/build/bin/FileCheck" "--nodejs" "/emsdk-portable/node/8.9.1_64bit/bin/node" "--host-rustcflags" "-Crpath -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/asmjs-unknown-emscripten/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--llvm-version" "9.0.0-rust-1.40.0-dev\n" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2019-10-11T12:43:27.5606991Z 
2019-10-11T12:43:27.5607036Z 
2019-10-11T12:43:27.5608075Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test --target asmjs-unknown-emscripten src/test/ui src/test/run-fail src/libstd src/liballoc src/libcore
2019-10-11T12:43:27.5608243Z Build completed unsuccessfully in 2:11:15
2019-10-11T12:43:27.5608243Z Build completed unsuccessfully in 2:11:15
2019-10-11T12:43:27.5663065Z == clock drift check ==
2019-10-11T12:43:27.5697193Z   local time: Fri Oct 11 12:43:27 UTC 2019
2019-10-11T12:43:27.6819264Z   network time: Fri, 11 Oct 2019 12:43:27 GMT
2019-10-11T12:43:27.6823077Z == end clock drift check ==
2019-10-11T12:43:28.3024022Z ##[error]Bash exited with code '1'.
2019-10-11T12:43:28.3077334Z ##[section]Starting: Upload CPU usage statistics
2019-10-11T12:43:28.3092971Z ==============================================================================
2019-10-11T12:43:28.3093113Z Task         : Bash
2019-10-11T12:43:28.3093201Z Description  : Run a Bash script on macOS, Linux, or Windows
