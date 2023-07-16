plain
2019-10-26T04:34:37.2097366Z Chocolatey installed 0/1 packages. 1 packages failed.
2019-10-26T04:34:37.2097840Z  See the log for details (C:\ProgramData\chocolatey\logs\chocolatey.log).
2019-10-26T04:34:37.2101409Z 
2019-10-26T04:34:37.2106883Z Failures
2019-10-26T04:34:37.2114059Z  - msys2 (exited 1) - msys2 not installed. An error occurred during installation:
2019-10-26T04:34:37.2114739Z  The remote server returned an error: (503) Server Unavailable. Service Unavailable
2019-10-26T04:34:37.6869507Z 
2019-10-26T04:34:37.6987308Z ##[error]Bash exited with code '1'.
2019-10-26T04:34:37.7196791Z ##[section]Starting: Upload CPU usage statistics
2019-10-26T04:34:37.7303864Z ==============================================================================
2019-10-26T04:34:37.7303953Z Task         : Bash
2019-10-26T04:34:37.7304022Z Description  : Run a Bash script on macOS, Linux, or Windows
---
2019-10-26T04:34:38.0423633Z ========================== Starting Command Output ===========================
2019-10-26T04:34:38.0447526Z [command]"C:\Program Files\Git\bin\bash.exe" --noprofile --norc /d/a/_temp/75103684-24f8-43d5-bc4f-9b9fc5db0e5d.sh
2019-10-26T04:34:38.0649871Z /d/a/_temp/75103684-24f8-43d5-bc4f-9b9fc5db0e5d.sh: line 1: aws: command not found
2019-10-26T04:34:38.0683021Z 
2019-10-26T04:34:38.0703867Z ##[error]Bash exited with code '127'.
2019-10-26T04:34:38.0777891Z ##[section]Starting: Checkout
2019-10-26T04:34:38.0884893Z ==============================================================================
2019-10-26T04:34:38.0885002Z Task         : Get sources
2019-10-26T04:34:38.0885206Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
