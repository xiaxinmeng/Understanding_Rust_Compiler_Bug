plain
2019-10-26T06:41:51.1335896Z Chocolatey installed 0/1 packages. 1 packages failed.
2019-10-26T06:41:51.1336016Z  See the log for details (C:\ProgramData\chocolatey\logs\chocolatey.log).
2019-10-26T06:41:51.1339362Z 
2019-10-26T06:41:51.1343989Z Failures
2019-10-26T06:41:51.1351536Z  - msys2 (exited 1) - msys2 not installed. An error occurred during installation:
2019-10-26T06:41:51.1351710Z  The remote server returned an error: (503) Server Unavailable. Service Unavailable
2019-10-26T06:41:51.6518368Z 
2019-10-26T06:41:51.6570564Z ##[error]Bash exited with code '1'.
2019-10-26T06:41:51.6760546Z ##[section]Starting: Upload CPU usage statistics
2019-10-26T06:41:51.6861240Z ==============================================================================
2019-10-26T06:41:51.6861358Z Task         : Bash
2019-10-26T06:41:51.6861444Z Description  : Run a Bash script on macOS, Linux, or Windows
---
2019-10-26T06:41:52.0178176Z ========================== Starting Command Output ===========================
2019-10-26T06:41:52.0179197Z [command]"C:\Program Files\Git\bin\bash.exe" --noprofile --norc /d/a/_temp/771d4433-a742-4c1f-bf9d-2f529b54fecc.sh
2019-10-26T06:41:52.0179318Z /d/a/_temp/771d4433-a742-4c1f-bf9d-2f529b54fecc.sh: line 1: aws: command not found
2019-10-26T06:41:52.0179426Z 
2019-10-26T06:41:52.0180789Z ##[error]Bash exited with code '127'.
2019-10-26T06:41:52.0198276Z ##[section]Starting: Checkout
2019-10-26T06:41:52.0296382Z ==============================================================================
2019-10-26T06:41:52.0296476Z Task         : Get sources
2019-10-26T06:41:52.0296575Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
