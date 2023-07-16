plain
2019-10-26T04:44:27.5984284Z do so (now or later) by using -b with the checkout command again. Example:
2019-10-26T04:44:27.5984483Z 
2019-10-26T04:44:27.5984701Z   git checkout -b <new-branch-name>
2019-10-26T04:44:27.5984852Z 
2019-10-26T04:44:27.5985033Z HEAD is now at 8448719bd Auto merge of #65824 - eddyb:def-key-copy, r=varkor
2019-10-26T04:44:27.6341880Z ##[section]Starting: Collect CPU-usage statistics in the background
2019-10-26T04:44:27.6446364Z ==============================================================================
2019-10-26T04:44:27.6446475Z Task         : Bash
2019-10-26T04:44:27.6446549Z Description  : Run a Bash script on macOS, Linux, or Windows
---
2019-10-26T04:46:06.5131906Z Chocolatey installed 0/1 packages. 1 packages failed.
2019-10-26T04:46:06.5132162Z  See the log for details (C:\ProgramData\chocolatey\logs\chocolatey.log).
2019-10-26T04:46:06.5137065Z 
2019-10-26T04:46:06.5140795Z Failures
2019-10-26T04:46:06.5146771Z  - msys2 (exited 1) - msys2 not installed. An error occurred during installation:
2019-10-26T04:46:06.5147040Z  The remote server returned an error: (503) Server Unavailable. Service Unavailable
2019-10-26T04:46:07.0190886Z 
2019-10-26T04:46:07.0263897Z ##[error]Bash exited with code '1'.
2019-10-26T04:46:07.0451373Z ##[section]Starting: Upload CPU usage statistics
2019-10-26T04:46:07.0554390Z ==============================================================================
2019-10-26T04:46:07.0554497Z Task         : Bash
2019-10-26T04:46:07.0554581Z Description  : Run a Bash script on macOS, Linux, or Windows
---
2019-10-26T04:46:07.3278435Z ========================== Starting Command Output ===========================
2019-10-26T04:46:07.3282935Z [command]"C:\Program Files\Git\bin\bash.exe" --noprofile --norc /d/a/_temp/f5eee86c-1361-42b7-8083-01aed6bde0f5.sh
2019-10-26T04:46:07.3693851Z /d/a/_temp/f5eee86c-1361-42b7-8083-01aed6bde0f5.sh: line 1: aws: command not found
2019-10-26T04:46:07.3737521Z 
2019-10-26T04:46:07.3755903Z ##[error]Bash exited with code '127'.
2019-10-26T04:46:07.3821127Z ##[section]Starting: Checkout
2019-10-26T04:46:07.3915528Z ==============================================================================
2019-10-26T04:46:07.3915629Z Task         : Get sources
2019-10-26T04:46:07.3915709Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
