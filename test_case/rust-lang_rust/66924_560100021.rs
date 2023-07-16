plain
2019-12-01T12:10:21.7538743Z do so (now or later) by using -b with the checkout command again. Example:
2019-12-01T12:10:21.7540322Z 
2019-12-01T12:10:21.7540926Z   git checkout -b <new-branch-name>
2019-12-01T12:10:21.7541343Z 
2019-12-01T12:10:21.7541758Z HEAD is now at c2df83889 Auto merge of #66924 - Centril:rollup-r7pljxh, r=Centril
2019-12-01T12:10:21.7840637Z ##[section]Starting: Setup environment
2019-12-01T12:10:21.7938880Z ==============================================================================
2019-12-01T12:10:21.7938985Z Task         : Bash
2019-12-01T12:10:21.7939056Z Description  : Run a Bash script on macOS, Linux, or Windows
---
2019-12-01T12:10:23.3816205Z 
2019-12-01T12:10:23.3816340Z 
2019-12-01T12:10:23.3816562Z 
2019-12-01T12:10:23.3816778Z 
2019-12-01T12:10:23.3817989Z  - #66346 (Replace .unwrap() with ? in std::os::unix::net)
2019-12-01T12:10:23.3818339Z  - #66789 (rustc: move mir::SourceScopeLocalData to a field of SourceScopeData.)
2019-12-01T12:10:23.3820425Z  - #66822 (libunwind_panic: adjust miri panic hack)
2019-12-01T12:10:23.3820562Z  - #66827 (handle diverging functions forwarding their return place)
2019-12-01T12:10:23.3820892Z  - #66828 (Less minification)
2019-12-01T12:10:23.3821125Z  - #66850 (rustc: hide HirId's fmt::Debug output from -Z span_free_formats.)
2019-12-01T12:10:23.3821240Z  - #66874 (Miri engine: proper support for `Assert` MIR terminators)
2019-12-01T12:10:23.3821349Z  - #66907 (rustc: don't just show raw DefIndex's in BrNamed's fmt::Debug impl.)
2019-12-01T12:10:23.3821530Z AGENT_DISABLELOGPLUGIN_TESTFILEPUBLISHERPLUGIN=true
2019-12-01T12:10:23.3821624Z AGENT_DISABLELOGPLUGIN_TESTRESULTLOGPLUGIN=true
2019-12-01T12:10:23.3821694Z AGENT_HOMEDIRECTORY=C:\agents\2.160.1
2019-12-01T12:10:23.3821773Z AGENT_ID=517
---
2019-12-01T12:10:23.3832224Z BUILD_SOURCEBRANCHNAME=auto
2019-12-01T12:10:23.3832300Z BUILD_SOURCESDIRECTORY=D:\a\1\s
2019-12-01T12:10:23.3832388Z BUILD_SOURCEVERSION=c2df838892902555d7e62c9d3848151df7a606ba
2019-12-01T12:10:23.3832457Z BUILD_SOURCEVERSIONAUTHOR=bors
2019-12-01T12:10:23.3832550Z BUILD_SOURCEVERSIONMESSAGE=Auto merge of #66924 - Centril:rollup-r7pljxh, r=Centril
2019-12-01T12:10:23.3832707Z CI_JOB_NAME=i686-msvc-1
2019-12-01T12:10:23.3832769Z COBERTURA_HOME=C:\cobertura-2.1.1
2019-12-01T12:10:23.3832852Z COMMONPROGRAMFILES=C:\Program Files\Common Files
2019-12-01T12:10:23.3832930Z COMMON_TESTRESULTSDIRECTORY=D:\a\1\TestResults
---
2019-12-01T12:11:49.8604428Z Chocolatey installed 0/1 packages. 1 packages failed.
2019-12-01T12:11:49.8604629Z  See the log for details (C:\ProgramData\chocolatey\logs\chocolatey.log).
2019-12-01T12:11:49.8608091Z 
2019-12-01T12:11:49.8611836Z Failures
2019-12-01T12:11:49.8618080Z  - msys2 (exited 1) - msys2 not installed. An error occurred during installation:
2019-12-01T12:11:49.8618343Z  The remote server returned an error: (503) Server Unavailable. Service Unavailable
2019-12-01T12:11:50.4145519Z 
2019-12-01T12:11:50.4221768Z ##[error]Bash exited with code '1'.
2019-12-01T12:11:50.4335376Z ##[section]Starting: Checkout
2019-12-01T12:11:50.4426693Z ==============================================================================
2019-12-01T12:11:50.4426776Z Task         : Get sources
2019-12-01T12:11:50.4426870Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
