plain
2019-11-26T16:58:56.9754792Z do so (now or later) by using -b with the checkout command again. Example:
2019-11-26T16:58:56.9754852Z 
2019-11-26T16:58:56.9754988Z   git checkout -b <new-branch-name>
2019-11-26T16:58:56.9755058Z 
2019-11-26T16:58:56.9755524Z HEAD is now at a256e1fec Auto merge of #66783 - Mark-Simulacrum:par, r=<try>
2019-11-26T16:58:57.0148594Z ##[section]Starting: Setup environment
2019-11-26T16:58:57.0257882Z ==============================================================================
2019-11-26T16:58:57.0257994Z Task         : Bash
2019-11-26T16:58:57.0258098Z Description  : Run a Bash script on macOS, Linux, or Windows
---
2019-11-26T16:58:58.7592610Z BUILD_SOURCEBRANCHNAME=try
2019-11-26T16:58:58.7592680Z BUILD_SOURCESDIRECTORY=D:\a\1\s
2019-11-26T16:58:58.7592780Z BUILD_SOURCEVERSION=a256e1fecd38642f9bfb07386e3248934f811555
2019-11-26T16:58:58.7592859Z BUILD_SOURCEVERSIONAUTHOR=bors
2019-11-26T16:58:58.7592972Z BUILD_SOURCEVERSIONMESSAGE=Auto merge of #66783 - Mark-Simulacrum:par, r=<try>
2019-11-26T16:58:58.7593143Z CI_JOB_NAME=dist-x86_64-msvc
2019-11-26T16:58:58.7593228Z COBERTURA_HOME=C:\cobertura-2.1.1
2019-11-26T16:58:58.7593308Z COMMONPROGRAMFILES=C:\Program Files\Common Files
2019-11-26T16:58:58.7593407Z COMMON_TESTRESULTSDIRECTORY=D:\a\1\TestResults
---
2019-11-26T16:58:58.7615114Z VSTS_AGENT_PERFLOG=c:\vsts\perflog
2019-11-26T16:58:58.7616316Z VSTS_PROCESS_LOOKUP_ID=vsts_83f37cdf-5cb5-4473-b58c-a66184e646cf
2019-11-26T16:58:58.7616490Z WINDIR=C:\windows
2019-11-26T16:58:58.7616580Z WIX=C:\Program Files (x86)\WiX Toolset v3.11\
2019-11-26T16:58:58.7616665Z [do not merge] posix semaphores over make jobserver
2019-11-26T16:58:58.7616822Z agent.jobstatus=Succeeded
2019-11-26T16:58:58.7617007Z r? @ghost
2019-11-26T16:58:58.7617115Z 
2019-11-26T16:58:58.7617253Z disk usage:
---
2019-11-26T17:05:51.1038494Z Submodule path 'src/doc/nomicon': checked out '041c46e692a2592853aeca132c8dfe8eb5a79a9e'
2019-11-26T17:05:51.7845322Z Submodule path 'src/doc/reference': checked out '9e843aeb4df083522c7277179bbaa25d0507731c'
2019-11-26T17:05:52.5307892Z Submodule path 'src/doc/rustc-guide': checked out '934380b7cfceaaa4e1b9bb0de4a372f32725520b'
2019-11-26T17:05:53.3712966Z Submodule path 'src/stdarch': checked out 'e0ab2c165ace03a61139b61f1d9b86b07028850f'
2019-11-26T17:05:54.6870644Z error: Server does not allow request for unadvertised object c8373fbde42777e389c889e8b16a1d5f823e7a68
2019-11-26T17:05:54.8061331Z Fetched in submodule path 'src/tools/cargo', but it did not contain c8373fbde42777e389c889e8b16a1d5f823e7a68. Direct fetching of that commit failed.
2019-11-26T17:05:54.8174491Z + sleep 1
2019-11-26T17:05:55.8365543Z + (( n++ ))
2019-11-26T17:05:55.8366445Z + echo 'Command failed. Attempt 2/5:'
2019-11-26T17:05:55.8366727Z + true
---
2019-11-26T17:06:02.0078525Z Submodule path 'src/doc/nomicon': checked out '041c46e692a2592853aeca132c8dfe8eb5a79a9e'
2019-11-26T17:06:02.7152502Z Submodule path 'src/doc/reference': checked out '9e843aeb4df083522c7277179bbaa25d0507731c'
2019-11-26T17:06:03.5720748Z Submodule path 'src/doc/rustc-guide': checked out '934380b7cfceaaa4e1b9bb0de4a372f32725520b'
2019-11-26T17:06:05.0644745Z Submodule path 'src/stdarch': checked out 'e0ab2c165ace03a61139b61f1d9b86b07028850f'
2019-11-26T17:06:06.4214027Z error: Server does not allow request for unadvertised object c8373fbde42777e389c889e8b16a1d5f823e7a68
2019-11-26T17:06:06.5455346Z Fetched in submodule path 'src/tools/cargo', but it did not contain c8373fbde42777e389c889e8b16a1d5f823e7a68. Direct fetching of that commit failed.
2019-11-26T17:06:06.5664444Z + sleep 2
2019-11-26T17:06:08.5788215Z + (( n++ ))
2019-11-26T17:06:08.5788986Z + echo 'Command failed. Attempt 3/5:'
2019-11-26T17:06:08.5789224Z Command failed. Attempt 3/5:
---
2019-11-26T17:06:14.0580972Z Submodule path 'src/doc/nomicon': checked out '041c46e692a2592853aeca132c8dfe8eb5a79a9e'
2019-11-26T17:06:14.7500827Z Submodule path 'src/doc/reference': checked out '9e843aeb4df083522c7277179bbaa25d0507731c'
2019-11-26T17:06:15.4025762Z Submodule path 'src/doc/rustc-guide': checked out '934380b7cfceaaa4e1b9bb0de4a372f32725520b'
2019-11-26T17:06:16.2208050Z Submodule path 'src/stdarch': checked out 'e0ab2c165ace03a61139b61f1d9b86b07028850f'
2019-11-26T17:06:18.0890993Z error: Server does not allow request for unadvertised object c8373fbde42777e389c889e8b16a1d5f823e7a68
2019-11-26T17:06:18.2136930Z Fetched in submodule path 'src/tools/cargo', but it did not contain c8373fbde42777e389c889e8b16a1d5f823e7a68. Direct fetching of that commit failed.
2019-11-26T17:06:18.2233127Z + sleep 3
2019-11-26T17:06:21.2730401Z + (( n++ ))
2019-11-26T17:06:21.2730568Z + echo 'Command failed. Attempt 4/5:'
2019-11-26T17:06:21.2731400Z Command failed. Attempt 4/5:
---
2019-11-26T17:06:27.8325971Z Submodule path 'src/doc/reference': checked out '9e843aeb4df083522c7277179bbaa25d0507731c'
2019-11-26T17:06:28.4903402Z Submodule path 'src/doc/rustc-guide': checked out '934380b7cfceaaa4e1b9bb0de4a372f32725520b'
2019-11-26T17:06:29.1046934Z tar: Exiting with failure status due to previous errors
2019-11-26T17:06:29.7077369Z Submodule path 'src/stdarch': checked out 'e0ab2c165ace03a61139b61f1d9b86b07028850f'
2019-11-26T17:06:30.8440225Z error: Server does not allow request for unadvertised object c8373fbde42777e389c889e8b16a1d5f823e7a68
2019-11-26T17:06:30.9560492Z Fetched in submodule path 'src/tools/cargo', but it did not contain c8373fbde42777e389c889e8b16a1d5f823e7a68. Direct fetching of that commit failed.
2019-11-26T17:06:30.9658215Z + sleep 4
2019-11-26T17:06:34.9845896Z + (( n++ ))
2019-11-26T17:06:34.9846760Z + echo 'Command failed. Attempt 5/5:'
2019-11-26T17:06:34.9847148Z Command failed. Attempt 5/5:
---
2019-11-26T17:06:38.7744833Z Submodule path 'src/doc/nomicon': checked out '041c46e692a2592853aeca132c8dfe8eb5a79a9e'
2019-11-26T17:06:39.3794314Z Submodule path 'src/doc/reference': checked out '9e843aeb4df083522c7277179bbaa25d0507731c'
2019-11-26T17:06:39.9540965Z Submodule path 'src/doc/rustc-guide': checked out '934380b7cfceaaa4e1b9bb0de4a372f32725520b'
2019-11-26T17:06:40.6391122Z Submodule path 'src/stdarch': checked out 'e0ab2c165ace03a61139b61f1d9b86b07028850f'
2019-11-26T17:06:41.7570510Z error: Server does not allow request for unadvertised object c8373fbde42777e389c889e8b16a1d5f823e7a68
2019-11-26T17:06:41.8705259Z Fetched in submodule path 'src/tools/cargo', but it did not contain c8373fbde42777e389c889e8b16a1d5f823e7a68. Direct fetching of that commit failed.
2019-11-26T17:06:41.8805249Z The command has failed after 5 attempts.
2019-11-26T17:06:41.8805249Z The command has failed after 5 attempts.
2019-11-26T17:06:41.8805352Z + echo 'The command has failed after 5 attempts.'
2019-11-26T17:06:41.8815456Z 
2019-11-26T17:06:41.8815456Z 
2019-11-26T17:06:41.8934656Z ##[error]Bash exited with code '1'.
2019-11-26T17:06:41.9005302Z ##[section]Starting: Checkout
2019-11-26T17:06:41.9119452Z ==============================================================================
2019-11-26T17:06:41.9119565Z Task         : Get sources
2019-11-26T17:06:41.9119656Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
