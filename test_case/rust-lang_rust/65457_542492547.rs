plain
2019-10-16T03:17:04.4203046Z do so (now or later) by using -b with the checkout command again. Example:
2019-10-16T03:17:04.4203554Z 
2019-10-16T03:17:04.4204224Z   git checkout -b <new-branch-name>
2019-10-16T03:17:04.4204330Z 
2019-10-16T03:17:04.4204733Z HEAD is now at 6321db391 Auto merge of #65457 - tmandry:rollup-y1fd3c6, r=tmandry
2019-10-16T03:17:04.4590284Z ##[section]Starting: Collect CPU-usage statistics in the background
2019-10-16T03:17:04.4698340Z ==============================================================================
2019-10-16T03:17:04.4698463Z Task         : Bash
2019-10-16T03:17:04.4698546Z Description  : Run a Bash script on macOS, Linux, or Windows
---
2019-10-16T03:17:09.9542621Z 
2019-10-16T03:17:10.7318595Z   0     0    0     0    0     0      0      0 --:--:-- --:--:-- --:--:--     0
2019-10-16T03:17:11.3047311Z   0 32.7M    0  304k    0     0   390k      0  0:01:25 --:--:--  0:01:25  390k
2019-10-16T03:17:11.3070049Z  64 32.7M   64 21.1M    0     0  15.6M      0  0:00:02  0:00:01  0:00:01 15.6M
2019-10-16T03:17:11.3070737Z curl: (56) OpenSSL SSL_read: error:1408F119:SSL routines:ssl3_get_record:decryption failed or bad record mac, errno 0
2019-10-16T03:17:11.3329671Z ##[error]Bash exited with code '56'.
2019-10-16T03:17:11.3681348Z ##[section]Starting: Upload CPU usage statistics
2019-10-16T03:17:11.3800765Z ==============================================================================
2019-10-16T03:17:11.3800876Z Task         : Bash
2019-10-16T03:17:11.3800957Z Description  : Run a Bash script on macOS, Linux, or Windows
---
2019-10-16T03:17:11.6577660Z [command]"C:\Program Files\Git\bin\bash.exe" --noprofile --norc -c pwd
2019-10-16T03:17:11.6927039Z /d/a/_temp
2019-10-16T03:17:11.6998287Z ========================== Starting Command Output ===========================
2019-10-16T03:17:11.7006718Z [command]"C:\Program Files\Git\bin\bash.exe" --noprofile --norc /d/a/_temp/8f84171e-d159-4e1d-af7e-9a29993d2ef5.sh
2019-10-16T03:17:11.7700053Z /d/a/_temp/8f84171e-d159-4e1d-af7e-9a29993d2ef5.sh: line 1: aws: command not found
2019-10-16T03:17:11.7751406Z ##[error]Bash exited with code '127'.
2019-10-16T03:17:11.7822902Z ##[section]Starting: Checkout
2019-10-16T03:17:11.7929295Z ==============================================================================
2019-10-16T03:17:11.7929414Z Task         : Get sources
2019-10-16T03:17:11.7929506Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
