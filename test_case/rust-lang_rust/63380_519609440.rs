plain
2019-08-08T17:14:05.6220111Z do so (now or later) by using -b with the checkout command again. Example:
2019-08-08T17:14:05.6221655Z 
2019-08-08T17:14:05.6222427Z   git checkout -b <new-branch-name>
2019-08-08T17:14:05.6222725Z 
2019-08-08T17:14:05.6223313Z HEAD is now at c215a1de6 Auto merge of #63380 - Centril:rollup-tzfhtnu, r=Centril
2019-08-08T17:14:05.6408039Z ##[section]Starting: Collect CPU-usage statistics in the background
2019-08-08T17:14:05.6421288Z ==============================================================================
2019-08-08T17:14:05.6421383Z Task         : Bash
2019-08-08T17:14:05.6421470Z Description  : Run a Bash script on macOS, Linux, or Windows
---
2019-08-08T17:15:13.5156339Z Script contents:
2019-08-08T17:15:13.5156852Z aws s3 cp --acl public-read cpu-usage.csv s3://$DEPLOY_BUCKET/rustc-builds/$BUILD_SOURCEVERSION/cpu-$SYSTEM_JOBNAME.csv
2019-08-08T17:15:13.5157247Z ========================== Starting Command Output ===========================
2019-08-08T17:15:13.5159713Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/c9d1048b-2c14-4c99-b9da-00cf5d3c808d.sh
2019-08-08T17:15:13.5160147Z /home/vsts/work/_temp/c9d1048b-2c14-4c99-b9da-00cf5d3c808d.sh: line 1: aws: command not found
2019-08-08T17:15:13.5161196Z ##[error]Bash exited with code '127'.
2019-08-08T17:15:13.5176300Z ##[section]Starting: Checkout
2019-08-08T17:15:13.5178363Z ==============================================================================
2019-08-08T17:15:13.5178516Z Task         : Get sources
2019-08-08T17:15:13.5178649Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
