plain
2019-10-23T19:02:05.4435345Z ##[section]Starting: Windows dist-i686-msvc
2019-10-23T19:02:05.6221903Z ##[section]Starting: Initialize job
2019-10-23T19:02:05.6222152Z Agent name: 'Azure Pipelines 6'
2019-10-23T19:02:05.6222258Z Agent machine name: 'fv-az433'
2019-10-23T19:02:05.6222322Z Current agent version: '2.159.2'
2019-10-23T19:02:05.6237142Z Agent running as: 'VssAdministrator'
2019-10-23T19:02:05.6650315Z Set build variables.
2019-10-23T19:02:05.6689112Z Download all required tasks.
2019-10-23T19:02:05.6825813Z Downloading task: Bash (3.159.3)
2019-10-23T19:02:06.7211306Z Downloading task: CmdLine (2.151.2)
---
2019-10-23T19:03:04.8270998Z  15  331M   15 49.8M    0     0  13.0M      0  0:00:25  0:00:03  0:00:22 13.0M
2019-10-23T19:03:05.8154392Z  17  331M   17 57.4M    0     0  11.9M      0  0:00:27  0:00:04  0:00:23 11.9M
2019-10-23T19:03:06.5420878Z  19  331M   19 65.0M    0     0  11.2M      0  0:00:29  0:00:05  0:00:24 12.9M
2019-10-23T19:03:06.5442784Z  20  331M   20 68.6M    0     0  10.5M      0  0:00:31  0:00:06  0:00:25 11.0M
2019-10-23T19:03:06.5443077Z curl: (56) OpenSSL SSL_read: error:1408F119:SSL routines:ssl3_get_record:decryption failed or bad record mac, errno 0
2019-10-23T19:03:06.5462315Z 
2019-10-23T19:03:06.5462635Z gzip: stdin: unexpected end of file
2019-10-23T19:03:06.5488086Z tar: Unexpected EOF in archive
2019-10-23T19:03:06.5488229Z tar: Unexpected EOF in archive
2019-10-23T19:03:06.5488317Z tar: Error is not recoverable: exiting now
2019-10-23T19:03:06.5522473Z 
2019-10-23T19:03:06.5613424Z ##[error]Bash exited with code '2'.
2019-10-23T19:03:06.5851403Z ##[section]Starting: Upload CPU usage statistics
2019-10-23T19:03:06.5952967Z ==============================================================================
2019-10-23T19:03:06.5953054Z Task         : Bash
2019-10-23T19:03:06.5953115Z Description  : Run a Bash script on macOS, Linux, or Windows
---
2019-10-23T19:03:06.8943669Z ========================== Starting Command Output ===========================
2019-10-23T19:03:06.8948883Z [command]"C:\Program Files\Git\bin\bash.exe" --noprofile --norc /d/a/_temp/ffaaba8f-ddaf-445d-88ed-e636ac3795f8.sh
2019-10-23T19:03:06.9373335Z /d/a/_temp/ffaaba8f-ddaf-445d-88ed-e636ac3795f8.sh: line 1: aws: command not found
2019-10-23T19:03:06.9401417Z 
2019-10-23T19:03:06.9421394Z ##[error]Bash exited with code '127'.
2019-10-23T19:03:06.9493021Z ##[section]Starting: Checkout
2019-10-23T19:03:06.9592266Z ==============================================================================
2019-10-23T19:03:06.9592349Z Task         : Get sources
2019-10-23T19:03:06.9592442Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
