plain
2019-07-31T07:26:53.5894987Z do so (now or later) by using -b with the checkout command again. Example:
2019-07-31T07:26:53.5895058Z 
2019-07-31T07:26:53.5895293Z   git checkout -b <new-branch-name>
2019-07-31T07:26:53.5895339Z 
2019-07-31T07:26:53.5895644Z HEAD is now at 9c61eabad Auto merge of #62429 - cuviper:iter-closures, r=cramertj
2019-07-31T07:26:53.6069552Z ##[section]Starting: Collect CPU-usage statistics in the background
2019-07-31T07:26:53.6073855Z ==============================================================================
2019-07-31T07:26:53.6073971Z Task         : Bash
2019-07-31T07:26:53.6074043Z Description  : Run a Bash script on macOS, Linux, or Windows
---
2019-07-31T07:27:53.9879717Z Script contents:
2019-07-31T07:27:53.9880896Z aws s3 cp --acl public-read cpu-usage.csv s3://$DEPLOY_BUCKET/rustc-builds/$BUILD_SOURCEVERSION/cpu-$SYSTEM_JOBNAME.csv
2019-07-31T07:27:53.9902490Z ========================== Starting Command Output ===========================
2019-07-31T07:27:53.9924130Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/f9816090-450e-46e4-a36c-f85b716596a0.sh
2019-07-31T07:27:54.0025536Z /home/vsts/work/_temp/f9816090-450e-46e4-a36c-f85b716596a0.sh: line 1: aws: command not found
2019-07-31T07:27:54.0119306Z ##[error]Bash exited with code '127'.
2019-07-31T07:27:54.0185200Z ##[section]Starting: Checkout
2019-07-31T07:27:54.0187714Z ==============================================================================
2019-07-31T07:27:54.0188002Z Task         : Get sources
2019-07-31T07:27:54.0188077Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
