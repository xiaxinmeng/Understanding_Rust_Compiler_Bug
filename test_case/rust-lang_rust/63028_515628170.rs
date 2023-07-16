plain
2019-07-26T23:19:17.3512337Z do so (now or later) by using -b with the checkout command again. Example:
2019-07-26T23:19:17.3512401Z 
2019-07-26T23:19:17.3512596Z   git checkout -b <new-branch-name>
2019-07-26T23:19:17.3512635Z 
2019-07-26T23:19:17.3513056Z HEAD is now at 92414ad99 Auto merge of #63028 - Centril:rollup-1jekkw7, r=Centril
2019-07-26T23:19:17.3648692Z ##[section]Starting: Collect CPU-usage statistics in the background
2019-07-26T23:19:17.3651419Z ==============================================================================
2019-07-26T23:19:17.3651513Z Task         : Bash
2019-07-26T23:19:17.3651574Z Description  : Run a Bash script on macOS, Linux, or Windows
---
2019-07-26T23:26:31.8840310Z Script contents:
2019-07-26T23:26:31.8840931Z aws s3 cp --acl public-read cpu-usage.csv s3://$DEPLOY_BUCKET/rustc-builds/$BUILD_SOURCEVERSION/cpu-$SYSTEM_JOBNAME.csv
2019-07-26T23:26:31.8860668Z ========================== Starting Command Output ===========================
2019-07-26T23:26:31.8883754Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/a2e07caf-dc5c-4585-8b82-36eadf245e85.sh
2019-07-26T23:26:31.8978949Z /home/vsts/work/_temp/a2e07caf-dc5c-4585-8b82-36eadf245e85.sh: line 1: aws: command not found
2019-07-26T23:26:31.9015014Z ##[error]Bash exited with code '127'.
2019-07-26T23:26:31.9044247Z ##[section]Starting: Checkout
2019-07-26T23:26:31.9045948Z ==============================================================================
2019-07-26T23:26:31.9046038Z Task         : Get sources
2019-07-26T23:26:31.9046101Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
