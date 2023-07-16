plain
2019-11-25T01:14:03.8286760Z do so (now or later) by using -b with the checkout command again. Example:
2019-11-25T01:14:03.8286879Z 
2019-11-25T01:14:03.8287168Z   git checkout -b <new-branch-name>
2019-11-25T01:14:03.8287239Z 
2019-11-25T01:14:03.8287916Z HEAD is now at e35f2ada4 Auto merge of #66669 - petrochenkov:tup2attr, r=matthewjasper
2019-11-25T01:14:03.8663701Z ##[section]Starting: Decide whether to run this job
2019-11-25T01:14:03.8784626Z ==============================================================================
2019-11-25T01:14:03.8784779Z Task         : Bash
2019-11-25T01:14:03.8784920Z Description  : Run a Bash script on macOS, Linux, or Windows
---
2019-11-25T01:14:05.1286271Z BUILD_SOURCEBRANCHNAME=auto
2019-11-25T01:14:05.1286471Z BUILD_SOURCESDIRECTORY=D:\a\2\s
2019-11-25T01:14:05.1286685Z BUILD_SOURCEVERSION=e35f2ada430b6f0fd928529cd0ece66d18588a56
2019-11-25T01:14:05.1286895Z BUILD_SOURCEVERSIONAUTHOR=bors
2019-11-25T01:14:05.1287126Z BUILD_SOURCEVERSIONMESSAGE=Auto merge of #66669 - petrochenkov:tup2attr, r=matthewjasper
2019-11-25T01:14:05.1287625Z CI_JOB_NAME=dist-i686-msvc
2019-11-25T01:14:05.1287814Z COBERTURA_HOME=C:\cobertura-2.1.1
2019-11-25T01:14:05.1288022Z COMMONPROGRAMFILES=C:\Program Files\Common Files
2019-11-25T01:14:05.1288251Z COMMON_TESTRESULTSDIRECTORY=D:\a\2\TestResults
---
2019-11-25T01:18:56.2129239Z 
2019-11-25T01:18:56.2129672Z Total Download Size:    168.30 MiB
2019-11-25T01:18:56.2129935Z Total Installed Size:  1048.57 MiB
2019-11-25T01:18:56.2130206Z 
2019-11-25T01:18:56.2130488Z :: Proceed with installation? [Y/n] error: Partition / too full: 48241 blocks needed, 40153 blocks free
2019-11-25T01:18:56.2130643Z error: failed to commit transaction (not enough free disk space)
2019-11-25T01:18:56.2642209Z Errors occurred, no packages were upgraded.
2019-11-25T01:18:56.2667103Z 
2019-11-25T01:18:56.2667103Z 
2019-11-25T01:18:56.2783365Z ##[error]Bash exited with code '1'.
2019-11-25T01:18:56.2919688Z ##[section]Starting: Checkout
2019-11-25T01:18:56.3017431Z ==============================================================================
2019-11-25T01:18:56.3017572Z Task         : Get sources
2019-11-25T01:18:56.3017690Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
