plain
2019-10-24T15:33:05.0356120Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/fd953504-a5d7-4952-875b-b18088978e48.sh
2019-10-24T15:33:05.0701660Z   % Total    % Received % Xferd  Average Speed   Time    Time     Time  Current
2019-10-24T15:33:05.0702783Z                                  Dload  Upload   Total   Spent    Left  Speed
2019-10-24T15:33:05.0702969Z 
2019-10-24T15:33:05.2166968Z   0     0    0     0    0     0      0      0 --:--:-- --:--:-- --:--:--     0curl: (7) Failed to connect to repo.msys2.org port 443: Connection refused
2019-10-24T15:33:05.2233487Z                                  Dload  Upload   Total   Spent    Left  Speed
2019-10-24T15:33:05.2233576Z 
2019-10-24T15:33:05.2233576Z 
2019-10-24T15:33:05.2480866Z   0     0    0     0    0     0      0      0 --:--:-- --:--:-- --:--:--     0curl: (7) Failed to connect to repo.msys2.org port 443: Connection refused
2019-10-24T15:33:05.2514174Z 
2019-10-24T15:33:05.2610724Z ##[error]Bash exited with code '7'.
2019-10-24T15:33:05.2784360Z ##[section]Starting: Upload CPU usage statistics
2019-10-24T15:33:05.2787769Z ==============================================================================
2019-10-24T15:33:05.2788074Z Task         : Bash
2019-10-24T15:33:05.2788153Z Description  : Run a Bash script on macOS, Linux, or Windows
---
2019-10-24T15:33:05.4258496Z Script contents:
2019-10-24T15:33:05.4259035Z aws s3 cp --acl public-read cpu-usage.csv s3://$DEPLOY_BUCKET/rustc-builds/$BUILD_SOURCEVERSION/cpu-$CI_JOB_NAME.csv
2019-10-24T15:33:05.4277954Z ========================== Starting Command Output ===========================
2019-10-24T15:33:05.4297401Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/4eb54fa2-d6bb-4679-bd54-8fd5453b7451.sh
2019-10-24T15:33:09.5772058Z Completed 164 Bytes/164 Bytes (210 Bytes/s) with 1 file(s) remaining
2019-10-24T15:33:09.5773556Z upload: ./cpu-usage.csv to s3://rust-lang-ci2/rustc-builds/40b4897916b2493165b41781219a8904ebbfe40e/cpu-.csv
2019-10-24T15:33:09.6391122Z ##[section]Finishing: Upload CPU usage statistics
2019-10-24T15:33:09.6408671Z ##[section]Starting: Checkout
2019-10-24T15:33:09.6410821Z ==============================================================================
2019-10-24T15:33:09.6410937Z Task         : Get sources
