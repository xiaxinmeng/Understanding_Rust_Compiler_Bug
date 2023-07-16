plain
2019-10-26T07:18:53.1399271Z do so (now or later) by using -b with the checkout command again. Example:
2019-10-26T07:18:53.1400079Z 
2019-10-26T07:18:53.1403308Z   git checkout -b <new-branch-name>
2019-10-26T07:18:53.1404331Z 
2019-10-26T07:18:53.1404938Z HEAD is now at cf96b4bad Auto merge of #65566 - estebank:let-expr-as-ty, r=Centril
2019-10-26T07:18:53.2013734Z ##[section]Starting: Collect CPU-usage statistics in the background
2019-10-26T07:18:53.2276470Z ==============================================================================
2019-10-26T07:18:53.2276592Z Task         : Bash
2019-10-26T07:18:53.2276677Z Description  : Run a Bash script on macOS, Linux, or Windows
---
2019-10-26T07:20:32.8004531Z Chocolatey installed 0/1 packages. 1 packages failed.
2019-10-26T07:20:32.8004965Z  See the log for details (C:\ProgramData\chocolatey\logs\chocolatey.log).
2019-10-26T07:20:32.8011060Z 
2019-10-26T07:20:32.8017389Z Failures
2019-10-26T07:20:32.8024735Z  - msys2 (exited 1) - msys2 not installed. An error occurred during installation:
2019-10-26T07:20:32.8025305Z  The remote server returned an error: (503) Server Unavailable. Service Unavailable
2019-10-26T07:20:33.3100079Z 
2019-10-26T07:20:33.3202189Z ##[error]Bash exited with code '1'.
2019-10-26T07:20:33.3437294Z ##[section]Starting: Upload CPU usage statistics
2019-10-26T07:20:33.3549928Z ==============================================================================
2019-10-26T07:20:33.3550044Z Task         : Bash
2019-10-26T07:20:33.3550116Z Description  : Run a Bash script on macOS, Linux, or Windows
---
2019-10-26T07:20:33.6908889Z ========================== Starting Command Output ===========================
2019-10-26T07:20:33.6914878Z [command]"C:\Program Files\Git\bin\bash.exe" --noprofile --norc /d/a/_temp/a38c3b31-4a84-465b-89b3-e2fc1ba236e7.sh
2019-10-26T07:20:33.7363987Z /d/a/_temp/a38c3b31-4a84-465b-89b3-e2fc1ba236e7.sh: line 1: aws: command not found
2019-10-26T07:20:33.7391556Z 
2019-10-26T07:20:33.7410693Z ##[error]Bash exited with code '127'.
2019-10-26T07:20:33.7487889Z ##[section]Starting: Checkout
2019-10-26T07:20:33.7639758Z ==============================================================================
2019-10-26T07:20:33.7639868Z Task         : Get sources
2019-10-26T07:20:33.7639973Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
