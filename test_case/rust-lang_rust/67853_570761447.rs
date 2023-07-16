plain
2020-01-04T02:36:40.9462925Z do so (now or later) by using -b with the checkout command again. Example:
2020-01-04T02:36:40.9463305Z 
2020-01-04T02:36:40.9463493Z   git checkout -b <new-branch-name>
2020-01-04T02:36:40.9463630Z 
2020-01-04T02:36:40.9463773Z HEAD is now at b23243dfe Auto merge of #67853 - Centril:rollup-sx5zi9n, r=Centril
2020-01-04T02:36:40.9841919Z ##[section]Starting: Setup environment
2020-01-04T02:36:40.9953792Z ==============================================================================
2020-01-04T02:36:40.9953890Z Task         : Bash
2020-01-04T02:36:40.9953959Z Description  : Run a Bash script on macOS, Linux, or Windows
---
2020-01-04T04:35:33.6919651Z 
2020-01-04T04:35:33.6920279Z 
2020-01-04T04:35:33.7676341Z failed to run: D:\a\1\s\build\bootstrap\debug\bootstrap test --exclude src/test/ui --exclude src/test/compile-fail
2020-01-04T04:35:33.7677086Z Build completed unsuccessfully in 1:48:34
2020-01-04T04:35:33.8233806Z make: *** [Makefile:89: ci-mingw-subset-1] Error 1
2020-01-04T04:35:33.9022986Z   local time: Sat Jan  4 04:35:33 CUT 2020
2020-01-04T04:35:34.3098838Z   network time: Sat, 04 Jan 2020 04:35:34 GMT
2020-01-04T04:35:34.3101742Z == end clock drift check ==
2020-01-04T04:35:34.3834320Z 
2020-01-04T04:35:34.3834320Z 
2020-01-04T04:35:34.7746249Z ##[error]Bash exited with code '2'.
2020-01-04T04:35:34.8121976Z ##[section]Starting: Checkout
2020-01-04T04:35:34.9044403Z ==============================================================================
2020-01-04T04:35:34.9044563Z Task         : Get sources
2020-01-04T04:35:34.9044653Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
