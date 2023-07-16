plain
2019-07-21T07:05:33.5721986Z do so (now or later) by using -b with the checkout command again. Example:
2019-07-21T07:05:33.5722040Z 
2019-07-21T07:05:33.5722323Z   git checkout -b <new-branch-name>
2019-07-21T07:05:33.5722370Z 
2019-07-21T07:05:33.5722683Z HEAD is now at fd584a066 Auto merge of #62840 - Centril:rollup-5ettkjy, r=Centril
2019-07-21T07:05:33.5875289Z ##[section]Starting: Collect CPU-usage statistics in the background
2019-07-21T07:05:33.5879069Z ==============================================================================
2019-07-21T07:05:33.5879164Z Task         : Bash
2019-07-21T07:05:33.5879248Z Description  : Run a Bash script on macOS, Linux, or Windows
---
2019-07-21T07:08:55.3050814Z Script contents:
2019-07-21T07:08:55.3051445Z aws s3 cp --acl public-read cpu-usage.csv s3://$DEPLOY_BUCKET/rustc-builds/$BUILD_SOURCEVERSION/cpu-$SYSTEM_JOBNAME.csv
2019-07-21T07:08:55.3071268Z ========================== Starting Command Output ===========================
2019-07-21T07:08:55.3092118Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/fc63736e-cfb2-44ae-a9f2-7c0cd287aa91.sh
2019-07-21T07:08:55.3186915Z /home/vsts/work/_temp/fc63736e-cfb2-44ae-a9f2-7c0cd287aa91.sh: line 1: aws: command not found
2019-07-21T07:08:55.3222103Z ##[error]Bash exited with code '127'.
2019-07-21T07:08:55.3255938Z ##[section]Starting: Checkout
2019-07-21T07:08:55.3257759Z ==============================================================================
2019-07-21T07:08:55.3257847Z Task         : Get sources
2019-07-21T07:08:55.3257948Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
