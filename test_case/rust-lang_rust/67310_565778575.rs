plain
2019-12-15T05:02:03.4301823Z do so (now or later) by using -b with the checkout command again. Example:
2019-12-15T05:02:03.4303875Z 
2019-12-15T05:02:03.4305629Z   git checkout -b <new-branch-name>
2019-12-15T05:02:03.4468669Z 
2019-12-15T05:02:03.4469766Z HEAD is now at b194600eb Auto merge of #67310 - Centril:rollup-22jiyow, r=Centril
2019-12-15T05:02:03.4692683Z ##[section]Starting: Setup environment
2019-12-15T05:02:03.4799856Z ==============================================================================
2019-12-15T05:02:03.4799966Z Task         : Bash
2019-12-15T05:02:03.4800061Z Description  : Run a Bash script on macOS, Linux, or Windows
---
2019-12-15T05:02:05.1283076Z BUILD_SOURCEBRANCHNAME=auto
2019-12-15T05:02:05.1283140Z BUILD_SOURCESDIRECTORY=D:\a\1\s
2019-12-15T05:02:05.1283473Z BUILD_SOURCEVERSION=b194600eb3879f4ddbf0c3c33b1a6df8ff0dca96
2019-12-15T05:02:05.1283551Z BUILD_SOURCEVERSIONAUTHOR=bors
2019-12-15T05:02:05.1283843Z BUILD_SOURCEVERSIONMESSAGE=Auto merge of #67310 - Centril:rollup-22jiyow, r=Centril
2019-12-15T05:02:05.1284028Z CI_JOB_NAME=i686-msvc-1
2019-12-15T05:02:05.1284120Z COBERTURA_HOME=C:\cobertura-2.1.1
2019-12-15T05:02:05.1284205Z COMMONPROGRAMFILES=C:\Program Files\Common Files
2019-12-15T05:02:05.1284311Z COMMON_TESTRESULTSDIRECTORY=D:\a\1\TestResults
---
2019-12-15T05:03:55.4541043Z Chocolatey installed 0/1 packages. 1 packages failed.
2019-12-15T05:03:55.4541126Z  See the log for details (C:\ProgramData\chocolatey\logs\chocolatey.log).
2019-12-15T05:03:55.4545346Z 
2019-12-15T05:03:55.4550286Z Failures
2019-12-15T05:03:55.4556957Z  - msys2 (exited 1) - msys2 not installed. An error occurred during installation:
2019-12-15T05:03:55.4557126Z  The remote server returned an error: (503) Server Unavailable. Service Unavailable
2019-12-15T05:03:55.4570377Z Enjoy using Chocolatey? Explore more amazing features to take your
2019-12-15T05:03:55.4570450Z experience to the next level at
2019-12-15T05:03:55.4570546Z  https://chocolatey.org/compare
2019-12-15T05:03:56.0280040Z 
2019-12-15T05:03:56.0280040Z 
2019-12-15T05:03:56.0365214Z ##[error]Bash exited with code '1'.
2019-12-15T05:03:56.0492205Z ##[section]Starting: Checkout
2019-12-15T05:03:56.0600195Z ==============================================================================
2019-12-15T05:03:56.0600290Z Task         : Get sources
2019-12-15T05:03:56.0600361Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
