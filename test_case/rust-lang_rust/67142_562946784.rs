plain
2019-12-08T10:54:27.8083829Z do so (now or later) by using -b with the checkout command again. Example:
2019-12-08T10:54:27.8085937Z 
2019-12-08T10:54:27.8087362Z   git checkout -b <new-branch-name>
2019-12-08T10:54:27.8087833Z 
2019-12-08T10:54:27.8087983Z HEAD is now at 194135cd0 Auto merge of #67142 - Centril:rollup-co72dts, r=Centril
2019-12-08T10:54:27.8455395Z ##[section]Starting: Setup environment
2019-12-08T10:54:27.8562901Z ==============================================================================
2019-12-08T10:54:27.8562982Z Task         : Bash
2019-12-08T10:54:27.8563062Z Description  : Run a Bash script on macOS, Linux, or Windows
---
2019-12-08T10:54:29.6417334Z BUILD_SOURCEBRANCHNAME=auto
2019-12-08T10:54:29.6417406Z BUILD_SOURCESDIRECTORY=D:\a\1\s
2019-12-08T10:54:29.6417508Z BUILD_SOURCEVERSION=194135cd01a1867da56aaadee29a662428aba1ee
2019-12-08T10:54:29.6417588Z BUILD_SOURCEVERSIONAUTHOR=bors
2019-12-08T10:54:29.6417700Z BUILD_SOURCEVERSIONMESSAGE=Auto merge of #67142 - Centril:rollup-co72dts, r=Centril
2019-12-08T10:54:29.6417875Z CI_JOB_NAME=i686-msvc-1
2019-12-08T10:54:29.6417961Z COBERTURA_HOME=C:\cobertura-2.1.1
2019-12-08T10:54:29.6418044Z COMMONPROGRAMFILES=C:\Program Files\Common Files
2019-12-08T10:54:29.6418151Z COMMON_TESTRESULTSDIRECTORY=D:\a\1\TestResults
---
2019-12-08T13:09:44.2334607Z command did not execute successfully: "D:\\a\\1\\s\\build\\i686-pc-windows-msvc\\stage0\\bin\\cargo.exe" "test" "-Zconfig-profile" "--target" "i686-pc-windows-msvc" "-Zbinary-dep-depinfo" "-j" "2" "--release" "--locked" "--color" "always" "--features" "panic-unwind backtrace compiler-builtins-c" "--manifest-path" "D:\\a\\1\\s\\src/libtest/Cargo.toml" "-p" "test" "--"
2019-12-08T13:09:44.2335593Z expected success, got: exit code: 101
2019-12-08T13:09:44.2335668Z 
2019-12-08T13:09:44.2335743Z 
2019-12-08T13:09:44.3146789Z failed to run: D:\a\1\s\build\bootstrap\debug\bootstrap test --exclude src/test/ui --exclude src/test/compile-fail --exclude src/tools/linkchecker
2019-12-08T13:09:44.3146945Z Build completed unsuccessfully in 2:03:09
2019-12-08T13:09:44.4000139Z make: *** [Makefile:80: ci-subset-1] Error 1
2019-12-08T13:09:44.4641772Z   local time: Sun Dec  8 13:09:44 CUT 2019
2019-12-08T13:09:44.9825456Z   network time: Sun, 08 Dec 2019 13:09:44 GMT
2019-12-08T13:09:44.9863653Z == end clock drift check ==
2019-12-08T13:09:45.1218760Z 
2019-12-08T13:09:45.1218760Z 
2019-12-08T13:09:45.4790177Z ##[error]Bash exited with code '2'.
2019-12-08T13:09:45.6588270Z ##[section]Starting: Checkout
2019-12-08T13:09:45.7671769Z ==============================================================================
2019-12-08T13:09:45.7671903Z Task         : Get sources
2019-12-08T13:09:45.7671995Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
