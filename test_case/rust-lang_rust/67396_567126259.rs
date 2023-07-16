plain
2019-12-18T15:04:41.6741937Z do so (now or later) by using -b with the checkout command again. Example:
2019-12-18T15:04:41.6743277Z 
2019-12-18T15:04:41.6743430Z   git checkout -b <new-branch-name>
2019-12-18T15:04:41.6744317Z 
2019-12-18T15:04:41.6744949Z HEAD is now at b746a58b5 Auto merge of #67396 - Mark-Simulacrum:rollup-85lxz7h, r=Mark-Simulacrum
2019-12-18T15:04:41.7153715Z ##[section]Starting: Setup environment
2019-12-18T15:04:41.7267257Z ==============================================================================
2019-12-18T15:04:41.7267353Z Task         : Bash
2019-12-18T15:04:41.7267593Z Description  : Run a Bash script on macOS, Linux, or Windows
---
2019-12-18T17:07:43.9688576Z 
2019-12-18T17:07:43.9688616Z 
2019-12-18T17:07:44.0267980Z failed to run: D:\a\1\s\build\bootstrap\debug\bootstrap test --exclude src/test/ui --exclude src/test/compile-fail
2019-12-18T17:07:44.0268253Z Build completed unsuccessfully in 1:52:28
2019-12-18T17:07:44.0678290Z make: *** [Makefile:89: ci-mingw-subset-1] Error 1
2019-12-18T17:07:44.1284014Z   local time: Wed Dec 18 17:07:44 CUT 2019
2019-12-18T17:07:44.6572318Z   network time: Wed, 18 Dec 2019 17:07:44 GMT
2019-12-18T17:07:44.6573441Z == end clock drift check ==
2019-12-18T17:07:44.7335947Z 
2019-12-18T17:07:44.7335947Z 
2019-12-18T17:07:45.0825834Z ##[error]Bash exited with code '2'.
2019-12-18T17:07:45.1539621Z ##[section]Starting: Checkout
2019-12-18T17:07:45.2318210Z ==============================================================================
2019-12-18T17:07:45.2318374Z Task         : Get sources
2019-12-18T17:07:45.2318462Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
