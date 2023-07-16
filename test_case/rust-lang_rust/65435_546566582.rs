plain
2019-10-26T04:09:53.7025687Z Chocolatey installed 0/1 packages. 1 packages failed.
2019-10-26T04:09:53.7025972Z  See the log for details (C:\ProgramData\chocolatey\logs\chocolatey.log).
2019-10-26T04:09:53.7031480Z 
2019-10-26T04:09:53.7035517Z Failures
2019-10-26T04:09:53.7043198Z  - msys2 (exited 1) - msys2 not installed. An error occurred during installation:
2019-10-26T04:09:53.7043515Z  The remote server returned an error: (503) Server Unavailable. Service Unavailable
2019-10-26T04:09:54.2236864Z 
2019-10-26T04:09:54.2406214Z ##[error]Bash exited with code '1'.
2019-10-26T04:09:54.2643178Z ##[section]Starting: Upload CPU usage statistics
2019-10-26T04:09:54.2755895Z ==============================================================================
2019-10-26T04:09:54.2756034Z Task         : Bash
2019-10-26T04:09:54.2756127Z Description  : Run a Bash script on macOS, Linux, or Windows
---
2019-10-26T04:09:54.5843400Z ========================== Starting Command Output ===========================
2019-10-26T04:09:54.5848566Z [command]"C:\Program Files\Git\bin\bash.exe" --noprofile --norc /d/a/_temp/e588f7a8-bb10-4c24-beeb-88e6d26393d9.sh
2019-10-26T04:09:54.6289207Z /d/a/_temp/e588f7a8-bb10-4c24-beeb-88e6d26393d9.sh: line 1: aws: command not found
2019-10-26T04:09:54.6319532Z 
2019-10-26T04:09:54.6340843Z ##[error]Bash exited with code '127'.
2019-10-26T04:09:54.6420142Z ##[section]Starting: Checkout
2019-10-26T04:09:54.6519784Z ==============================================================================
2019-10-26T04:09:54.6519870Z Task         : Get sources
2019-10-26T04:09:54.6519975Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
