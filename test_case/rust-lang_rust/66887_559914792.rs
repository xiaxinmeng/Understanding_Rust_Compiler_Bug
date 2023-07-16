plain
2019-11-30T05:40:15.0191094Z do so (now or later) by using -b with the checkout command again. Example:
2019-11-30T05:40:15.0191140Z 
2019-11-30T05:40:15.0191189Z   git checkout -b <new-branch-name>
2019-11-30T05:40:15.0191245Z 
2019-11-30T05:40:15.0191311Z HEAD is now at 500f8e2aa Auto merge of #66887 - dtolnay:rollup-uxowp8d, r=dtolnay
2019-11-30T05:40:15.0548925Z ##[section]Starting: Setup environment
2019-11-30T05:40:15.0652459Z ==============================================================================
2019-11-30T05:40:15.0652733Z Task         : Bash
2019-11-30T05:40:15.0652811Z Description  : Run a Bash script on macOS, Linux, or Windows
---
2019-11-30T05:40:16.7234637Z 
2019-11-30T05:40:16.7234825Z  - #66818 (Format libstd/os with rustfmt)
2019-11-30T05:40:16.7235072Z  - #66819 (Format libstd/sys with rustfmt)
2019-11-30T05:40:16.7235297Z  - #66820 (Format libstd with rustfmt)
2019-11-30T05:40:16.7235469Z  - #66847 (Allow any identifier as format arg name)
2019-11-30T05:40:16.7241845Z AGENT_DISABLELOGPLUGIN_TESTFILEPUBLISHERPLUGIN=true
2019-11-30T05:40:16.7242037Z AGENT_DISABLELOGPLUGIN_TESTRESULTLOGPLUGIN=true
2019-11-30T05:40:16.7242441Z AGENT_HOMEDIRECTORY=C:\agents\2.160.1
2019-11-30T05:40:16.7242547Z AGENT_ID=515
---
2019-11-30T05:40:16.7254301Z BUILD_SOURCEBRANCHNAME=auto
2019-11-30T05:40:16.7254364Z BUILD_SOURCESDIRECTORY=D:\a\1\s
2019-11-30T05:40:16.7254759Z BUILD_SOURCEVERSION=500f8e2aa7a354d8a01b731642fc92fea5bda8cb
2019-11-30T05:40:16.7254828Z BUILD_SOURCEVERSIONAUTHOR=bors
2019-11-30T05:40:16.7254926Z BUILD_SOURCEVERSIONMESSAGE=Auto merge of #66887 - dtolnay:rollup-uxowp8d, r=dtolnay
2019-11-30T05:40:16.7255091Z CI_JOB_NAME=i686-msvc-1
2019-11-30T05:40:16.7255151Z COBERTURA_HOME=C:\cobertura-2.1.1
2019-11-30T05:40:16.7255238Z COMMONPROGRAMFILES=C:\Program Files\Common Files
2019-11-30T05:40:16.7255385Z COMMON_TESTRESULTSDIRECTORY=D:\a\1\TestResults
---
2019-11-30T05:41:55.0572079Z Chocolatey installed 0/1 packages. 1 packages failed.
2019-11-30T05:41:55.0572571Z  See the log for details (C:\ProgramData\chocolatey\logs\chocolatey.log).
2019-11-30T05:41:55.0576096Z 
2019-11-30T05:41:55.0582431Z Failures
2019-11-30T05:41:55.0588950Z  - msys2 (exited 1) - msys2 not installed. An error occurred during installation:
2019-11-30T05:41:55.0589538Z  The remote server returned an error: (503) Server Unavailable. Service Unavailable
2019-11-30T05:41:55.0604955Z Did you know the proceeds of Pro (and some proceeds from other
2019-11-30T05:41:55.0605889Z  licensed editions) go into bettering the community infrastructure?
2019-11-30T05:41:55.0606390Z  Your support ensures an active community, keeps Chocolatey tip top,
2019-11-30T05:41:55.0606875Z  plus it nets you some awesome features!
2019-11-30T05:41:55.0606875Z  plus it nets you some awesome features!
2019-11-30T05:41:55.0607311Z  https://chocolatey.org/compare
2019-11-30T05:41:55.5027425Z 
2019-11-30T05:41:55.5112188Z ##[error]Bash exited with code '1'.
2019-11-30T05:41:55.5276237Z ##[section]Starting: Checkout
2019-11-30T05:41:55.5386635Z ==============================================================================
2019-11-30T05:41:55.5386755Z Task         : Get sources
2019-11-30T05:41:55.5386842Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
