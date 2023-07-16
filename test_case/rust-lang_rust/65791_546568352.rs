plain
2019-10-26T04:38:54.5744586Z do so (now or later) by using -b with the checkout command again. Example:
2019-10-26T04:38:54.5744644Z 
2019-10-26T04:38:54.5744748Z   git checkout -b <new-branch-name>
2019-10-26T04:38:54.5745069Z 
2019-10-26T04:38:54.5745207Z HEAD is now at 28b81eab4 Auto merge of #65791 - dorfsmay:doc_keyword_continue, r=Mark-Simulacrum
2019-10-26T04:38:54.6203374Z ##[section]Starting: Collect CPU-usage statistics in the background
2019-10-26T04:38:54.6309762Z ==============================================================================
2019-10-26T04:38:54.6309848Z Task         : Bash
2019-10-26T04:38:54.6309923Z Description  : Run a Bash script on macOS, Linux, or Windows
---
2019-10-26T04:40:34.4899910Z Chocolatey installed 0/1 packages. 1 packages failed.
2019-10-26T04:40:34.4900336Z  See the log for details (C:\ProgramData\chocolatey\logs\chocolatey.log).
2019-10-26T04:40:34.4905145Z 
2019-10-26T04:40:34.4910566Z Failures
2019-10-26T04:40:34.4917925Z  - msys2 (exited 1) - msys2 not installed. An error occurred during installation:
2019-10-26T04:40:34.4918421Z  The remote server returned an error: (503) Server Unavailable. Service Unavailable
2019-10-26T04:40:35.0386174Z 
2019-10-26T04:40:35.0477310Z ##[error]Bash exited with code '1'.
2019-10-26T04:40:35.0678397Z ##[section]Starting: Upload CPU usage statistics
2019-10-26T04:40:35.0790172Z ==============================================================================
2019-10-26T04:40:35.0790309Z Task         : Bash
2019-10-26T04:40:35.0790402Z Description  : Run a Bash script on macOS, Linux, or Windows
---
2019-10-26T04:40:35.4007024Z ========================== Starting Command Output ===========================
2019-10-26T04:40:35.4013152Z [command]"C:\Program Files\Git\bin\bash.exe" --noprofile --norc /d/a/_temp/fb93c018-2802-4299-9535-70c96a9147da.sh
2019-10-26T04:40:35.4482784Z /d/a/_temp/fb93c018-2802-4299-9535-70c96a9147da.sh: line 1: aws: command not found
2019-10-26T04:40:35.4517509Z 
2019-10-26T04:40:35.4536852Z ##[error]Bash exited with code '127'.
2019-10-26T04:40:35.4610899Z ##[section]Starting: Checkout
2019-10-26T04:40:35.4713112Z ==============================================================================
2019-10-26T04:40:35.4713227Z Task         : Get sources
2019-10-26T04:40:35.4713342Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
