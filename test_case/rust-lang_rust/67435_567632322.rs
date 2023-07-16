plain
2019-12-19T19:33:24.6178167Z do so (now or later) by using -b with the checkout command again. Example:
2019-12-19T19:33:24.6178321Z 
2019-12-19T19:33:24.6178508Z   git checkout -b <new-branch-name>
2019-12-19T19:33:24.6178653Z 
2019-12-19T19:33:24.6178874Z HEAD is now at f5a30f554 Auto merge of #67435 - Mark-Simulacrum:rollup-tvlt9px, r=Mark-Simulacrum
2019-12-19T19:33:24.6557718Z ##[section]Starting: Setup environment
2019-12-19T19:33:24.6663278Z ==============================================================================
2019-12-19T19:33:24.6663396Z Task         : Bash
2019-12-19T19:33:24.6663476Z Description  : Run a Bash script on macOS, Linux, or Windows
---
2019-12-19T19:35:02.0345843Z Chocolatey installed 0/1 packages. 1 packages failed.
2019-12-19T19:35:02.0345923Z  See the log for details (C:\ProgramData\chocolatey\logs\chocolatey.log).
2019-12-19T19:35:02.0350713Z 
2019-12-19T19:35:02.0354568Z Failures
2019-12-19T19:35:02.0360986Z  - msys2 (exited 1) - msys2 not installed. An error occurred during installation:
2019-12-19T19:35:02.0361092Z  The remote server returned an error: (503) Server Unavailable. Service Unavailable
2019-12-19T19:35:02.5434705Z 
2019-12-19T19:35:02.5538594Z ##[error]Bash exited with code '1'.
2019-12-19T19:35:02.5708232Z ##[section]Starting: Checkout
2019-12-19T19:35:02.5818198Z ==============================================================================
2019-12-19T19:35:02.5818298Z Task         : Get sources
2019-12-19T19:35:02.5818407Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
