plain
2019-09-01T00:06:22.2228804Z Looks like docker image is the same as before, not uploading
2019-09-01T00:06:28.7893580Z [CI_JOB_NAME=dist-i586-gnu-i586-i686-musl]
2019-09-01T00:06:28.9960710Z [CI_JOB_NAME=dist-i586-gnu-i586-i686-musl]
2019-09-01T00:06:28.9994177Z == clock drift check ==
2019-09-01T00:06:29.0005087Z   local time: Sun Sep  1 00:06:29 UTC 2019
2019-09-01T00:06:29.0228621Z   network time: Sun, 01 Sep 2019 00:06:29 GMT
2019-09-01T00:06:29.0251106Z Starting sccache server...
2019-09-01T00:06:29.1198343Z configure: processing command line
2019-09-01T00:06:29.1199706Z configure: 
2019-09-01T00:06:29.1200824Z configure: rust.dist-src        := False
---
2019-09-01T01:14:43.3091163Z [RUSTC-TIMING] compiletest test:false 17.827
2019-09-01T01:14:43.3119062Z     Finished release [optimized] target(s) in 2m 09s
2019-09-01T01:14:43.3219252Z [TIMING] ToolBuild { compiler: Compiler { stage: 0, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu", tool: "compiletest", path: "src/tools/compiletest", mode: ToolBootstrap, is_optional_tool: false, source_type: InTree, extra_features: [] } -- 129.395
2019-09-01T01:14:43.3346755Z Check compiletest suite=ui mode=ui (x86_64-unknown-linux-gnu -> i586-unknown-linux-gnu)
2019-09-01T01:14:43.3370345Z thread 'main' panicked at 'UnrecognizedOption("C")', src/tools/compiletest/src/main.rs:262:19
2019-09-01T01:14:43.3370870Z 
2019-09-01T01:14:43.3370914Z 
2019-09-01T01:14:43.3370914Z 
2019-09-01T01:14:43.3373110Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/i586-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-i586-unknown-linux-gnu" "--mode" "ui" "--target" "i586-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/checkout/obj/build/x86_64-unknown-linux-gnu/llvm/build/bin/FileCheck" "-Clinker=cc" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/i586-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--llvm-version" "9.0.0-rust-1.39.0-nightly\n" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2019-09-01T01:14:43.3374144Z 
2019-09-01T01:14:43.3374190Z 
2019-09-01T01:14:43.3381708Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test --target i586-unknown-linux-gnu,i686-unknown-linux-musl
2019-09-01T01:14:43.3381903Z Build completed unsuccessfully in 1:05:37
2019-09-01T01:14:43.3381903Z Build completed unsuccessfully in 1:05:37
2019-09-01T01:14:43.3435296Z == clock drift check ==
2019-09-01T01:14:43.3450776Z   local time: Sun Sep  1 01:14:43 UTC 2019
2019-09-01T01:14:43.5355312Z   network time: Sun, 01 Sep 2019 01:14:43 GMT
2019-09-01T01:14:43.5358375Z == end clock drift check ==
2019-09-01T01:14:44.9356275Z ##[error]Bash exited with code '1'.
2019-09-01T01:14:44.9393469Z ##[section]Starting: Upload CPU usage statistics
2019-09-01T01:14:44.9399050Z ==============================================================================
2019-09-01T01:14:44.9399211Z Task         : Bash
2019-09-01T01:14:44.9399284Z Description  : Run a Bash script on macOS, Linux, or Windows
