plain
2020-01-02T20:11:45.4895874Z do so (now or later) by using -b with the checkout command again. Example:
2020-01-02T20:11:45.4895918Z 
2020-01-02T20:11:45.4895988Z   git checkout -b <new-branch-name>
2020-01-02T20:11:45.4897210Z 
2020-01-02T20:11:45.4897311Z HEAD is now at 36363765f Auto merge of #67815 - Dylan-DPC:rollup-4seh8mx, r=Dylan-DPC
2020-01-02T20:11:45.5336312Z ##[section]Starting: Setup environment
2020-01-02T20:11:45.5448823Z ==============================================================================
2020-01-02T20:11:45.5448916Z Task         : Bash
2020-01-02T20:11:45.5449008Z Description  : Run a Bash script on macOS, Linux, or Windows
---
2020-01-02T22:13:56.3148451Z 
2020-01-02T22:13:56.3148494Z 
2020-01-02T22:13:56.3618053Z failed to run: D:\a\1\s\build\bootstrap\debug\bootstrap test --exclude src/test/ui --exclude src/test/compile-fail
2020-01-02T22:13:56.3618300Z Build completed unsuccessfully in 1:51:53
2020-01-02T22:13:56.4174342Z make: *** [Makefile:89: ci-mingw-subset-1] Error 1
2020-01-02T22:13:56.4793147Z   local time: Thu Jan  2 22:13:56 CUT 2020
2020-01-02T22:13:57.0399582Z   network time: Thu, 02 Jan 2020 22:13:57 GMT
2020-01-02T22:13:57.0399733Z == end clock drift check ==
2020-01-02T22:13:57.0839607Z 
2020-01-02T22:13:57.0839607Z 
2020-01-02T22:13:57.3899359Z ##[error]Bash exited with code '2'.
2020-01-02T22:13:57.4729075Z ##[section]Starting: Checkout
2020-01-02T22:13:57.5899077Z ==============================================================================
2020-01-02T22:13:57.5899253Z Task         : Get sources
2020-01-02T22:13:57.5899347Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
