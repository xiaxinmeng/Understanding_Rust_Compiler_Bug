plain
2019-11-08T11:10:37.8400851Z do so (now or later) by using -b with the checkout command again. Example:
2019-11-08T11:10:37.8400936Z 
2019-11-08T11:10:37.8401330Z   git checkout -b <new-branch-name>
2019-11-08T11:10:37.8401394Z 
2019-11-08T11:10:37.8402936Z HEAD is now at 3cfba5244 Auto merge of #65608 - matthewjasper:mir-eval-order, r=pnkfelix
2019-11-08T11:10:37.8800790Z ##[section]Starting: Decide whether to run this job
2019-11-08T11:10:37.8900916Z ==============================================================================
2019-11-08T11:10:37.8900998Z Task         : Bash
2019-11-08T11:10:37.8901071Z Description  : Run a Bash script on macOS, Linux, or Windows
---
2019-11-08T11:10:39.2480534Z environment variables:
2019-11-08T11:10:39.3316633Z 
2019-11-08T11:10:39.3317472Z 
2019-11-08T11:10:39.3317627Z 
2019-11-08T11:10:39.3317880Z * Ensures correct evaluation order of assignment expressions where the RHS is a FRU or is a use of a local of reference type.
2019-11-08T11:10:39.3318084Z * Fixes a soundness issue with built-in index operations
2019-11-08T11:10:39.3318253Z * Removes an unnecessary symbol to string conversion
2019-11-08T11:10:39.3318980Z AGENT_DISABLELOGPLUGIN_TESTFILEPUBLISHERPLUGIN=true
2019-11-08T11:10:39.3319151Z AGENT_DISABLELOGPLUGIN_TESTRESULTLOGPLUGIN=true
2019-11-08T11:10:39.3319676Z AGENT_HOMEDIRECTORY=C:\agents\2.160.0
2019-11-08T11:10:39.3319852Z AGENT_ID=514
---
2019-11-08T11:10:39.3332743Z BUILD_SOURCEBRANCHNAME=auto
2019-11-08T11:10:39.3332810Z BUILD_SOURCESDIRECTORY=D:\a\1\s
2019-11-08T11:10:39.3333213Z BUILD_SOURCEVERSION=3cfba5244f4318991c9910d35ed3188e88509881
2019-11-08T11:10:39.3333287Z BUILD_SOURCEVERSIONAUTHOR=bors
2019-11-08T11:10:39.3333389Z BUILD_SOURCEVERSIONMESSAGE=Auto merge of #65608 - matthewjasper:mir-eval-order, r=pnkfelix
2019-11-08T11:10:39.3333713Z COBERTURA_HOME=C:\cobertura-2.1.1
2019-11-08T11:10:39.3333801Z COMMONPROGRAMFILES=C:\Program Files\Common Files
2019-11-08T11:10:39.3333874Z COMMON_TESTRESULTSDIRECTORY=D:\a\1\TestResults
2019-11-08T11:10:39.3333955Z COMPUTERNAME=fv-az425
---
2019-11-08T12:15:23.4667716Z    Compiling termcolor v1.0.4
2019-11-08T12:15:27.7220439Z [RUSTC-TIMING] termcolor test:false 4.236
2019-11-08T12:15:27.7283515Z    Compiling synstructure v0.12.1
2019-11-08T12:16:10.3958396Z 
2019-11-08T12:16:10.3959572Z This application has requested the Runtime to terminate it in an unusual way.
2019-11-08T12:16:10.3959920Z Please contact the application's support team for more information.
2019-11-08T12:16:11.9532144Z Assertion failed!
2019-11-08T12:16:11.9551919Z Program: D:\a\1\s\build\i686-pc-windows-gnu\stage1\bin\rustc.exe
2019-11-08T12:16:11.9551919Z Program: D:\a\1\s\build\i686-pc-windows-gnu\stage1\bin\rustc.exe
2019-11-08T12:16:11.9552236Z File: D:/a/1/s/src/llvm-project/llvm/include/llvm/CodeGen/MachineOperand.h, Line 527
2019-11-08T12:16:11.9552422Z 
2019-11-08T12:16:11.9552621Z Expression: isImm() && "Wrong MachineOperand accessor"
2019-11-08T12:16:11.9568868Z error: could not compile `syn`.
2019-11-08T12:16:12.1581946Z warning: build failed, waiting for other jobs to finish...
2019-11-08T12:16:16.4982569Z [RUSTC-TIMING] synstructure test:false 48.717
2019-11-08T12:16:16.5125522Z error: build failed
2019-11-08T12:16:16.5125522Z error: build failed
2019-11-08T12:16:16.5173902Z command did not execute successfully: "D:\\a\\1\\s\\build\\i686-pc-windows-gnu\\stage0\\bin\\cargo.exe" "build" "-Zconfig-profile" "--target" "i686-pc-windows-gnu" "-Zbinary-dep-depinfo" "-j" "2" "--release" "--locked" "--color" "always" "--features" "" "--manifest-path" "D:\\a\\1\\s\\src/rustc/Cargo.toml" "--message-format" "json-render-diagnostics"
2019-11-08T12:16:16.5174285Z expected success, got: exit code: 101
2019-11-08T12:16:16.6205472Z failed to run: D:\a\1\s\build\bootstrap\debug\bootstrap test src/test/ui src/test/compile-fail
2019-11-08T12:16:16.6205607Z Build completed unsuccessfully in 0:56:01
2019-11-08T12:16:16.6634939Z make: *** [Makefile:91: ci-mingw-subset-2] Error 1
2019-11-08T12:16:16.7413900Z   local time: Fri Nov  8 12:16:16 CUT 2019
2019-11-08T12:16:17.5316691Z   network time: Fri, 08 Nov 2019 12:16:17 GMT
2019-11-08T12:16:17.5339595Z == end clock drift check ==
2019-11-08T12:16:17.6076246Z 
2019-11-08T12:16:17.6076246Z 
2019-11-08T12:16:17.8561892Z ##[error]Bash exited with code '2'.
2019-11-08T12:16:17.9177212Z ##[section]Starting: Checkout
2019-11-08T12:16:17.9968429Z ==============================================================================
2019-11-08T12:16:17.9968559Z Task         : Get sources
2019-11-08T12:16:17.9968639Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
