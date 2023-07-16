plain
2019-11-30T05:48:13.9327797Z do so (now or later) by using -b with the checkout command again. Example:
2019-11-30T05:48:13.9327879Z 
2019-11-30T05:48:13.9327942Z   git checkout -b <new-branch-name>
2019-11-30T05:48:13.9327989Z 
2019-11-30T05:48:13.9328291Z HEAD is now at 8c459b53a Auto merge of #66887 - dtolnay:rollup-uxowp8d, r=dtolnay
2019-11-30T05:48:13.9702972Z ##[section]Starting: Setup environment
2019-11-30T05:48:13.9815106Z ==============================================================================
2019-11-30T05:48:13.9815209Z Task         : Bash
2019-11-30T05:48:13.9815309Z Description  : Run a Bash script on macOS, Linux, or Windows
---
2019-11-30T05:48:16.5252283Z 
2019-11-30T05:48:16.5252353Z  - #66818 (Format libstd/os with rustfmt)
2019-11-30T05:48:16.5252421Z  - #66819 (Format libstd/sys with rustfmt)
2019-11-30T05:48:16.5252501Z  - #66820 (Format libstd with rustfmt)
2019-11-30T05:48:16.5252573Z  - #66847 (Allow any identifier as format arg name)
2019-11-30T05:48:16.5252743Z AGENT_DISABLELOGPLUGIN_TESTFILEPUBLISHERPLUGIN=true
2019-11-30T05:48:16.5252816Z AGENT_DISABLELOGPLUGIN_TESTRESULTLOGPLUGIN=true
2019-11-30T05:48:16.5252904Z AGENT_HOMEDIRECTORY=C:\agents\2.160.1
2019-11-30T05:48:16.5252967Z AGENT_ID=518
---
2019-11-30T05:48:16.5259094Z BUILD_SOURCEBRANCHNAME=auto
2019-11-30T05:48:16.5259156Z BUILD_SOURCESDIRECTORY=D:\a\1\s
2019-11-30T05:48:16.5259244Z BUILD_SOURCEVERSION=8c459b53abfdc8293b23be100d61d412dfdb30a4
2019-11-30T05:48:16.5259329Z BUILD_SOURCEVERSIONAUTHOR=bors
2019-11-30T05:48:16.5259410Z BUILD_SOURCEVERSIONMESSAGE=Auto merge of #66887 - dtolnay:rollup-uxowp8d, r=dtolnay
2019-11-30T05:48:16.5259570Z CI_JOB_NAME=x86_64-mingw-1
2019-11-30T05:48:16.5259647Z COBERTURA_HOME=C:\cobertura-2.1.1
2019-11-30T05:48:16.5259717Z COMMONPROGRAMFILES=C:\Program Files\Common Files
2019-11-30T05:48:16.5259803Z COMMON_TESTRESULTSDIRECTORY=D:\a\1\TestResults
---
2019-11-30T05:49:15.7997493Z Chocolatey installed 0/1 packages. 1 packages failed.
2019-11-30T05:49:15.7997748Z  See the log for details (C:\ProgramData\chocolatey\logs\chocolatey.log).
2019-11-30T05:49:15.8003023Z 
2019-11-30T05:49:15.8007297Z Failures
2019-11-30T05:49:15.8017026Z  - msys2 (exited 1) - msys2 not installed. An error occurred during installation:
2019-11-30T05:49:15.8017209Z  The remote server returned an error: (503) Server Unavailable. Service Unavailable
2019-11-30T05:49:16.3318509Z 
2019-11-30T05:49:16.3426661Z ##[error]Bash exited with code '1'.
2019-11-30T05:49:16.3594145Z ##[section]Starting: Checkout
2019-11-30T05:49:16.3697991Z ==============================================================================
2019-11-30T05:49:16.3698103Z Task         : Get sources
2019-11-30T05:49:16.3698204Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
