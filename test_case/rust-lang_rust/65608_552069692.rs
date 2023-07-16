plain
2019-11-09T04:44:23.0141699Z environment variables:
2019-11-09T04:44:23.1105441Z 
2019-11-09T04:44:23.1106152Z 
2019-11-09T04:44:23.1106333Z 
2019-11-09T04:44:23.1106630Z * Ensures correct evaluation order of assignment expressions where the RHS is a FRU or is a use of a local of reference type.
2019-11-09T04:44:23.1106859Z * Fixes a soundness issue with built-in index operations
2019-11-09T04:44:23.1107116Z * Removes an unnecessary symbol to string conversion
2019-11-09T04:44:23.1107810Z AGENT_DISABLELOGPLUGIN_TESTFILEPUBLISHERPLUGIN=true
2019-11-09T04:44:23.1108781Z AGENT_DISABLELOGPLUGIN_TESTRESULTLOGPLUGIN=true
2019-11-09T04:44:23.1111217Z AGENT_HOMEDIRECTORY=C:\agents\2.160.0
2019-11-09T04:44:23.1111426Z AGENT_ID=512
---
2019-11-09T05:53:36.3184078Z [RUSTC-TIMING] memoffset test:false 0.173
2019-11-09T05:53:39.0131461Z [RUSTC-TIMING] parking_lot_core test:false 2.659
2019-11-09T05:53:39.0316738Z    Compiling synstructure v0.12.1
2019-11-09T05:54:30.8192134Z 
2019-11-09T05:54:30.8192700Z This application has requested the Runtime to terminate it in an unusual way.
2019-11-09T05:54:30.8193298Z Please contact the application's support team for more information.
2019-11-09T05:54:32.5147218Z Assertion failed!
2019-11-09T05:54:32.5147489Z Program: D:\a\1\s\build\i686-pc-windows-gnu\stage1\bin\rustc.exe
2019-11-09T05:54:32.5147489Z Program: D:\a\1\s\build\i686-pc-windows-gnu\stage1\bin\rustc.exe
2019-11-09T05:54:32.5147634Z File: D:/a/1/s/src/llvm-project/llvm/include/llvm/CodeGen/MachineOperand.h, Line 527
2019-11-09T05:54:32.5147703Z 
2019-11-09T05:54:32.5148010Z Expression: isImm() && "Wrong MachineOperand accessor"
2019-11-09T05:54:32.5148240Z error: could not compile `syn`.
2019-11-09T05:54:32.5148344Z warning: build failed, waiting for other jobs to finish...
2019-11-09T05:54:35.6572595Z [RUSTC-TIMING] synstructure test:false 56.599
2019-11-09T05:54:35.6681995Z error: build failed
2019-11-09T05:54:35.6681995Z error: build failed
2019-11-09T05:54:35.6738092Z command did not execute successfully: "D:\\a\\1\\s\\build\\i686-pc-windows-gnu\\stage0\\bin\\cargo.exe" "build" "-Zconfig-profile" "--target" "i686-pc-windows-gnu" "-Zbinary-dep-depinfo" "-j" "2" "--release" "--locked" "--color" "always" "--features" "" "--manifest-path" "D:\\a\\1\\s\\src/rustc/Cargo.toml" "--message-format" "json-render-diagnostics"
2019-11-09T05:54:35.6738651Z expected success, got: exit code: 101
2019-11-09T05:54:35.7687414Z failed to run: D:\a\1\s\build\bootstrap\debug\bootstrap test src/test/ui src/test/compile-fail
2019-11-09T05:54:35.7687742Z Build completed unsuccessfully in 0:59:56
2019-11-09T05:54:35.8262584Z make: *** [Makefile:91: ci-mingw-subset-2] Error 1
2019-11-09T05:54:35.8855194Z   local time: Sat Nov  9 05:54:35 CUT 2019
2019-11-09T05:54:36.4998331Z   network time: Sat, 09 Nov 2019 05:54:36 GMT
2019-11-09T05:54:36.5014168Z == end clock drift check ==
2019-11-09T05:54:36.5876936Z 
2019-11-09T05:54:36.5876936Z 
2019-11-09T05:54:36.9151238Z ##[error]Bash exited with code '2'.
2019-11-09T05:54:36.9588668Z ##[section]Starting: Checkout
2019-11-09T05:54:37.0440331Z ==============================================================================
2019-11-09T05:54:37.0440466Z Task         : Get sources
2019-11-09T05:54:37.0440552Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
