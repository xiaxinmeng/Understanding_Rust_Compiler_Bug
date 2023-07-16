plain
2019-12-22T23:52:07.2682999Z do so (now or later) by using -b with the checkout command again. Example:
2019-12-22T23:52:07.2684128Z 
2019-12-22T23:52:07.2684428Z   git checkout -b <new-branch-name>
2019-12-22T23:52:07.2684651Z 
2019-12-22T23:52:07.2685060Z HEAD is now at 864d8dcc3 Auto merge of #67545 - Centril:rollup-t0n3lqk, r=Centril
2019-12-22T23:52:07.3110366Z ##[section]Starting: Setup environment
2019-12-22T23:52:07.3217543Z ==============================================================================
2019-12-22T23:52:07.3217630Z Task         : Bash
2019-12-22T23:52:07.3217716Z Description  : Run a Bash script on macOS, Linux, or Windows
---
2019-12-22T23:53:45.4924079Z Chocolatey installed 0/1 packages. 1 packages failed.
2019-12-22T23:53:45.4924206Z  See the log for details (C:\ProgramData\chocolatey\logs\chocolatey.log).
2019-12-22T23:53:45.4927871Z 
2019-12-22T23:53:45.4933016Z Failures
2019-12-22T23:53:45.4941194Z  - msys2 (exited 1) - msys2 not installed. An error occurred during installation:
2019-12-22T23:53:45.4942107Z  The remote server returned an error: (503) Server Unavailable. Service Unavailable
2019-12-22T23:53:46.0745736Z 
2019-12-22T23:53:46.0827164Z ##[error]Bash exited with code '1'.
2019-12-22T23:53:46.1001394Z ##[section]Starting: Checkout
2019-12-22T23:53:46.1113161Z ==============================================================================
2019-12-22T23:53:46.1113272Z Task         : Get sources
2019-12-22T23:53:46.1113363Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
