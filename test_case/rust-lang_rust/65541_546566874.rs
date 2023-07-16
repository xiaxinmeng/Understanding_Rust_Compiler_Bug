plain
2019-10-26T04:14:04.5584938Z Chocolatey installed 0/1 packages. 1 packages failed.
2019-10-26T04:14:04.5585316Z  See the log for details (C:\ProgramData\chocolatey\logs\chocolatey.log).
2019-10-26T04:14:04.5589116Z 
2019-10-26T04:14:04.5602542Z Failures
2019-10-26T04:14:04.5602862Z  - msys2 (exited 1) - msys2 not installed. An error occurred during installation:
2019-10-26T04:14:04.5603126Z  The remote server returned an error: (503) Server Unavailable. Service Unavailable
2019-10-26T04:14:04.5617639Z Enjoy using Chocolatey? Explore more amazing features to take your
2019-10-26T04:14:04.5617823Z experience to the next level at
2019-10-26T04:14:04.5618001Z  https://chocolatey.org/compare
2019-10-26T04:14:05.0272762Z 
2019-10-26T04:14:05.0272762Z 
2019-10-26T04:14:05.0351241Z ##[error]Bash exited with code '1'.
2019-10-26T04:14:05.0540548Z ##[section]Starting: Upload CPU usage statistics
2019-10-26T04:14:05.0650305Z ==============================================================================
2019-10-26T04:14:05.0650411Z Task         : Bash
2019-10-26T04:14:05.0650488Z Description  : Run a Bash script on macOS, Linux, or Windows
---
2019-10-26T04:14:05.3419689Z ========================== Starting Command Output ===========================
2019-10-26T04:14:05.3428074Z [command]"C:\Program Files\Git\bin\bash.exe" --noprofile --norc /d/a/_temp/1d371b5c-a6a6-484c-a73b-dc07bef31627.sh
2019-10-26T04:14:05.3875543Z /d/a/_temp/1d371b5c-a6a6-484c-a73b-dc07bef31627.sh: line 1: aws: command not found
2019-10-26T04:14:05.3904153Z 
2019-10-26T04:14:05.3929290Z ##[error]Bash exited with code '127'.
2019-10-26T04:14:05.3998150Z ##[section]Starting: Checkout
2019-10-26T04:14:05.4091391Z ==============================================================================
2019-10-26T04:14:05.4091484Z Task         : Get sources
2019-10-26T04:14:05.4091749Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
