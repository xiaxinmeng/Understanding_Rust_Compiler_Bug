plain
2019-12-15T06:26:46.3367747Z do so (now or later) by using -b with the checkout command again. Example:
2019-12-15T06:26:46.3368818Z 
2019-12-15T06:26:46.3368962Z   git checkout -b <new-branch-name>
2019-12-15T06:26:46.3369678Z 
2019-12-15T06:26:46.3370344Z HEAD is now at 4fcb0c958 Auto merge of #67310 - Centril:rollup-22jiyow, r=Centril
2019-12-15T06:26:46.3814363Z ##[section]Starting: Setup environment
2019-12-15T06:26:46.3910234Z ==============================================================================
2019-12-15T06:26:46.3910327Z Task         : Bash
2019-12-15T06:26:46.3910418Z Description  : Run a Bash script on macOS, Linux, or Windows
---
2019-12-15T06:26:48.1229400Z BUILD_SOURCEBRANCHNAME=auto
2019-12-15T06:26:48.1229487Z BUILD_SOURCESDIRECTORY=D:\a\1\s
2019-12-15T06:26:48.1229589Z BUILD_SOURCEVERSION=4fcb0c958c5cefb5f2839841046beaff33ba7218
2019-12-15T06:26:48.1229669Z BUILD_SOURCEVERSIONAUTHOR=bors
2019-12-15T06:26:48.1229784Z BUILD_SOURCEVERSIONMESSAGE=Auto merge of #67310 - Centril:rollup-22jiyow, r=Centril
2019-12-15T06:26:48.1229970Z CI_JOB_NAME=i686-mingw-2
2019-12-15T06:26:48.1230042Z COBERTURA_HOME=C:\cobertura-2.1.1
2019-12-15T06:26:48.1230138Z COMMONPROGRAMFILES=C:\Program Files\Common Files
2019-12-15T06:26:48.1230238Z COMMON_TESTRESULTSDIRECTORY=D:\a\1\TestResults
---
2019-12-15T06:28:02.8892449Z Chocolatey installed 0/1 packages. 1 packages failed.
2019-12-15T06:28:02.8892561Z  See the log for details (C:\ProgramData\chocolatey\logs\chocolatey.log).
2019-12-15T06:28:02.8896581Z 
2019-12-15T06:28:02.8904995Z Failures
2019-12-15T06:28:02.8911465Z  - msys2 (exited 1) - msys2 not installed. An error occurred during installation:
2019-12-15T06:28:02.8911619Z  The remote server returned an error: (503) Server Unavailable. Service Unavailable
2019-12-15T06:28:03.8367961Z 
2019-12-15T06:28:03.8442765Z ##[error]Bash exited with code '1'.
2019-12-15T06:28:03.8597610Z ##[section]Starting: Checkout
2019-12-15T06:28:03.8710126Z ==============================================================================
2019-12-15T06:28:03.8710231Z Task         : Get sources
2019-12-15T06:28:03.8710347Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
