plain
2019-10-26T04:24:06.3944573Z do so (now or later) by using -b with the checkout command again. Example:
2019-10-26T04:24:06.3944715Z 
2019-10-26T04:24:06.3944916Z   git checkout -b <new-branch-name>
2019-10-26T04:24:06.3945120Z 
2019-10-26T04:24:06.3945528Z HEAD is now at 41bcc6f94 Auto merge of #65834 - Mark-Simulacrum:driver-clean, r=nikomatsakis
2019-10-26T04:24:06.4313384Z ##[section]Starting: Collect CPU-usage statistics in the background
2019-10-26T04:24:06.4436906Z ==============================================================================
2019-10-26T04:24:06.4437016Z Task         : Bash
2019-10-26T04:24:06.4437093Z Description  : Run a Bash script on macOS, Linux, or Windows
---
2019-10-26T04:25:40.8742733Z Chocolatey installed 0/1 packages. 1 packages failed.
2019-10-26T04:25:40.8742854Z  See the log for details (C:\ProgramData\chocolatey\logs\chocolatey.log).
2019-10-26T04:25:40.8746009Z 
2019-10-26T04:25:40.8751449Z Failures
2019-10-26T04:25:40.8758069Z  - msys2 (exited 1) - msys2 not installed. An error occurred during installation:
2019-10-26T04:25:40.8758222Z  The remote server returned an error: (503) Server Unavailable. Service Unavailable
2019-10-26T04:25:41.4341517Z 
2019-10-26T04:25:41.4437869Z ##[error]Bash exited with code '1'.
2019-10-26T04:25:41.4651180Z ##[section]Starting: Upload CPU usage statistics
2019-10-26T04:25:41.4776567Z ==============================================================================
2019-10-26T04:25:41.4777082Z Task         : Bash
2019-10-26T04:25:41.4777184Z Description  : Run a Bash script on macOS, Linux, or Windows
---
2019-10-26T04:25:41.7747965Z ========================== Starting Command Output ===========================
2019-10-26T04:25:41.7753477Z [command]"C:\Program Files\Git\bin\bash.exe" --noprofile --norc /d/a/_temp/6f81ee19-9b79-46b7-b89a-076bcf39e409.sh
2019-10-26T04:25:41.8198448Z /d/a/_temp/6f81ee19-9b79-46b7-b89a-076bcf39e409.sh: line 1: aws: command not found
2019-10-26T04:25:41.8231746Z 
2019-10-26T04:25:41.8249617Z ##[error]Bash exited with code '127'.
2019-10-26T04:25:41.8322916Z ##[section]Starting: Checkout
2019-10-26T04:25:41.8429963Z ==============================================================================
2019-10-26T04:25:41.8430078Z Task         : Get sources
2019-10-26T04:25:41.8430169Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
