plain
2019-10-26T04:05:26.6882267Z Chocolatey installed 0/1 packages. 1 packages failed.
2019-10-26T04:05:26.6882373Z  See the log for details (C:\ProgramData\chocolatey\logs\chocolatey.log).
2019-10-26T04:05:26.6886488Z 
2019-10-26T04:05:26.6890979Z Failures
2019-10-26T04:05:26.6897973Z  - msys2 (exited 1) - msys2 not installed. An error occurred during installation:
2019-10-26T04:05:26.6898130Z  The remote server returned an error: (503) Server Unavailable. Service Unavailable
2019-10-26T04:05:27.2648320Z 
2019-10-26T04:05:27.2727921Z ##[error]Bash exited with code '1'.
2019-10-26T04:05:27.2941242Z ##[section]Starting: Upload CPU usage statistics
2019-10-26T04:05:27.3050467Z ==============================================================================
2019-10-26T04:05:27.3050558Z Task         : Bash
2019-10-26T04:05:27.3050647Z Description  : Run a Bash script on macOS, Linux, or Windows
---
2019-10-26T04:05:27.5921823Z ========================== Starting Command Output ===========================
2019-10-26T04:05:27.5928678Z [command]"C:\Program Files\Git\bin\bash.exe" --noprofile --norc /d/a/_temp/14d1e8ad-af68-4cae-99bd-2df7273327fa.sh
2019-10-26T04:05:27.6371201Z /d/a/_temp/14d1e8ad-af68-4cae-99bd-2df7273327fa.sh: line 1: aws: command not found
2019-10-26T04:05:27.6403752Z 
2019-10-26T04:05:27.6425409Z ##[error]Bash exited with code '127'.
2019-10-26T04:05:27.6504119Z ##[section]Starting: Checkout
2019-10-26T04:05:27.6605334Z ==============================================================================
2019-10-26T04:05:27.6605441Z Task         : Get sources
2019-10-26T04:05:27.6605711Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
