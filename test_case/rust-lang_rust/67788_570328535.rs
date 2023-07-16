plain
2020-01-02T16:41:46.9609177Z do so (now or later) by using -b with the checkout command again. Example:
2020-01-02T16:41:46.9609229Z 
2020-01-02T16:41:46.9609304Z   git checkout -b <new-branch-name>
2020-01-02T16:41:46.9609349Z 
2020-01-02T16:41:46.9609421Z HEAD is now at fa7756b3d Auto merge of #67788 - cjgillot:delint-day, r=Zoxc
2020-01-02T16:41:47.0009969Z ##[section]Starting: Setup environment
2020-01-02T16:41:47.0126798Z ==============================================================================
2020-01-02T16:41:47.0126895Z Task         : Bash
2020-01-02T16:41:47.0127129Z Description  : Run a Bash script on macOS, Linux, or Windows
---
2020-01-02T18:39:19.1010235Z 
2020-01-02T18:39:19.1011012Z 
2020-01-02T18:39:19.1450941Z failed to run: D:\a\1\s\build\bootstrap\debug\bootstrap test --exclude src/test/ui --exclude src/test/compile-fail
2020-01-02T18:39:19.1451604Z Build completed unsuccessfully in 1:47:18
2020-01-02T18:39:19.2060610Z make: *** [Makefile:89: ci-mingw-subset-1] Error 1
2020-01-02T18:39:19.2663715Z   local time: Thu Jan  2 18:39:19 CUT 2020
2020-01-02T18:39:19.7935403Z   network time: Thu, 02 Jan 2020 18:39:19 GMT
2020-01-02T18:39:19.7936437Z == end clock drift check ==
2020-01-02T18:39:19.8841323Z 
2020-01-02T18:39:19.8841323Z 
2020-01-02T18:39:20.2670901Z ##[error]Bash exited with code '2'.
2020-01-02T18:39:20.3219761Z ##[section]Starting: Checkout
2020-01-02T18:39:20.3925852Z ==============================================================================
2020-01-02T18:39:20.3925952Z Task         : Get sources
2020-01-02T18:39:20.3926061Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
