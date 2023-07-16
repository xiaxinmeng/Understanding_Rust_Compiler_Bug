plain
2019-07-25T18:07:56.5445741Z do so (now or later) by using -b with the checkout command again. Example:
2019-07-25T18:07:56.5445791Z 
2019-07-25T18:07:56.5445996Z   git checkout -b <new-branch-name>
2019-07-25T18:07:56.5446030Z 
2019-07-25T18:07:56.5446270Z HEAD is now at f0e2187e4 Auto merge of #62771 - petrochenkov:depext, r=eddyb
2019-07-25T18:07:56.5603809Z ##[section]Starting: Collect CPU-usage statistics in the background
2019-07-25T18:07:56.5606714Z ==============================================================================
2019-07-25T18:07:56.5606783Z Task         : Bash
2019-07-25T18:07:56.5606853Z Description  : Run a Bash script on macOS, Linux, or Windows
---
2019-07-25T18:11:00.0664966Z Script contents:
2019-07-25T18:11:00.0665395Z aws s3 cp --acl public-read cpu-usage.csv s3://$DEPLOY_BUCKET/rustc-builds/$BUILD_SOURCEVERSION/cpu-$SYSTEM_JOBNAME.csv
2019-07-25T18:11:00.0681231Z ========================== Starting Command Output ===========================
2019-07-25T18:11:00.0700103Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/5b285404-ab84-4e7f-9176-ad8b0820101c.sh
2019-07-25T18:11:00.0774060Z /home/vsts/work/_temp/5b285404-ab84-4e7f-9176-ad8b0820101c.sh: line 1: aws: command not found
2019-07-25T18:11:00.0832442Z ##[error]Bash exited with code '127'.
2019-07-25T18:11:00.0860038Z ##[section]Starting: Checkout
2019-07-25T18:11:00.0861565Z ==============================================================================
2019-07-25T18:11:00.0861631Z Task         : Get sources
2019-07-25T18:11:00.0861722Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
