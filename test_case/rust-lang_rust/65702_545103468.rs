plain
2019-10-22T18:51:44.2558529Z do so (now or later) by using -b with the checkout command again. Example:
2019-10-22T18:51:44.2558988Z 
2019-10-22T18:51:44.2559452Z   git checkout -b <new-branch-name>
2019-10-22T18:51:44.2559985Z 
2019-10-22T18:51:44.2560405Z HEAD is now at 40efa02d7 Auto merge of #65702 - JohnTitor:rollup-5014hez, r=JohnTitor
2019-10-22T18:51:44.2948710Z ##[section]Starting: Collect CPU-usage statistics in the background
2019-10-22T18:51:44.3063747Z ==============================================================================
2019-10-22T18:51:44.3063860Z Task         : Bash
2019-10-22T18:51:44.3063963Z Description  : Run a Bash script on macOS, Linux, or Windows
---
2019-10-22T18:51:46.1319516Z ==============================================================================
2019-10-22T18:51:49.9906875Z Generating script.
2019-10-22T18:51:50.0488550Z ========================== Starting Command Output ===========================
2019-10-22T18:51:50.0955743Z ##[command]"C:\windows\system32\cmd.exe" /D /E:ON /V:OFF /S /C "CALL "D:\a\_temp\5a9f90a7-9aab-4f10-8e23-57fc81a5c3ba.cmd""
2019-10-22T18:52:02.2033050Z iwr : The remote name could not be resolved: 'rust-lang-ci-mirrors.s3-us-west-1.amazonaws.com'
2019-10-22T18:52:02.2033943Z At line:1 char:43
2019-10-22T18:52:02.2034228Z + ... yContinue'; iwr -outf sccache\sccache.exe https://rust-lang-ci-mirror ...
2019-10-22T18:52:02.2034440Z +                 ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
2019-10-22T18:52:02.2034644Z     + CategoryInfo          : InvalidOperation: (System.Net.HttpWebRequest:HttpWebRequest) [Invoke-WebRequest], WebExc 
2019-10-22T18:52:02.2034849Z    eption
2019-10-22T18:52:02.2035056Z     + FullyQualifiedErrorId : WebCmdletWebResponseException,Microsoft.PowerShell.Commands.InvokeWebRequestCommand
2019-10-22T18:52:02.2035231Z  
2019-10-22T18:52:02.3381151Z ##[error]Cmd.exe exited with code '1'.
2019-10-22T18:52:02.4174365Z ##[section]Starting: Upload CPU usage statistics
2019-10-22T18:52:02.4300165Z ==============================================================================
2019-10-22T18:52:02.4300286Z Task         : Bash
2019-10-22T18:52:02.4300380Z Description  : Run a Bash script on macOS, Linux, or Windows
---
2019-10-22T18:52:02.7475403Z ========================== Starting Command Output ===========================
2019-10-22T18:52:02.7481176Z [command]"C:\Program Files\Git\bin\bash.exe" --noprofile --norc /d/a/_temp/66829b3c-273b-492e-9eb2-ed58a4272b41.sh
2019-10-22T18:52:02.7942241Z /d/a/_temp/66829b3c-273b-492e-9eb2-ed58a4272b41.sh: line 1: aws: command not found
2019-10-22T18:52:02.7978807Z 
2019-10-22T18:52:02.7994562Z ##[error]Bash exited with code '127'.
2019-10-22T18:52:02.8073746Z ##[section]Starting: Checkout
2019-10-22T18:52:02.8185439Z ==============================================================================
2019-10-22T18:52:02.8185541Z Task         : Get sources
2019-10-22T18:52:02.8185617Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
