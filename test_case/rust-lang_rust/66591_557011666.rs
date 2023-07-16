plain
2019-11-21T10:02:40.8705495Z do so (now or later) by using -b with the checkout command again. Example:
2019-11-21T10:02:40.8706530Z 
2019-11-21T10:02:40.8707473Z   git checkout -b <new-branch-name>
2019-11-21T10:02:40.8708349Z 
2019-11-21T10:02:40.8708663Z HEAD is now at 00d771443 Auto merge of #66591 - Centril:rollup-z7iww0q, r=Centril
2019-11-21T10:02:40.9076494Z ##[section]Starting: Setup environment
2019-11-21T10:02:40.9182228Z ==============================================================================
2019-11-21T10:02:40.9182326Z Task         : Bash
2019-11-21T10:02:40.9182390Z Description  : Run a Bash script on macOS, Linux, or Windows
---
2019-11-21T10:02:42.5995574Z 
2019-11-21T10:02:42.5995699Z 
2019-11-21T10:02:42.5995831Z  - #65730 (Suggest to add lifetime constraint at explicit ouput of functions)
2019-11-21T10:02:42.5995947Z  - #66460 (Add a proc-macro to derive HashStable in librustc dependencies)
2019-11-21T10:02:42.5996084Z  - #66468 (Cleanup Miri SIMD intrinsics)
2019-11-21T10:02:42.5996178Z  - #66520 (Disable gdb pretty printer global section on wasm targets)
2019-11-21T10:02:42.5996295Z  - #66524 (Support multiple revisions in `compiletest`)
2019-11-21T10:02:42.5996512Z  - #66569 (GitHub Actions: preparations, part 1)
2019-11-21T10:02:42.5996608Z AGENT_BUILDDIRECTORY=D:\a\1
2019-11-21T10:02:42.5996693Z AGENT_DISABLELOGPLUGIN_TESTFILEPUBLISHERPLUGIN=true
2019-11-21T10:02:42.5996800Z AGENT_DISABLELOGPLUGIN_TESTRESULTLOGPLUGIN=true
---
2019-11-21T10:02:42.6005371Z BUILD_SOURCEBRANCHNAME=auto
2019-11-21T10:02:42.6005467Z BUILD_SOURCESDIRECTORY=D:\a\1\s
2019-11-21T10:02:42.6005573Z BUILD_SOURCEVERSION=00d771443272e353b1ea4d7d172209a6ca3783f6
2019-11-21T10:02:42.6005653Z BUILD_SOURCEVERSIONAUTHOR=bors
2019-11-21T10:02:42.6005775Z BUILD_SOURCEVERSIONMESSAGE=Auto merge of #66591 - Centril:rollup-z7iww0q, r=Centril
2019-11-21T10:02:42.6005953Z CI_JOB_NAME=x86_64-msvc-2
2019-11-21T10:02:42.6006027Z COBERTURA_HOME=C:\cobertura-2.1.1
2019-11-21T10:02:42.6006133Z COMMONPROGRAMFILES=C:\Program Files\Common Files
2019-11-21T10:02:42.6006235Z COMMON_TESTRESULTSDIRECTORY=D:\a\1\TestResults
---
2019-11-21T10:02:57.7126102Z  29  480M   29  141M    0     0  14.6M      0  0:00:32  0:00:09  0:00:23 17.3M
2019-11-21T10:02:59.3072950Z  33  480M   33  163M    0     0  15.2M      0  0:00:31  0:00:10  0:00:21 18.4M
2019-11-21T10:02:59.4504273Z  38  480M   38  186M    0     0  15.2M      0  0:00:31  0:00:12  0:00:19 17.4M
2019-11-21T10:02:59.4541839Z  39  480M   39  190M    0     0  15.3M      0  0:00:31  0:00:12  0:00:19 17.3M
2019-11-21T10:02:59.4542009Z curl: (56) OpenSSL SSL_read: SSL_ERROR_SYSCALL, errno 10054
2019-11-21T10:02:59.4560372Z 
2019-11-21T10:02:59.4560870Z gzip: stdin: unexpected end of file
2019-11-21T10:02:59.4561736Z tar: Unexpected EOF in archive
2019-11-21T10:02:59.4563699Z tar: Unexpected EOF in archive
2019-11-21T10:02:59.4563958Z tar: Error is not recoverable: exiting now
2019-11-21T10:02:59.4626975Z 
2019-11-21T10:02:59.4707199Z ##[error]Bash exited with code '2'.
2019-11-21T10:02:59.4893072Z ##[section]Starting: Checkout
2019-11-21T10:02:59.5009035Z ==============================================================================
2019-11-21T10:02:59.5009291Z Task         : Get sources
2019-11-21T10:02:59.5009548Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
