plain
2019-10-26T04:58:39.5903084Z do so (now or later) by using -b with the checkout command again. Example:
2019-10-26T04:58:39.5903464Z 
2019-10-26T04:58:39.5903564Z   git checkout -b <new-branch-name>
2019-10-26T04:58:39.5903629Z 
2019-10-26T04:58:39.5903741Z HEAD is now at e5740fa9c Auto merge of #65828 - bjorn3:add_source_info_eq_hash, r=petrochenkov
2019-10-26T04:58:39.6267667Z ##[section]Starting: Collect CPU-usage statistics in the background
2019-10-26T04:58:39.6387904Z ==============================================================================
2019-10-26T04:58:39.6388138Z Task         : Bash
2019-10-26T04:58:39.6388214Z Description  : Run a Bash script on macOS, Linux, or Windows
---
2019-10-26T04:59:58.6955520Z Chocolatey installed 0/1 packages. 1 packages failed.
2019-10-26T04:59:58.6955646Z  See the log for details (C:\ProgramData\chocolatey\logs\chocolatey.log).
2019-10-26T04:59:58.6960807Z 
2019-10-26T04:59:58.6964348Z Failures
2019-10-26T04:59:58.6972233Z  - msys2 (exited 1) - msys2 not installed. An error occurred during installation:
2019-10-26T04:59:58.6972398Z  The remote server returned an error: (503) Server Unavailable. Service Unavailable
2019-10-26T04:59:59.1743829Z 
2019-10-26T04:59:59.1827675Z ##[error]Bash exited with code '1'.
2019-10-26T04:59:59.2027526Z ##[section]Starting: Upload CPU usage statistics
2019-10-26T04:59:59.2128509Z ==============================================================================
2019-10-26T04:59:59.2128603Z Task         : Bash
2019-10-26T04:59:59.2128673Z Description  : Run a Bash script on macOS, Linux, or Windows
---
2019-10-26T04:59:59.5092748Z ========================== Starting Command Output ===========================
2019-10-26T04:59:59.5099644Z [command]"C:\Program Files\Git\bin\bash.exe" --noprofile --norc /d/a/_temp/502e376a-18a5-437b-a30f-1284a01882fa.sh
2019-10-26T04:59:59.5523564Z /d/a/_temp/502e376a-18a5-437b-a30f-1284a01882fa.sh: line 1: aws: command not found
2019-10-26T04:59:59.5552430Z 
2019-10-26T04:59:59.5572018Z ##[error]Bash exited with code '127'.
2019-10-26T04:59:59.5646222Z ##[section]Starting: Checkout
2019-10-26T04:59:59.5749026Z ==============================================================================
2019-10-26T04:59:59.5749290Z Task         : Get sources
2019-10-26T04:59:59.5749369Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
