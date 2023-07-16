plain
2019-10-26T04:17:49.5008219Z Chocolatey installed 0/1 packages. 1 packages failed.
2019-10-26T04:17:49.5008305Z  See the log for details (C:\ProgramData\chocolatey\logs\chocolatey.log).
2019-10-26T04:17:49.5012992Z 
2019-10-26T04:17:49.5018172Z Failures
2019-10-26T04:17:49.5024695Z  - msys2 (exited 1) - msys2 not installed. An error occurred during installation:
2019-10-26T04:17:49.5024800Z  The remote server returned an error: (503) Server Unavailable. Service Unavailable
2019-10-26T04:17:49.9565193Z 
2019-10-26T04:17:49.9645340Z ##[error]Bash exited with code '1'.
2019-10-26T04:17:49.9835669Z ##[section]Starting: Upload CPU usage statistics
2019-10-26T04:17:49.9933052Z ==============================================================================
2019-10-26T04:17:49.9933130Z Task         : Bash
2019-10-26T04:17:49.9933191Z Description  : Run a Bash script on macOS, Linux, or Windows
---
2019-10-26T04:17:50.2569935Z ========================== Starting Command Output ===========================
2019-10-26T04:17:50.2575964Z [command]"C:\Program Files\Git\bin\bash.exe" --noprofile --norc /d/a/_temp/ed355e7c-91ff-45bb-92b7-6cac0941ffb9.sh
2019-10-26T04:17:50.3005826Z /d/a/_temp/ed355e7c-91ff-45bb-92b7-6cac0941ffb9.sh: line 1: aws: command not found
2019-10-26T04:17:50.3035085Z 
2019-10-26T04:17:50.3053622Z ##[error]Bash exited with code '127'.
2019-10-26T04:17:50.3121730Z ##[section]Starting: Checkout
2019-10-26T04:17:50.3235998Z ==============================================================================
2019-10-26T04:17:50.3236118Z Task         : Get sources
2019-10-26T04:17:50.3236211Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
