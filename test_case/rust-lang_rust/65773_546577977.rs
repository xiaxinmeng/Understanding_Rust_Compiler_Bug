plain
2019-10-26T07:14:49.4086574Z Chocolatey installed 0/1 packages. 1 packages failed.
2019-10-26T07:14:49.4087085Z  See the log for details (C:\ProgramData\chocolatey\logs\chocolatey.log).
2019-10-26T07:14:49.4091594Z 
2019-10-26T07:14:49.4095897Z Failures
2019-10-26T07:14:49.4104698Z  - msys2 (exited 1) - msys2 not installed. An error occurred during installation:
2019-10-26T07:14:49.4105276Z  The remote server returned an error: (503) Server Unavailable. Service Unavailable
2019-10-26T07:14:49.8732406Z 
2019-10-26T07:14:49.8821286Z ##[error]Bash exited with code '1'.
2019-10-26T07:14:49.9077846Z ##[section]Starting: Upload CPU usage statistics
2019-10-26T07:14:49.9201757Z ==============================================================================
2019-10-26T07:14:49.9201876Z Task         : Bash
2019-10-26T07:14:49.9201969Z Description  : Run a Bash script on macOS, Linux, or Windows
---
2019-10-26T07:14:50.2225540Z ========================== Starting Command Output ===========================
2019-10-26T07:14:50.2231853Z [command]"C:\Program Files\Git\bin\bash.exe" --noprofile --norc /d/a/_temp/767215e5-d024-48f7-a883-aedb6e16d401.sh
2019-10-26T07:14:50.2699432Z /d/a/_temp/767215e5-d024-48f7-a883-aedb6e16d401.sh: line 1: aws: command not found
2019-10-26T07:14:50.2741112Z 
2019-10-26T07:14:50.2762006Z ##[error]Bash exited with code '127'.
2019-10-26T07:14:50.2837297Z ##[section]Starting: Checkout
2019-10-26T07:14:50.2948097Z ==============================================================================
2019-10-26T07:14:50.2948197Z Task         : Get sources
2019-10-26T07:14:50.2948307Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
