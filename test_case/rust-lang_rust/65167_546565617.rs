plain
2019-10-26T03:56:11.0497355Z do so (now or later) by using -b with the checkout command again. Example:
2019-10-26T03:56:11.0497581Z 
2019-10-26T03:56:11.0497746Z   git checkout -b <new-branch-name>
2019-10-26T03:56:11.0497861Z 
2019-10-26T03:56:11.0498031Z HEAD is now at 5fa0e21b8 Auto merge of #65167 - hermitcore:rusty-hermit, r=alexcrichton
2019-10-26T03:56:11.0913583Z ##[section]Starting: Collect CPU-usage statistics in the background
2019-10-26T03:56:11.1024042Z ==============================================================================
2019-10-26T03:56:11.1024140Z Task         : Bash
2019-10-26T03:56:11.1024222Z Description  : Run a Bash script on macOS, Linux, or Windows
---
2019-10-26T03:57:46.8170327Z Chocolatey installed 0/1 packages. 1 packages failed.
2019-10-26T03:57:46.8170789Z  See the log for details (C:\ProgramData\chocolatey\logs\chocolatey.log).
2019-10-26T03:57:46.8174514Z 
2019-10-26T03:57:46.8179381Z Failures
2019-10-26T03:57:46.8186383Z  - msys2 (exited 1) - msys2 not installed. An error occurred during installation:
2019-10-26T03:57:46.8186877Z  The remote server returned an error: (503) Server Unavailable. Service Unavailable
2019-10-26T03:57:47.7846462Z 
2019-10-26T03:57:47.7921255Z ##[error]Bash exited with code '1'.
2019-10-26T03:57:47.8124985Z ##[section]Starting: Upload CPU usage statistics
2019-10-26T03:57:47.8233174Z ==============================================================================
2019-10-26T03:57:47.8233448Z Task         : Bash
2019-10-26T03:57:47.8233514Z Description  : Run a Bash script on macOS, Linux, or Windows
---
2019-10-26T03:57:48.1203151Z ========================== Starting Command Output ===========================
2019-10-26T03:57:48.1208447Z [command]"C:\Program Files\Git\bin\bash.exe" --noprofile --norc /d/a/_temp/cd96e239-20e8-4d66-9fcc-8ca1acb68e44.sh
2019-10-26T03:57:48.1632835Z /d/a/_temp/cd96e239-20e8-4d66-9fcc-8ca1acb68e44.sh: line 1: aws: command not found
2019-10-26T03:57:48.1664611Z 
2019-10-26T03:57:48.1683372Z ##[error]Bash exited with code '127'.
2019-10-26T03:57:48.1758216Z ##[section]Starting: Checkout
2019-10-26T03:57:48.1860806Z ==============================================================================
2019-10-26T03:57:48.1880077Z Task         : Get sources
2019-10-26T03:57:48.1880189Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
