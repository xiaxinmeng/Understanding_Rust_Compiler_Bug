plain
2020-01-16T12:49:11.0630527Z command did not execute successfully: "D:\\a\\1\\s\\build\\i686-pc-windows-msvc\\stage0\\bin\\cargo.exe" "test" "-Zconfig-profile" "--target" "i686-pc-windows-msvc" "-Zbinary-dep-depinfo" "-j" "2" "--release" "--locked" "--color" "always" "--features" "panic-unwind backtrace compiler-builtins-c" "--manifest-path" "D:\\a\\1\\s\\src/libtest/Cargo.toml" "-p" "test" "--"
2020-01-16T12:49:11.0630715Z expected success, got: exit code: 101
2020-01-16T12:49:11.0631278Z 
2020-01-16T12:49:11.0631587Z 
2020-01-16T12:49:11.0631971Z failed to run: D:\a\1\s\build\bootstrap\debug\bootstrap test --exclude src/test/ui --exclude src/test/compile-fail --exclude src/tools/linkchecker
2020-01-16T12:49:11.0632356Z Build completed unsuccessfully in 2:06:07
2020-01-16T12:49:11.0632713Z make: *** [Makefile:80: ci-subset-1] Error 1
2020-01-16T12:49:11.0633586Z   local time: Thu Jan 16 12:49:10 CUT 2020
2020-01-16T12:49:11.2925245Z   network time: Thu, 16 Jan 2020 12:49:11 GMT
2020-01-16T12:49:11.2925472Z == end clock drift check ==
2020-01-16T12:49:11.3736158Z 
2020-01-16T12:49:11.3736158Z 
2020-01-16T12:49:11.6776523Z ##[error]Bash exited with code '2'.
2020-01-16T12:49:11.7612412Z ##[section]Starting: Checkout
2020-01-16T12:49:11.8478002Z ==============================================================================
2020-01-16T12:49:11.8478148Z Task         : Get sources
2020-01-16T12:49:11.8478247Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
