plain
2019-12-19T19:48:27.1154327Z do so (now or later) by using -b with the checkout command again. Example:
2019-12-19T19:48:27.1154604Z 
2019-12-19T19:48:27.1154886Z   git checkout -b <new-branch-name>
2019-12-19T19:48:27.1155318Z 
2019-12-19T19:48:27.1155634Z HEAD is now at 3eb96210f Auto merge of #67435 - Mark-Simulacrum:rollup-tvlt9px, r=Mark-Simulacrum
2019-12-19T19:48:27.1559894Z ##[section]Starting: Setup environment
2019-12-19T19:48:27.1675966Z ==============================================================================
2019-12-19T19:48:27.1676222Z Task         : Bash
2019-12-19T19:48:27.1676316Z Description  : Run a Bash script on macOS, Linux, or Windows
---
2019-12-19T19:50:02.7967236Z Chocolatey installed 0/1 packages. 1 packages failed.
2019-12-19T19:50:02.7967742Z  See the log for details (C:\ProgramData\chocolatey\logs\chocolatey.log).
2019-12-19T19:50:02.7970851Z 
2019-12-19T19:50:02.7976328Z Failures
2019-12-19T19:50:02.7982969Z  - msys2 (exited 1) - msys2 not installed. An error occurred during installation:
2019-12-19T19:50:02.7983597Z  The remote server returned an error: (503) Server Unavailable. Service Unavailable
2019-12-19T19:50:03.2816686Z 
2019-12-19T19:50:03.2906677Z ##[error]Bash exited with code '1'.
2019-12-19T19:50:03.3038028Z ##[section]Starting: Checkout
2019-12-19T19:50:03.3146019Z ==============================================================================
2019-12-19T19:50:03.3146123Z Task         : Get sources
2019-12-19T19:50:03.3146320Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
