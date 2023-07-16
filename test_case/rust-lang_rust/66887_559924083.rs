plain
2019-11-30T08:03:27.6489943Z do so (now or later) by using -b with the checkout command again. Example:
2019-11-30T08:03:27.6490145Z 
2019-11-30T08:03:27.6490421Z   git checkout -b <new-branch-name>
2019-11-30T08:03:27.6490591Z 
2019-11-30T08:03:27.6490759Z HEAD is now at ace910a9a Auto merge of #66887 - dtolnay:rollup-uxowp8d, r=Centril
2019-11-30T08:03:27.6887675Z ##[section]Starting: Setup environment
2019-11-30T08:03:27.6996919Z ==============================================================================
2019-11-30T08:03:27.6997136Z Task         : Bash
2019-11-30T08:03:27.6997209Z Description  : Run a Bash script on macOS, Linux, or Windows
---
2019-11-30T08:03:29.3386359Z 
2019-11-30T08:03:29.3386501Z  - #66818 (Format libstd/os with rustfmt)
2019-11-30T08:03:29.3386675Z  - #66819 (Format libstd/sys with rustfmt)
2019-11-30T08:03:29.3386845Z  - #66820 (Format libstd with rustfmt)
2019-11-30T08:03:29.3389507Z  - #66847 (Allow any identifier as format arg name)
2019-11-30T08:03:29.3394410Z AGENT_DISABLELOGPLUGIN_TESTFILEPUBLISHERPLUGIN=true
2019-11-30T08:03:29.3394698Z AGENT_DISABLELOGPLUGIN_TESTRESULTLOGPLUGIN=true
2019-11-30T08:03:29.3394931Z AGENT_HOMEDIRECTORY=C:\agents\2.160.1
2019-11-30T08:03:29.3395126Z AGENT_ID=512
---
2019-11-30T08:03:29.3405360Z BUILD_SOURCEBRANCHNAME=auto
2019-11-30T08:03:29.3405428Z BUILD_SOURCESDIRECTORY=D:\a\1\s
2019-11-30T08:03:29.3405529Z BUILD_SOURCEVERSION=ace910a9a193c738bb4d93c152d66b3e1d8b824a
2019-11-30T08:03:29.3405604Z BUILD_SOURCEVERSIONAUTHOR=bors
2019-11-30T08:03:29.3405709Z BUILD_SOURCEVERSIONMESSAGE=Auto merge of #66887 - dtolnay:rollup-uxowp8d, r=Centril
2019-11-30T08:03:29.3405888Z CI_JOB_NAME=i686-msvc-1
2019-11-30T08:03:29.3405980Z COBERTURA_HOME=C:\cobertura-2.1.1
2019-11-30T08:03:29.3406058Z COMMONPROGRAMFILES=C:\Program Files\Common Files
2019-11-30T08:03:29.3406154Z COMMON_TESTRESULTSDIRECTORY=D:\a\1\TestResults
---
2019-11-30T08:04:59.0780154Z Chocolatey installed 0/1 packages. 1 packages failed.
2019-11-30T08:04:59.0780674Z  See the log for details (C:\ProgramData\chocolatey\logs\chocolatey.log).
2019-11-30T08:04:59.0785246Z 
2019-11-30T08:04:59.0790872Z Failures
2019-11-30T08:04:59.0799005Z  - msys2 (exited 1) - msys2 not installed. An error occurred during installation:
2019-11-30T08:04:59.0799496Z  The remote server returned an error: (503) Server Unavailable. Service Unavailable
2019-11-30T08:04:59.6123008Z 
2019-11-30T08:04:59.6202103Z ##[error]Bash exited with code '1'.
2019-11-30T08:04:59.6351351Z ##[section]Starting: Checkout
2019-11-30T08:04:59.6459599Z ==============================================================================
2019-11-30T08:04:59.6459688Z Task         : Get sources
2019-11-30T08:04:59.6459787Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
