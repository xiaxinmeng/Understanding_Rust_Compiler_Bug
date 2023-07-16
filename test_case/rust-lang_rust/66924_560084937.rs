plain
2019-12-01T09:50:06.8137208Z do so (now or later) by using -b with the checkout command again. Example:
2019-12-01T09:50:06.8137402Z 
2019-12-01T09:50:06.8137558Z   git checkout -b <new-branch-name>
2019-12-01T09:50:06.8137713Z 
2019-12-01T09:50:06.8138137Z HEAD is now at 07c6d2dc8 Auto merge of #66924 - Centril:rollup-r7pljxh, r=Centril
2019-12-01T09:50:06.8699823Z ##[section]Starting: Setup environment
2019-12-01T09:50:06.8813743Z ==============================================================================
2019-12-01T09:50:06.8813847Z Task         : Bash
2019-12-01T09:50:06.8813947Z Description  : Run a Bash script on macOS, Linux, or Windows
---
2019-12-01T09:50:08.6227213Z 
2019-12-01T09:50:08.6227248Z 
2019-12-01T09:50:08.6227385Z 
2019-12-01T09:50:08.6227425Z 
2019-12-01T09:50:08.6227562Z  - #66346 (Replace .unwrap() with ? in std::os::unix::net)
2019-12-01T09:50:08.6227674Z  - #66789 (rustc: move mir::SourceScopeLocalData to a field of SourceScopeData.)
2019-12-01T09:50:08.6227833Z  - #66822 (libunwind_panic: adjust miri panic hack)
2019-12-01T09:50:08.6227932Z  - #66827 (handle diverging functions forwarding their return place)
2019-12-01T09:50:08.6228383Z  - #66828 (Less minification)
2019-12-01T09:50:08.6228502Z  - #66850 (rustc: hide HirId's fmt::Debug output from -Z span_free_formats.)
2019-12-01T09:50:08.6228623Z  - #66874 (Miri engine: proper support for `Assert` MIR terminators)
2019-12-01T09:50:08.6229015Z  - #66907 (rustc: don't just show raw DefIndex's in BrNamed's fmt::Debug impl.)
2019-12-01T09:50:08.6229677Z AGENT_DISABLELOGPLUGIN_TESTFILEPUBLISHERPLUGIN=true
2019-12-01T09:50:08.6235359Z AGENT_DISABLELOGPLUGIN_TESTRESULTLOGPLUGIN=true
2019-12-01T09:50:08.6235479Z AGENT_HOMEDIRECTORY=C:\agents\2.160.1
2019-12-01T09:50:08.6235593Z AGENT_ID=532
---
2019-12-01T09:50:08.6242284Z BUILD_SOURCEBRANCHNAME=auto
2019-12-01T09:50:08.6242381Z BUILD_SOURCESDIRECTORY=D:\a\2\s
2019-12-01T09:50:08.6242623Z BUILD_SOURCEVERSION=07c6d2dc8ad0f04b9a86ce2d49484ad793047851
2019-12-01T09:50:08.6242719Z BUILD_SOURCEVERSIONAUTHOR=bors
2019-12-01T09:50:08.6242983Z BUILD_SOURCEVERSIONMESSAGE=Auto merge of #66924 - Centril:rollup-r7pljxh, r=Centril
2019-12-01T09:50:08.6243254Z CI_JOB_NAME=dist-x86_64-msvc
2019-12-01T09:50:08.6243348Z COBERTURA_HOME=C:\cobertura-2.1.1
2019-12-01T09:50:08.6243446Z COMMONPROGRAMFILES=C:\Program Files\Common Files
2019-12-01T09:50:08.6243531Z COMMON_TESTRESULTSDIRECTORY=D:\a\2\TestResults
---
2019-12-01T09:51:36.3430249Z Chocolatey installed 0/1 packages. 1 packages failed.
2019-12-01T09:51:36.3430473Z  See the log for details (C:\ProgramData\chocolatey\logs\chocolatey.log).
2019-12-01T09:51:36.3434642Z 
2019-12-01T09:51:36.3440503Z Failures
2019-12-01T09:51:36.3447874Z  - msys2 (exited 1) - msys2 not installed. An error occurred during installation:
2019-12-01T09:51:36.3448222Z  The remote server returned an error: (503) Server Unavailable. Service Unavailable
2019-12-01T09:51:36.9012979Z 
2019-12-01T09:51:36.9098251Z ##[error]Bash exited with code '1'.
2019-12-01T09:51:36.9248104Z ##[section]Starting: Checkout
2019-12-01T09:51:36.9356066Z ==============================================================================
2019-12-01T09:51:36.9356300Z Task         : Get sources
2019-12-01T09:51:36.9356380Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
