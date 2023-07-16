plain
2019-10-22T18:59:42.4854253Z ==============================================================================
2019-10-22T18:59:43.9986296Z Generating script.
2019-10-22T18:59:44.0426302Z ========================== Starting Command Output ===========================
2019-10-22T18:59:44.0816753Z ##[command]"C:\windows\system32\cmd.exe" /D /E:ON /V:OFF /S /C "CALL "D:\a\_temp\ec0e1344-145d-49e3-ad78-3aad0ff38233.cmd""
2019-10-22T18:59:56.4369587Z iwr : The remote name could not be resolved: 'rust-lang-ci-mirrors.s3-us-west-1.amazonaws.com'
2019-10-22T18:59:56.4369837Z At line:1 char:43
2019-10-22T18:59:56.4369995Z + ... yContinue'; iwr -outf sccache\sccache.exe https://rust-lang-ci-mirror ...
2019-10-22T18:59:56.4370109Z +                 ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
2019-10-22T18:59:56.4370254Z     + CategoryInfo          : InvalidOperation: (System.Net.HttpWebRequest:HttpWebRequest) [Invoke-WebRequest], WebExc 
2019-10-22T18:59:56.4370377Z    eption
2019-10-22T18:59:56.4370477Z     + FullyQualifiedErrorId : WebCmdletWebResponseException,Microsoft.PowerShell.Commands.InvokeWebRequestCommand
2019-10-22T18:59:56.4370818Z  
2019-10-22T18:59:56.5644998Z ##[error]Cmd.exe exited with code '1'.
2019-10-22T18:59:56.6575709Z ##[section]Starting: Upload CPU usage statistics
2019-10-22T18:59:56.6707473Z ==============================================================================
2019-10-22T18:59:56.6707583Z Task         : Bash
2019-10-22T18:59:56.6707679Z Description  : Run a Bash script on macOS, Linux, or Windows
---
2019-10-22T18:59:57.0092347Z ========================== Starting Command Output ===========================
2019-10-22T18:59:57.0099962Z [command]"C:\Program Files\Git\bin\bash.exe" --noprofile --norc /d/a/_temp/3d6acab5-5dbc-4e5d-bf2e-b529827d0a99.sh
2019-10-22T18:59:57.0637569Z /d/a/_temp/3d6acab5-5dbc-4e5d-bf2e-b529827d0a99.sh: line 1: aws: command not found
2019-10-22T18:59:57.0680507Z 
2019-10-22T18:59:57.0699688Z ##[error]Bash exited with code '127'.
2019-10-22T18:59:57.0789519Z ##[section]Starting: Checkout
2019-10-22T18:59:57.0910043Z ==============================================================================
2019-10-22T18:59:57.0910157Z Task         : Get sources
2019-10-22T18:59:57.0910236Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
