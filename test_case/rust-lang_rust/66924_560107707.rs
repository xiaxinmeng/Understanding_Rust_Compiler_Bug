plain
2019-12-01T13:20:03.9383360Z do so (now or later) by using -b with the checkout command again. Example:
2019-12-01T13:20:03.9383518Z 
2019-12-01T13:20:03.9383591Z   git checkout -b <new-branch-name>
2019-12-01T13:20:03.9383637Z 
2019-12-01T13:20:03.9383779Z HEAD is now at f9752418e Auto merge of #66924 - Centril:rollup-r7pljxh, r=Centril
2019-12-01T13:20:03.9711383Z ##[section]Starting: Setup environment
2019-12-01T13:20:03.9830497Z ==============================================================================
2019-12-01T13:20:03.9830608Z Task         : Bash
2019-12-01T13:20:03.9830838Z Description  : Run a Bash script on macOS, Linux, or Windows
---
2019-12-01T13:20:05.8696425Z 
2019-12-01T13:20:05.8696568Z 
2019-12-01T13:20:05.8696718Z 
2019-12-01T13:20:05.8696847Z 
2019-12-01T13:20:05.8697083Z  - #66346 (Replace .unwrap() with ? in std::os::unix::net)
2019-12-01T13:20:05.8697303Z  - #66789 (rustc: move mir::SourceScopeLocalData to a field of SourceScopeData.)
2019-12-01T13:20:05.8698482Z  - #66822 (libunwind_panic: adjust miri panic hack)
2019-12-01T13:20:05.8699378Z  - #66827 (handle diverging functions forwarding their return place)
2019-12-01T13:20:05.8701076Z  - #66828 (Less minification)
2019-12-01T13:20:05.8701361Z  - #66850 (rustc: hide HirId's fmt::Debug output from -Z span_free_formats.)
2019-12-01T13:20:05.8701620Z  - #66874 (Miri engine: proper support for `Assert` MIR terminators)
2019-12-01T13:20:05.8701839Z  - #66907 (rustc: don't just show raw DefIndex's in BrNamed's fmt::Debug impl.)
2019-12-01T13:20:05.8702230Z AGENT_DISABLELOGPLUGIN_TESTFILEPUBLISHERPLUGIN=true
2019-12-01T13:20:05.8702412Z AGENT_DISABLELOGPLUGIN_TESTRESULTLOGPLUGIN=true
2019-12-01T13:20:05.8702599Z AGENT_HOMEDIRECTORY=C:\agents\2.160.1
2019-12-01T13:20:05.8702779Z AGENT_ID=518
---
2019-12-01T13:20:05.8712914Z BUILD_SOURCEBRANCHNAME=auto
2019-12-01T13:20:05.8712984Z BUILD_SOURCESDIRECTORY=D:\a\1\s
2019-12-01T13:20:05.8713085Z BUILD_SOURCEVERSION=f9752418e2c9ba84181172c2c50e7b699333a60d
2019-12-01T13:20:05.8713162Z BUILD_SOURCEVERSIONAUTHOR=bors
2019-12-01T13:20:05.8713270Z BUILD_SOURCEVERSIONMESSAGE=Auto merge of #66924 - Centril:rollup-r7pljxh, r=Centril
2019-12-01T13:20:05.8713448Z CI_JOB_NAME=i686-msvc-1
2019-12-01T13:20:05.8713535Z COBERTURA_HOME=C:\cobertura-2.1.1
2019-12-01T13:20:05.8713614Z COMMONPROGRAMFILES=C:\Program Files\Common Files
2019-12-01T13:20:05.8713720Z COMMON_TESTRESULTSDIRECTORY=D:\a\1\TestResults
---
2019-12-01T13:21:45.8377912Z Chocolatey installed 0/1 packages. 1 packages failed.
2019-12-01T13:21:45.8378337Z  See the log for details (C:\ProgramData\chocolatey\logs\chocolatey.log).
2019-12-01T13:21:45.8383257Z 
2019-12-01T13:21:45.8388679Z Failures
2019-12-01T13:21:45.8399500Z  - msys2 (exited 1) - msys2 not installed. An error occurred during installation:
2019-12-01T13:21:45.8400049Z  The remote server returned an error: (503) Server Unavailable. Service Unavailable
2019-12-01T13:21:46.4031127Z 
2019-12-01T13:21:46.4131339Z ##[error]Bash exited with code '1'.
2019-12-01T13:21:46.4296733Z ##[section]Starting: Checkout
2019-12-01T13:21:46.4409725Z ==============================================================================
2019-12-01T13:21:46.4409854Z Task         : Get sources
2019-12-01T13:21:46.4409954Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
