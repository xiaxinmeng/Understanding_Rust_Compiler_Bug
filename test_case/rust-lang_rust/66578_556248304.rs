plain
2019-11-20T18:50:28.4756369Z do so (now or later) by using -b with the checkout command again. Example:
2019-11-20T18:50:28.4756699Z 
2019-11-20T18:50:28.4757339Z   git checkout -b <new-branch-name>
2019-11-20T18:50:28.4757547Z 
2019-11-20T18:50:28.4757815Z HEAD is now at 762a091ea Auto merge of #66578 - Centril:rollup-pgz1v7t, r=Centril
2019-11-20T18:50:28.5150305Z ##[section]Starting: Decide whether to run this job
2019-11-20T18:50:28.5263335Z ==============================================================================
2019-11-20T18:50:28.5263438Z Task         : Bash
2019-11-20T18:50:28.5263535Z Description  : Run a Bash script on macOS, Linux, or Windows
---
2019-11-20T18:50:29.9202002Z 
2019-11-20T18:50:29.9202327Z 
2019-11-20T18:50:29.9202642Z 
2019-11-20T18:50:29.9203084Z 
2019-11-20T18:50:29.9203460Z  - #66060 (Making ICEs and test them in incremental)
2019-11-20T18:50:29.9204700Z  - #66298 (rustdoc: fixes #64305: disable search field instead of hidding it)
2019-11-20T18:50:29.9205024Z  - #66457 (Just derive Hashstable in librustc)
2019-11-20T18:50:29.9205674Z  - #66496 (rustc_metadata: Privatize more things)
2019-11-20T18:50:29.9205890Z  - #66514 (Fix selected crate search filter)
2019-11-20T18:50:29.9206094Z  - #66535 (Avoid ICE when `break`ing to an unreachable label)
2019-11-20T18:50:29.9206319Z  - #66573 (Ignore run-make reproducible-build-2 on Mac)
2019-11-20T18:50:29.9208069Z AGENT_DISABLELOGPLUGIN_TESTFILEPUBLISHERPLUGIN=true
2019-11-20T18:50:29.9208808Z AGENT_DISABLELOGPLUGIN_TESTRESULTLOGPLUGIN=true
2019-11-20T18:50:29.9212604Z AGENT_HOMEDIRECTORY=C:\agents\2.160.1
2019-11-20T18:50:29.9212899Z AGENT_ID=523
---
2019-11-20T18:50:29.9222225Z BUILD_SOURCEBRANCHNAME=auto
2019-11-20T18:50:29.9222297Z BUILD_SOURCESDIRECTORY=D:\a\1\s
2019-11-20T18:50:29.9222403Z BUILD_SOURCEVERSION=762a091ea98c2456cfc2c4356a2461585eb4c51b
2019-11-20T18:50:29.9222624Z BUILD_SOURCEVERSIONAUTHOR=bors
2019-11-20T18:50:29.9222744Z BUILD_SOURCEVERSIONMESSAGE=Auto merge of #66578 - Centril:rollup-pgz1v7t, r=Centril
2019-11-20T18:50:29.9222926Z CI_JOB_NAME=x86_64-mingw-2
2019-11-20T18:50:29.9223015Z COBERTURA_HOME=C:\cobertura-2.1.1
2019-11-20T18:50:29.9223097Z COMMONPROGRAMFILES=C:\Program Files\Common Files
2019-11-20T18:50:29.9223208Z COMMON_TESTRESULTSDIRECTORY=D:\a\1\TestResults
---
2019-11-20T18:56:13.3500377Z  10 43.6M   10 4538k    0     0  47077      0  0:16:11  0:01:38  0:14:33     0
2019-11-20T18:56:13.3643111Z  10 43.6M   10 4538k    0     0  47075      0  0:16:11  0:01:38  0:14:33     0
2019-11-20T18:56:13.3645772Z curl: (18) transfer closed with 41088713 bytes remaining to read
2019-11-20T18:56:13.3695863Z 
2019-11-20T18:56:13.3871939Z ##[error]Bash exited with code '18'.
2019-11-20T18:56:13.4032999Z ##[section]Starting: Checkout
2019-11-20T18:56:13.4150767Z ==============================================================================
2019-11-20T18:56:13.4150917Z Task         : Get sources
2019-11-20T18:56:13.4151011Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
