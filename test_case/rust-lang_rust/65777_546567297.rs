plain
2019-10-26T04:21:51.0368825Z Chocolatey installed 0/1 packages. 1 packages failed.
2019-10-26T04:21:51.0369076Z  See the log for details (C:\ProgramData\chocolatey\logs\chocolatey.log).
2019-10-26T04:21:51.0372764Z 
2019-10-26T04:21:51.0377724Z Failures
2019-10-26T04:21:51.0383380Z  - msys2 (exited 1) - msys2 not installed. An error occurred during installation:
2019-10-26T04:21:51.0383699Z  The remote server returned an error: (503) Server Unavailable. Service Unavailable
2019-10-26T04:21:51.4993028Z 
2019-10-26T04:21:51.5083704Z ##[error]Bash exited with code '1'.
2019-10-26T04:21:51.5323919Z ##[section]Starting: Upload CPU usage statistics
2019-10-26T04:21:51.5437491Z ==============================================================================
2019-10-26T04:21:51.5437769Z Task         : Bash
2019-10-26T04:21:51.5437872Z Description  : Run a Bash script on macOS, Linux, or Windows
---
2019-10-26T04:21:51.8471387Z ========================== Starting Command Output ===========================
2019-10-26T04:21:51.8484474Z [command]"C:\Program Files\Git\bin\bash.exe" --noprofile --norc /d/a/_temp/c21719a1-1431-4e0a-87b5-792353b371ea.sh
2019-10-26T04:21:51.8887027Z /d/a/_temp/c21719a1-1431-4e0a-87b5-792353b371ea.sh: line 1: aws: command not found
2019-10-26T04:21:51.8919375Z 
2019-10-26T04:21:51.8940640Z ##[error]Bash exited with code '127'.
2019-10-26T04:21:51.9019850Z ##[section]Starting: Checkout
2019-10-26T04:21:51.9123350Z ==============================================================================
2019-10-26T04:21:51.9123475Z Task         : Get sources
2019-10-26T04:21:51.9123578Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
