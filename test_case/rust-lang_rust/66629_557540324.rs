plain
2019-11-22T13:49:09.0471515Z do so (now or later) by using -b with the checkout command again. Example:
2019-11-22T13:49:09.0471871Z 
2019-11-22T13:49:09.0472141Z   git checkout -b <new-branch-name>
2019-11-22T13:49:09.0473696Z 
2019-11-22T13:49:09.0473872Z HEAD is now at c02df0863 Auto merge of #66629 - pietroalbini:rollup-mbxzl5c, r=pietroalbini
2019-11-22T13:49:09.0882325Z ##[section]Starting: Setup environment
2019-11-22T13:49:09.0994756Z ==============================================================================
2019-11-22T13:49:09.0994880Z Task         : Bash
2019-11-22T13:49:09.0994987Z Description  : Run a Bash script on macOS, Linux, or Windows
---
2019-11-22T13:49:10.9205234Z 
2019-11-22T13:49:10.9205402Z 
2019-11-22T13:49:10.9205527Z 
2019-11-22T13:49:10.9205647Z 
2019-11-22T13:49:10.9205854Z  - #66566 (Document pitfall with `impl PartialEq<B> for A`)
2019-11-22T13:49:10.9206045Z  - #66569 (GitHub Actions: preparations, part 1)
2019-11-22T13:49:10.9206260Z  - #66576 (made gdb pretty-printing more robust when printing uninitialized vec)
2019-11-22T13:49:10.9206633Z AGENT_DISABLELOGPLUGIN_TESTFILEPUBLISHERPLUGIN=true
2019-11-22T13:49:10.9206832Z AGENT_DISABLELOGPLUGIN_TESTRESULTLOGPLUGIN=true
2019-11-22T13:49:10.9207013Z AGENT_HOMEDIRECTORY=C:\agents\2.160.1
2019-11-22T13:49:10.9207241Z AGENT_ID=518
---
2019-11-22T13:49:10.9224534Z BUILD_SOURCEBRANCHNAME=auto
2019-11-22T13:49:10.9224633Z BUILD_SOURCESDIRECTORY=D:\a\1\s
2019-11-22T13:49:10.9225038Z BUILD_SOURCEVERSION=c02df086343793229ff6a16f333fb6fe9769aea9
2019-11-22T13:49:10.9225144Z BUILD_SOURCEVERSIONAUTHOR=bors
2019-11-22T13:49:10.9225256Z BUILD_SOURCEVERSIONMESSAGE=Auto merge of #66629 - pietroalbini:rollup-mbxzl5c, r=pietroalbini
2019-11-22T13:49:10.9225455Z CI_JOB_NAME=i686-msvc-1
2019-11-22T13:49:10.9225526Z COBERTURA_HOME=C:\cobertura-2.1.1
2019-11-22T13:49:10.9225628Z COMMONPROGRAMFILES=C:\Program Files\Common Files
2019-11-22T13:49:10.9225714Z COMMON_TESTRESULTSDIRECTORY=D:\a\1\TestResults
---
2019-11-22T13:53:56.1559159Z  there is nothing to do
2019-11-22T13:53:56.1559471Z :: Starting full system upgrade...
2019-11-22T13:53:56.2248927Z  there is nothing to do
2019-11-22T13:53:56.2812182Z kill: 1703: No such process
2019-11-22T13:53:57.1653974Z PATH environment variable does not have C:\tools\msys64 in it. Adding...
2019-11-22T13:53:57.5814484Z Environment Vars (like PATH) have changed. Close/reopen your shell to
2019-11-22T13:53:57.5815016Z  see the changes (or in powershell/cmd.exe just type `refreshenv`).
2019-11-22T13:53:58.1786587Z   Software installed to 'C:\tools\msys64'
2019-11-22T13:53:58.2489434Z 
2019-11-22T13:53:58.2490729Z Chocolatey installed 1/1 packages. 
2019-11-22T13:53:58.2491107Z  See the log for details (C:\ProgramData\chocolatey\logs\chocolatey.log).
---
2019-11-22T13:53:59.0909928Z ========================== Starting Command Output ===========================
2019-11-22T13:53:59.0915614Z [command]"C:\Program Files\Git\bin\bash.exe" --noprofile --norc /d/a/_temp/0c07008b-0c30-43a6-b0e5-8caa79fb808c.sh
2019-11-22T13:53:59.1909722Z src/ci/scripts/install-msys2-packages.sh: line 9: pacman: command not found
2019-11-22T13:53:59.1969606Z 
2019-11-22T13:53:59.2059833Z ##[error]Bash exited with code '127'.
2019-11-22T13:53:59.2182503Z ##[section]Starting: Checkout
2019-11-22T13:53:59.2285166Z ==============================================================================
2019-11-22T13:53:59.2285259Z Task         : Get sources
2019-11-22T13:53:59.2285364Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
