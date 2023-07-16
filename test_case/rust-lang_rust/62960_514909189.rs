plain
2019-07-25T05:57:17.0170549Z do so (now or later) by using -b with the checkout command again. Example:
2019-07-25T05:57:17.0170651Z 
2019-07-25T05:57:17.0171074Z   git checkout -b <new-branch-name>
2019-07-25T05:57:17.0171178Z 
2019-07-25T05:57:17.0171651Z HEAD is now at fa5c15a1c Auto merge of #62960 - Centril:rollup-8jay7g7, r=Centril
2019-07-25T05:57:17.0365932Z ##[section]Starting: Collect CPU-usage statistics in the background
2019-07-25T05:57:17.0369522Z ==============================================================================
2019-07-25T05:57:17.0369614Z Task         : Bash
2019-07-25T05:57:17.0369704Z Description  : Run a Bash script on macOS, Linux, or Windows
---
2019-07-25T06:00:32.7706237Z Script contents:
2019-07-25T06:00:32.7706640Z aws s3 cp --acl public-read cpu-usage.csv s3://$DEPLOY_BUCKET/rustc-builds/$BUILD_SOURCEVERSION/cpu-$SYSTEM_JOBNAME.csv
2019-07-25T06:00:32.7732005Z ========================== Starting Command Output ===========================
2019-07-25T06:00:32.7749981Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/d3266e5d-f274-4d43-b947-e537dc6c1b7c.sh
2019-07-25T06:00:32.7825546Z /home/vsts/work/_temp/d3266e5d-f274-4d43-b947-e537dc6c1b7c.sh: line 1: aws: command not found
2019-07-25T06:00:32.7903098Z ##[error]Bash exited with code '127'.
2019-07-25T06:00:32.7937156Z ##[section]Starting: Checkout
2019-07-25T06:00:32.7939056Z ==============================================================================
2019-07-25T06:00:32.7939138Z Task         : Get sources
2019-07-25T06:00:32.7939229Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
