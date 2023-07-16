plain
2019-10-22T19:11:29.5676683Z do so (now or later) by using -b with the checkout command again. Example:
2019-10-22T19:11:29.5676754Z 
2019-10-22T19:11:29.5676834Z   git checkout -b <new-branch-name>
2019-10-22T19:11:29.5676890Z 
2019-10-22T19:11:29.5677015Z HEAD is now at 93193b822 Auto merge of #65074 - Rantanen:json-byte-pos, r=matklad
2019-10-22T19:11:29.6094268Z ##[section]Starting: Collect CPU-usage statistics in the background
2019-10-22T19:11:29.6228055Z ==============================================================================
2019-10-22T19:11:29.6228152Z Task         : Bash
2019-10-22T19:11:29.6228245Z Description  : Run a Bash script on macOS, Linux, or Windows
---
2019-10-22T19:11:30.6842714Z ==============================================================================
2019-10-22T19:11:32.4133421Z Generating script.
2019-10-22T19:11:32.4196578Z ========================== Starting Command Output ===========================
2019-10-22T19:11:32.4579240Z ##[command]"C:\windows\system32\cmd.exe" /D /E:ON /V:OFF /S /C "CALL "D:\a\_temp\98eab51a-8581-4c15-a65d-9cca954d7520.cmd""
2019-10-22T19:11:44.8694315Z iwr : The remote name could not be resolved: 'rust-lang-ci-mirrors.s3-us-west-1.amazonaws.com'
2019-10-22T19:11:44.8694644Z At line:1 char:43
2019-10-22T19:11:44.8694744Z + ... yContinue'; iwr -outf sccache\sccache.exe https://rust-lang-ci-mirror ...
2019-10-22T19:11:44.8695434Z +                 ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
2019-10-22T19:11:44.8696092Z     + CategoryInfo          : InvalidOperation: (System.Net.HttpWebRequest:HttpWebRequest) [Invoke-WebRequest], WebExc 
2019-10-22T19:11:44.8696701Z    eption
2019-10-22T19:11:44.8697698Z     + FullyQualifiedErrorId : WebCmdletWebResponseException,Microsoft.PowerShell.Commands.InvokeWebRequestCommand
2019-10-22T19:11:44.8697978Z  
2019-10-22T19:11:45.0008378Z ##[error]Cmd.exe exited with code '1'.
2019-10-22T19:11:45.0794337Z ##[section]Starting: Upload CPU usage statistics
2019-10-22T19:11:45.0988325Z ==============================================================================
2019-10-22T19:11:45.0988464Z Task         : Bash
2019-10-22T19:11:45.0988546Z Description  : Run a Bash script on macOS, Linux, or Windows
---
2019-10-22T19:11:45.4330899Z ========================== Starting Command Output ===========================
2019-10-22T19:11:45.4337286Z [command]"C:\Program Files\Git\bin\bash.exe" --noprofile --norc /d/a/_temp/e72207d0-9cf0-4fcd-8292-6d34481d4715.sh
2019-10-22T19:11:45.4830315Z /d/a/_temp/e72207d0-9cf0-4fcd-8292-6d34481d4715.sh: line 1: aws: command not found
2019-10-22T19:11:45.4877604Z 
2019-10-22T19:11:45.4900118Z ##[error]Bash exited with code '127'.
2019-10-22T19:11:45.4977285Z ##[section]Starting: Checkout
2019-10-22T19:11:45.5089986Z ==============================================================================
2019-10-22T19:11:45.5090088Z Task         : Get sources
2019-10-22T19:11:45.5090198Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
