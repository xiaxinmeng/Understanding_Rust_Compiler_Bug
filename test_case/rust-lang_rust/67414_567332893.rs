plain
2019-12-19T04:54:14.2844448Z do so (now or later) by using -b with the checkout command again. Example:
2019-12-19T04:54:14.2844688Z 
2019-12-19T04:54:14.2844894Z   git checkout -b <new-branch-name>
2019-12-19T04:54:14.2845251Z 
2019-12-19T04:54:14.2845456Z HEAD is now at 246bf368c Auto merge of #67414 - Mark-Simulacrum:rollup-gxjwn6c, r=Mark-Simulacrum
2019-12-19T04:54:14.3248765Z ##[section]Starting: Setup environment
2019-12-19T04:54:14.3356992Z ==============================================================================
2019-12-19T04:54:14.3357102Z Task         : Bash
2019-12-19T04:54:14.3357178Z Description  : Run a Bash script on macOS, Linux, or Windows
---
2019-12-19T04:55:57.0550465Z Chocolatey installed 0/1 packages. 1 packages failed.
2019-12-19T04:55:57.0550876Z  See the log for details (C:\ProgramData\chocolatey\logs\chocolatey.log).
2019-12-19T04:55:57.0554644Z 
2019-12-19T04:55:57.0559534Z Failures
2019-12-19T04:55:57.0565881Z  - msys2 (exited 1) - msys2 not installed. An error occurred during installation:
2019-12-19T04:55:57.0566410Z  The remote server returned an error: (503) Server Unavailable. Service Unavailable
2019-12-19T04:55:57.5719830Z 
2019-12-19T04:55:57.5845665Z ##[error]Bash exited with code '1'.
2019-12-19T04:55:57.5969311Z ##[section]Starting: Checkout
2019-12-19T04:55:57.6076942Z ==============================================================================
2019-12-19T04:55:57.6077031Z Task         : Get sources
2019-12-19T04:55:57.6077130Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
