plain
2019-11-24T17:27:54.1738085Z do so (now or later) by using -b with the checkout command again. Example:
2019-11-24T17:27:54.1739356Z 
2019-11-24T17:27:54.1739588Z   git checkout -b <new-branch-name>
2019-11-24T17:27:54.1739800Z 
2019-11-24T17:27:54.1740026Z HEAD is now at 082c6481e Auto merge of #66540 - nnethercote:SmallVec-Candidate-match_pairs, r=matthewjasper
2019-11-24T17:27:54.2144215Z ##[section]Starting: Decide whether to run this job
2019-11-24T17:27:54.2242619Z ==============================================================================
2019-11-24T17:27:54.2242708Z Task         : Bash
2019-11-24T17:27:54.2242783Z Description  : Run a Bash script on macOS, Linux, or Windows
---
2019-11-24T17:27:55.4969342Z BUILD_SOURCEBRANCHNAME=auto
2019-11-24T17:27:55.4969436Z BUILD_SOURCESDIRECTORY=D:\a\1\s
2019-11-24T17:27:55.4969532Z BUILD_SOURCEVERSION=082c6481eaf5953d4e8ab51a223747859c64d51b
2019-11-24T17:27:55.4969638Z BUILD_SOURCEVERSIONAUTHOR=bors
2019-11-24T17:27:55.4969779Z BUILD_SOURCEVERSIONMESSAGE=Auto merge of #66540 - nnethercote:SmallVec-Candidate-match_pairs, r=matthewjasper
2019-11-24T17:27:55.4970065Z CI_JOB_NAME=x86_64-msvc-1
2019-11-24T17:27:55.4970146Z COBERTURA_HOME=C:\cobertura-2.1.1
2019-11-24T17:27:55.4970251Z COMMONPROGRAMFILES=C:\Program Files\Common Files
2019-11-24T17:27:55.4970343Z COMMON_TESTRESULTSDIRECTORY=D:\a\1\TestResults
---
2019-11-24T17:27:55.4985403Z USERDOMAIN=fv-az172
2019-11-24T17:27:55.4985492Z USERDOMAIN_ROAMINGPROFILE=fv-az172
2019-11-24T17:27:55.4985580Z USERNAME=VssAdministrator
2019-11-24T17:27:55.4985674Z USERPROFILE=C:\Users\VssAdministrator
2019-11-24T17:27:55.4985762Z Use a `SmallVec` for `Candidate::match_pairs`.
2019-11-24T17:27:55.4986026Z VS140COMNTOOLS=C:\Program Files (x86)\Microsoft Visual Studio 14.0\Common7\Tools\
2019-11-24T17:27:55.4986131Z VSTS_AGENT_PERFLOG=c:\vsts\perflog
2019-11-24T17:27:55.4986241Z VSTS_PROCESS_LOOKUP_ID=vsts_7422ad9a-1773-4e81-b4e8-c2485db483e2
2019-11-24T17:27:55.4986329Z WINDIR=C:\windows
---
2019-11-24T17:30:27.6250150Z 
2019-11-24T17:30:27.6250292Z 
2019-11-24T17:30:27.6880184Z kill: 1600: No such process
2019-11-24T17:30:27.6884039Z kill: 1599: No such process
2019-11-24T18:14:34.2477517Z Chocolatey timed out waiting for the command to finish. The timeout 
2019-11-24T18:14:34.2478154Z  specified (or the default value) was '2700' seconds. Perhaps try a 
2019-11-24T18:14:34.2478317Z  higher `--execution-timeout`? See `choco -h` for details.
2019-11-24T18:14:35.0759296Z The install of msys2 was NOT successful.
2019-11-24T18:14:35.0760784Z Error while running 'C:\ProgramData\chocolatey\lib\msys2\tools\chocolateyinstall.ps1'.
2019-11-24T18:14:35.0761420Z  See log for details.
2019-11-24T18:14:37.2064139Z Chocolatey installed 0/1 packages. 1 packages failed.
2019-11-24T18:14:37.2064400Z  See the log for details (C:\ProgramData\chocolatey\logs\chocolatey.log).
2019-11-24T18:14:37.2068465Z 
2019-11-24T18:14:37.2074365Z Failures
2019-11-24T18:14:37.2074365Z Failures
2019-11-24T18:14:37.2081174Z  - msys2 (exited -1) - Error while running 'C:\ProgramData\chocolatey\lib\msys2\tools\chocolateyinstall.ps1'.
2019-11-24T18:14:37.2081536Z  See log for details.
2019-11-24T18:14:47.6596291Z The STDIO streams did not close within 10 seconds of the exit event from process 'C:\Program Files\Git\bin\bash.exe'. This may indicate a child process inherited the STDIO streams and has not yet exited.
2019-11-24T18:14:47.6686013Z ##[error]Bash exited with code '127'.
2019-11-24T18:14:52.6889956Z ##[section]Starting: Checkout
2019-11-24T18:14:52.7019819Z ==============================================================================
2019-11-24T18:14:52.7019976Z Task         : Get sources
2019-11-24T18:14:52.7020074Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
