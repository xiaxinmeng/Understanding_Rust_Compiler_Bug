plain
2019-12-18T14:31:56.7722398Z do so (now or later) by using -b with the checkout command again. Example:
2019-12-18T14:31:56.7722451Z 
2019-12-18T14:31:56.7722678Z   git checkout -b <new-branch-name>
2019-12-18T14:31:56.7722796Z 
2019-12-18T14:31:56.7722888Z HEAD is now at 8b2914063 Auto merge of #67396 - Mark-Simulacrum:rollup-85lxz7h, r=Mark-Simulacrum
2019-12-18T14:31:56.8084334Z ##[section]Starting: Setup environment
2019-12-18T14:31:56.8190146Z ==============================================================================
2019-12-18T14:31:56.8190258Z Task         : Bash
2019-12-18T14:31:56.8190333Z Description  : Run a Bash script on macOS, Linux, or Windows
---
2019-12-18T14:33:59.1464466Z Chocolatey installed 0/1 packages. 1 packages failed.
2019-12-18T14:33:59.1464600Z  See the log for details (C:\ProgramData\chocolatey\logs\chocolatey.log).
2019-12-18T14:33:59.1468827Z 
2019-12-18T14:33:59.1473023Z Failures
2019-12-18T14:33:59.1479467Z  - msys2 (exited 1) - msys2 not installed. An error occurred during installation:
2019-12-18T14:33:59.1479638Z  The remote server returned an error: (503) Server Unavailable. Service Unavailable
2019-12-18T14:33:59.6377500Z 
2019-12-18T14:33:59.6474746Z ##[error]Bash exited with code '1'.
2019-12-18T14:33:59.6616639Z ##[section]Starting: Checkout
2019-12-18T14:33:59.6731803Z ==============================================================================
2019-12-18T14:33:59.6731987Z Task         : Get sources
2019-12-18T14:33:59.6732082Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
