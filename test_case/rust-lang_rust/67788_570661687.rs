plain
2020-01-03T17:46:52.8586999Z command did not execute successfully: "D:\\a\\1\\s\\build\\x86_64-pc-windows-msvc\\stage0\\bin\\cargo.exe" "test" "-Zconfig-profile" "--target" "x86_64-pc-windows-msvc" "-Zbinary-dep-depinfo" "-j" "2" "--release" "--locked" "--color" "always" "--features" "panic-unwind backtrace profiler compiler-builtins-c" "--manifest-path" "D:\\a\\1\\s\\src/libtest/Cargo.toml" "-p" "test" "--"
2020-01-03T17:46:52.8587213Z expected success, got: exit code: 101
2020-01-03T17:46:52.8587258Z 
2020-01-03T17:46:52.8587320Z 
2020-01-03T17:46:52.9880543Z failed to run: D:\a\1\s\build\bootstrap\debug\bootstrap test --exclude src/test/ui --exclude src/test/compile-fail --exclude src/tools/linkchecker
2020-01-03T17:46:52.9880863Z Build completed unsuccessfully in 2:01:26
2020-01-03T17:46:53.0801610Z make: *** [Makefile:80: ci-subset-1] Error 1
2020-01-03T17:46:53.1430849Z   local time: Fri Jan  3 17:46:53 CUT 2020
2020-01-03T17:46:53.5485017Z   network time: Fri, 03 Jan 2020 17:46:53 GMT
2020-01-03T17:46:53.5510578Z == end clock drift check ==
2020-01-03T17:46:53.6701702Z 
2020-01-03T17:46:53.6701702Z 
2020-01-03T17:46:54.0110802Z ##[error]Bash exited with code '2'.
2020-01-03T17:46:54.0626949Z ##[section]Starting: Checkout
2020-01-03T17:46:54.1446024Z ==============================================================================
2020-01-03T17:46:54.1446156Z Task         : Get sources
2020-01-03T17:46:54.1446241Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
