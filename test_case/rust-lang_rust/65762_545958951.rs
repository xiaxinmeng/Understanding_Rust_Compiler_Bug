plain
2019-10-24T14:52:53.0377665Z do so (now or later) by using -b with the checkout command again. Example:
2019-10-24T14:52:53.0378310Z 
2019-10-24T14:52:53.0379018Z   git checkout -b <new-branch-name>
2019-10-24T14:52:53.0380723Z 
2019-10-24T14:52:53.0381312Z HEAD is now at 7d35376b6 Auto merge of #65762 - mati865:msys2-bug, r=Mark-Simulacrum
2019-10-24T14:52:53.0746730Z ##[section]Starting: Collect CPU-usage statistics in the background
2019-10-24T14:52:53.0856093Z ==============================================================================
2019-10-24T14:52:53.0856387Z Task         : Bash
2019-10-24T14:52:53.0856502Z Description  : Run a Bash script on macOS, Linux, or Windows
---
2019-10-24T14:57:21.8110282Z 
2019-10-24T14:57:21.8110660Z ========================== Starting Command Output ===========================
2019-10-24T14:57:21.8130233Z [command]D:\a\msys2\usr\bin\bash.exe --noprofile --norc /d/a/_temp/81eeeedb-7d35-49b5-ae61-1ad255f9269e.sh
2019-10-24T14:57:21.8130443Z 
2019-10-24T14:57:21.8192198Z ##[error]Bash exited with code '1'.
2019-10-24T14:57:21.8323717Z ##[section]Starting: Upload CPU usage statistics
2019-10-24T14:57:21.8428634Z ==============================================================================
2019-10-24T14:57:21.8428761Z Task         : Bash
2019-10-24T14:57:21.8428854Z Description  : Run a Bash script on macOS, Linux, or Windows
---
2019-10-24T14:57:22.1830121Z ========================== Starting Command Output ===========================
2019-10-24T14:57:22.1831259Z [command]D:\a\msys2\usr\bin\bash.exe --noprofile --norc /d/a/_temp/db3099aa-0cc0-4228-9afe-1c3001d41380.sh
2019-10-24T14:57:22.1831480Z /d/a/_temp/db3099aa-0cc0-4228-9afe-1c3001d41380.sh: line 1: aws: command not found
2019-10-24T14:57:22.1831614Z 
2019-10-24T14:57:22.1832621Z ##[error]Bash exited with code '127'.
2019-10-24T14:57:22.1854152Z ##[section]Starting: Checkout
2019-10-24T14:57:22.1958057Z ==============================================================================
2019-10-24T14:57:22.1958167Z Task         : Get sources
2019-10-24T14:57:22.1958283Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
