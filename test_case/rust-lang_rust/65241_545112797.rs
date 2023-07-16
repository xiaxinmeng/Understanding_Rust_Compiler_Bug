plain
2019-10-22T19:15:23.7579801Z do so (now or later) by using -b with the checkout command again. Example:
2019-10-22T19:15:23.7580803Z 
2019-10-22T19:15:23.7581161Z   git checkout -b <new-branch-name>
2019-10-22T19:15:23.7581331Z 
2019-10-22T19:15:23.7581527Z HEAD is now at a6197db24 Auto merge of #65241 - tmiasko:no-std-san, r=nikomatsakis
2019-10-22T19:15:23.8023838Z ##[section]Starting: Collect CPU-usage statistics in the background
2019-10-22T19:15:23.8156471Z ==============================================================================
2019-10-22T19:15:23.8156571Z Task         : Bash
2019-10-22T19:15:23.8156669Z Description  : Run a Bash script on macOS, Linux, or Windows
---
2019-10-22T19:15:24.7699512Z Help         : https://docs.microsoft.com/azure/devops/pipelines/tasks/utility/command-line
2019-10-22T19:15:24.7699624Z ==============================================================================
2019-10-22T19:15:26.2135601Z Generating script.
2019-10-22T19:15:26.2537400Z ========================== Starting Command Output ===========================
2019-10-22T19:15:26.2809850Z ##[command]"C:\windows\system32\cmd.exe" /D /E:ON /V:OFF /S /C "CALL "D:\a\_temp\5a129283-f8da-4615-b961-3b75f0400cdd.cmd""
2019-10-22T19:15:38.3296236Z iwr : The remote name could not be resolved: 'rust-lang-ci-mirrors.s3-us-west-1.amazonaws.com'
2019-10-22T19:15:38.3297186Z At line:1 char:43
2019-10-22T19:15:38.3297489Z + ... yContinue'; iwr -outf sccache\sccache.exe https://rust-lang-ci-mirror ...
2019-10-22T19:15:38.3297729Z +                 ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
2019-10-22T19:15:38.3299627Z     + CategoryInfo          : InvalidOperation: (System.Net.HttpWebRequest:HttpWebRequest) [Invoke-WebRequest], WebExc 
2019-10-22T19:15:38.3300090Z    eption
2019-10-22T19:15:38.3300216Z     + FullyQualifiedErrorId : WebCmdletWebResponseException,Microsoft.PowerShell.Commands.InvokeWebRequestCommand
2019-10-22T19:15:38.3300343Z  
2019-10-22T19:15:38.5127927Z ##[error]Cmd.exe exited with code '1'.
2019-10-22T19:15:38.6073031Z ##[section]Starting: Upload CPU usage statistics
2019-10-22T19:15:38.6208035Z ==============================================================================
2019-10-22T19:15:38.6208155Z Task         : Bash
2019-10-22T19:15:38.6208255Z Description  : Run a Bash script on macOS, Linux, or Windows
---
2019-10-22T19:15:38.9582139Z ========================== Starting Command Output ===========================
2019-10-22T19:15:38.9589023Z [command]"C:\Program Files\Git\bin\bash.exe" --noprofile --norc /d/a/_temp/74b9bd72-e5d3-41c4-808b-8f57897e3e23.sh
2019-10-22T19:15:39.0048248Z /d/a/_temp/74b9bd72-e5d3-41c4-808b-8f57897e3e23.sh: line 1: aws: command not found
2019-10-22T19:15:39.0086711Z 
2019-10-22T19:15:39.0109070Z ##[error]Bash exited with code '127'.
2019-10-22T19:15:39.0200119Z ##[section]Starting: Checkout
2019-10-22T19:15:39.0313129Z ==============================================================================
2019-10-22T19:15:39.0313237Z Task         : Get sources
2019-10-22T19:15:39.0313334Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
