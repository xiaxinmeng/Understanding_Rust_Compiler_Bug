plain
2019-07-25T05:47:06.7894973Z do so (now or later) by using -b with the checkout command again. Example:
2019-07-25T05:47:06.7895048Z 
2019-07-25T05:47:06.7895760Z   git checkout -b <new-branch-name>
2019-07-25T05:47:06.7895811Z 
2019-07-25T05:47:06.7896372Z HEAD is now at c251000e8 Auto merge of #62960 - Centril:rollup-8jay7g7, r=Centril
2019-07-25T05:47:06.8047994Z ##[section]Starting: Collect CPU-usage statistics in the background
2019-07-25T05:47:06.8051164Z ==============================================================================
2019-07-25T05:47:06.8051491Z Task         : Bash
2019-07-25T05:47:06.8051559Z Description  : Run a Bash script on macOS, Linux, or Windows
---
2019-07-25T05:50:25.4472717Z Script contents:
2019-07-25T05:50:25.4473102Z aws s3 cp --acl public-read cpu-usage.csv s3://$DEPLOY_BUCKET/rustc-builds/$BUILD_SOURCEVERSION/cpu-$SYSTEM_JOBNAME.csv
2019-07-25T05:50:25.4473402Z ========================== Starting Command Output ===========================
2019-07-25T05:50:25.4530194Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/13e94c2f-0e78-43be-9f5e-c1342dfe8235.sh
2019-07-25T05:50:25.4531230Z /home/vsts/work/_temp/13e94c2f-0e78-43be-9f5e-c1342dfe8235.sh: line 1: aws: command not found
2019-07-25T05:50:25.4532600Z ##[error]Bash exited with code '127'.
2019-07-25T05:50:25.4547330Z ##[section]Starting: Checkout
2019-07-25T05:50:25.4549011Z ==============================================================================
2019-07-25T05:50:25.4549114Z Task         : Get sources
2019-07-25T05:50:25.4549191Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
