plain
2019-07-26T21:15:10.4387391Z do so (now or later) by using -b with the checkout command again. Example:
2019-07-26T21:15:10.4387849Z 
2019-07-26T21:15:10.4388317Z   git checkout -b <new-branch-name>
2019-07-26T21:15:10.4388977Z 
2019-07-26T21:15:10.4389527Z HEAD is now at f051aab34 Auto merge of #63025 - Centril:rollup-10kfdv9, r=Centril
2019-07-26T21:15:10.4578456Z ##[section]Starting: Collect CPU-usage statistics in the background
2019-07-26T21:15:10.4581514Z ==============================================================================
2019-07-26T21:15:10.4581624Z Task         : Bash
2019-07-26T21:15:10.4581696Z Description  : Run a Bash script on macOS, Linux, or Windows
---
2019-07-26T21:20:15.2234545Z Script contents:
2019-07-26T21:20:15.2235074Z aws s3 cp --acl public-read cpu-usage.csv s3://$DEPLOY_BUCKET/rustc-builds/$BUILD_SOURCEVERSION/cpu-$SYSTEM_JOBNAME.csv
2019-07-26T21:20:15.2252810Z ========================== Starting Command Output ===========================
2019-07-26T21:20:15.2277534Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/81439cfa-8a60-4dd9-b85e-b23ccc08eff9.sh
2019-07-26T21:20:15.2369291Z /home/vsts/work/_temp/81439cfa-8a60-4dd9-b85e-b23ccc08eff9.sh: line 1: aws: command not found
2019-07-26T21:20:15.2402985Z ##[error]Bash exited with code '127'.
2019-07-26T21:20:15.2429197Z ##[section]Starting: Checkout
2019-07-26T21:20:15.2430906Z ==============================================================================
2019-07-26T21:20:15.2431011Z Task         : Get sources
2019-07-26T21:20:15.2431089Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
