plain
2019-07-24T09:58:34.0716998Z do so (now or later) by using -b with the checkout command again. Example:
2019-07-24T09:58:34.0717053Z 
2019-07-24T09:58:34.0717308Z   git checkout -b <new-branch-name>
2019-07-24T09:58:34.0717557Z 
2019-07-24T09:58:34.0717899Z HEAD is now at 76da8ad20 Auto merge of #62923 - Centril:rollup-53i3am3, r=Centril
2019-07-24T09:58:34.0858953Z ##[section]Starting: Collect CPU-usage statistics in the background
2019-07-24T09:58:34.0861989Z ==============================================================================
2019-07-24T09:58:34.0862082Z Task         : Bash
2019-07-24T09:58:34.0862157Z Description  : Run a Bash script on macOS, Linux, or Windows
---
2019-07-24T10:01:44.9020940Z Script contents:
2019-07-24T10:01:44.9021701Z aws s3 cp --acl public-read cpu-usage.csv s3://$DEPLOY_BUCKET/rustc-builds/$BUILD_SOURCEVERSION/cpu-$SYSTEM_JOBNAME.csv
2019-07-24T10:01:44.9042567Z ========================== Starting Command Output ===========================
2019-07-24T10:01:44.9061190Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/2ffe1be6-ce13-482b-a8d8-527a86b73b29.sh
2019-07-24T10:01:44.9153843Z /home/vsts/work/_temp/2ffe1be6-ce13-482b-a8d8-527a86b73b29.sh: line 1: aws: command not found
2019-07-24T10:01:44.9188811Z ##[error]Bash exited with code '127'.
2019-07-24T10:01:44.9219057Z ##[section]Starting: Checkout
2019-07-24T10:01:44.9220961Z ==============================================================================
2019-07-24T10:01:44.9221050Z Task         : Get sources
2019-07-24T10:01:44.9221168Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
