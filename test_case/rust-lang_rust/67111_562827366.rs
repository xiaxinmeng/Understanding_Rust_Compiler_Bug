plain
2019-12-07T05:54:20.1516342Z do so (now or later) by using -b with the checkout command again. Example:
2019-12-07T05:54:20.1516658Z 
2019-12-07T05:54:20.1516813Z   git checkout -b <new-branch-name>
2019-12-07T05:54:20.1517081Z 
2019-12-07T05:54:20.1517289Z HEAD is now at c74d2fa4e Auto merge of #67111 - Centril:rollup-gtd4sad, r=Centril
2019-12-07T05:54:20.1918211Z ##[section]Starting: Setup environment
2019-12-07T05:54:20.2031330Z ==============================================================================
2019-12-07T05:54:20.2031430Z Task         : Bash
2019-12-07T05:54:20.2031496Z Description  : Run a Bash script on macOS, Linux, or Windows
---
2019-12-07T05:54:21.7967093Z  - #66899 (Standard library support for riscv64gc-unknown-linux-gnu)
2019-12-07T05:54:21.7967391Z  - #66927 (Miri core engine: use throw_ub instead of throw_panic)
2019-12-07T05:54:21.7967507Z  - #66984 (Move clean types into their own file)
2019-12-07T05:54:21.7967741Z  - #66991 (Cleanup BodyCache)
2019-12-07T05:54:21.7967846Z  - #67003 (Fix TypedArena returning wrong pointers for recursive allocations)
2019-12-07T05:54:21.7968101Z AGENT_BUILDDIRECTORY=D:\a\1
2019-12-07T05:54:21.7968180Z AGENT_DISABLELOGPLUGIN_TESTFILEPUBLISHERPLUGIN=true
2019-12-07T05:54:21.7968280Z AGENT_DISABLELOGPLUGIN_TESTRESULTLOGPLUGIN=true
2019-12-07T05:54:21.7968371Z AGENT_HOMEDIRECTORY=C:\agents\2.160.1
---
2019-12-07T05:54:21.7981278Z BUILD_SOURCEBRANCHNAME=auto
2019-12-07T05:54:21.7981340Z BUILD_SOURCESDIRECTORY=D:\a\1\s
2019-12-07T05:54:21.7981436Z BUILD_SOURCEVERSION=c74d2fa4e260d42d4f9e86735f8291174a086c15
2019-12-07T05:54:21.7981506Z BUILD_SOURCEVERSIONAUTHOR=bors
2019-12-07T05:54:21.7981654Z BUILD_SOURCEVERSIONMESSAGE=Auto merge of #67111 - Centril:rollup-gtd4sad, r=Centril
2019-12-07T05:54:21.7981820Z CI_JOB_NAME=x86_64-msvc-1
2019-12-07T05:54:21.7981902Z COBERTURA_HOME=C:\cobertura-2.1.1
2019-12-07T05:54:21.7981974Z COMMONPROGRAMFILES=C:\Program Files\Common Files
2019-12-07T05:54:21.7982074Z COMMON_TESTRESULTSDIRECTORY=D:\a\1\TestResults
---
2019-12-07T08:08:19.1933308Z command did not execute successfully: "D:\\a\\1\\s\\build\\bootstrap/debug/rustdoc" "--test" "D:\\a\\1\\s\\src/doc/rustdoc\\src\\documentation-tests.md" "--test-args" ""
2019-12-07T08:08:19.1933430Z expected success, got: exit code: 101
2019-12-07T08:08:19.1933471Z 
2019-12-07T08:08:19.1933499Z 
2019-12-07T08:08:19.2828657Z failed to run: D:\a\1\s\build\bootstrap\debug\bootstrap test --exclude src/test/ui --exclude src/test/compile-fail --exclude src/tools/linkchecker
2019-12-07T08:08:19.2828814Z Build completed unsuccessfully in 2:02:42
2019-12-07T08:08:19.3762399Z make: *** [Makefile:80: ci-subset-1] Error 1
2019-12-07T08:08:19.4252782Z   local time: Sat Dec  7 08:08:19 CUT 2019
2019-12-07T08:08:19.8953729Z   network time: Sat, 07 Dec 2019 08:08:19 GMT
2019-12-07T08:08:19.8978383Z == end clock drift check ==
2019-12-07T08:08:20.0073863Z 
2019-12-07T08:08:20.0073863Z 
2019-12-07T08:08:20.4177103Z ##[error]Bash exited with code '2'.
2019-12-07T08:08:20.4847497Z ##[section]Starting: Checkout
2019-12-07T08:08:20.5775416Z ==============================================================================
2019-12-07T08:08:20.5775551Z Task         : Get sources
2019-12-07T08:08:20.5775635Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
