plain
2019-07-25T06:09:19.0335438Z do so (now or later) by using -b with the checkout command again. Example:
2019-07-25T06:09:19.0335491Z 
2019-07-25T06:09:19.0336118Z   git checkout -b <new-branch-name>
2019-07-25T06:09:19.0336160Z 
2019-07-25T06:09:19.0337340Z HEAD is now at 52089343d Auto merge of #62960 - Centril:rollup-8jay7g7, r=Centril
2019-07-25T06:09:19.0514670Z ##[section]Starting: Collect CPU-usage statistics in the background
2019-07-25T06:09:19.0517908Z ==============================================================================
2019-07-25T06:09:19.0518020Z Task         : Bash
2019-07-25T06:09:19.0518244Z Description  : Run a Bash script on macOS, Linux, or Windows
---
2019-07-25T06:12:18.8715932Z Script contents:
2019-07-25T06:12:18.8717733Z aws s3 cp --acl public-read cpu-usage.csv s3://$DEPLOY_BUCKET/rustc-builds/$BUILD_SOURCEVERSION/cpu-$SYSTEM_JOBNAME.csv
2019-07-25T06:12:18.8734861Z ========================== Starting Command Output ===========================
2019-07-25T06:12:18.8753756Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/5514c190-dee3-491e-9746-e51d16a9a880.sh
2019-07-25T06:12:18.8847772Z /home/vsts/work/_temp/5514c190-dee3-491e-9746-e51d16a9a880.sh: line 1: aws: command not found
2019-07-25T06:12:18.8881916Z ##[error]Bash exited with code '127'.
2019-07-25T06:12:18.8911543Z ##[section]Starting: Checkout
2019-07-25T06:12:18.8913252Z ==============================================================================
2019-07-25T06:12:18.8913366Z Task         : Get sources
2019-07-25T06:12:18.8913435Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
