plain
2019-12-01T09:09:06.3034075Z do so (now or later) by using -b with the checkout command again. Example:
2019-12-01T09:09:06.3034343Z 
2019-12-01T09:09:06.3034525Z   git checkout -b <new-branch-name>
2019-12-01T09:09:06.3034657Z 
2019-12-01T09:09:06.3227009Z HEAD is now at 0e442f7ec Auto merge of #66924 - Centril:rollup-r7pljxh, r=Centril
2019-12-01T09:09:06.3371555Z ##[section]Starting: Setup environment
2019-12-01T09:09:06.3492569Z ==============================================================================
2019-12-01T09:09:06.3492683Z Task         : Bash
2019-12-01T09:09:06.3492777Z Description  : Run a Bash script on macOS, Linux, or Windows
---
2019-12-01T09:09:08.0341850Z 
2019-12-01T09:09:08.0343176Z 
2019-12-01T09:09:08.0349828Z 
2019-12-01T09:09:08.0350139Z 
2019-12-01T09:09:08.0350470Z  - #66346 (Replace .unwrap() with ? in std::os::unix::net)
2019-12-01T09:09:08.0350641Z  - #66789 (rustc: move mir::SourceScopeLocalData to a field of SourceScopeData.)
2019-12-01T09:09:08.0350750Z  - #66822 (libunwind_panic: adjust miri panic hack)
2019-12-01T09:09:08.0350931Z  - #66827 (handle diverging functions forwarding their return place)
2019-12-01T09:09:08.0351031Z  - #66828 (Less minification)
2019-12-01T09:09:08.0351121Z  - #66850 (rustc: hide HirId's fmt::Debug output from -Z span_free_formats.)
2019-12-01T09:09:08.0351258Z  - #66874 (Miri engine: proper support for `Assert` MIR terminators)
2019-12-01T09:09:08.0351359Z  - #66907 (rustc: don't just show raw DefIndex's in BrNamed's fmt::Debug impl.)
2019-12-01T09:09:08.0351852Z AGENT_DISABLELOGPLUGIN_TESTFILEPUBLISHERPLUGIN=true
2019-12-01T09:09:08.0351990Z AGENT_DISABLELOGPLUGIN_TESTRESULTLOGPLUGIN=true
2019-12-01T09:09:08.0352093Z AGENT_HOMEDIRECTORY=C:\agents\2.160.1
2019-12-01T09:09:08.0352163Z AGENT_ID=518
---
2019-12-01T09:09:08.0362257Z BUILD_SOURCEBRANCHNAME=auto
2019-12-01T09:09:08.0362327Z BUILD_SOURCESDIRECTORY=D:\a\1\s
2019-12-01T09:09:08.0362425Z BUILD_SOURCEVERSION=0e442f7ec8b17c274bb505033f69012ec2e28d53
2019-12-01T09:09:08.0362504Z BUILD_SOURCEVERSIONAUTHOR=bors
2019-12-01T09:09:08.0362610Z BUILD_SOURCEVERSIONMESSAGE=Auto merge of #66924 - Centril:rollup-r7pljxh, r=Centril
2019-12-01T09:09:08.0362785Z CI_JOB_NAME=i686-msvc-2
2019-12-01T09:09:08.0362868Z COBERTURA_HOME=C:\cobertura-2.1.1
2019-12-01T09:09:08.0362948Z COMMONPROGRAMFILES=C:\Program Files\Common Files
2019-12-01T09:09:08.0363044Z COMMON_TESTRESULTSDIRECTORY=D:\a\1\TestResults
---
2019-12-01T09:10:38.7149784Z Chocolatey installed 0/1 packages. 1 packages failed.
2019-12-01T09:10:38.7150056Z  See the log for details (C:\ProgramData\chocolatey\logs\chocolatey.log).
2019-12-01T09:10:38.7153425Z 
2019-12-01T09:10:38.7157389Z Failures
2019-12-01T09:10:38.7163222Z  - msys2 (exited 1) - msys2 not installed. An error occurred during installation:
2019-12-01T09:10:38.7163550Z  The remote server returned an error: (503) Server Unavailable. Service Unavailable
2019-12-01T09:10:39.1644377Z 
2019-12-01T09:10:39.1727595Z ##[error]Bash exited with code '1'.
2019-12-01T09:10:39.1858612Z ##[section]Starting: Checkout
2019-12-01T09:10:39.1986621Z ==============================================================================
2019-12-01T09:10:39.1986736Z Task         : Get sources
2019-12-01T09:10:39.1986848Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
