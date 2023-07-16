plain
2019-07-25T18:43:34.3029647Z Script contents:
2019-07-25T18:43:34.3031837Z aws s3 cp --acl public-read cpu-usage.csv s3://$DEPLOY_BUCKET/rustc-builds/$BUILD_SOURCEVERSION/cpu-$SYSTEM_JOBNAME.csv
2019-07-25T18:43:34.3054120Z ========================== Starting Command Output ===========================
2019-07-25T18:43:34.3071847Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/ba6e28b1-162b-40a0-b930-adfd71c869cf.sh
2019-07-25T18:43:34.3165530Z /home/vsts/work/_temp/ba6e28b1-162b-40a0-b930-adfd71c869cf.sh: line 1: aws: command not found
2019-07-25T18:43:34.3199028Z ##[error]Bash exited with code '127'.
2019-07-25T18:43:34.3231498Z ##[section]Starting: Checkout
2019-07-25T18:43:34.3233357Z ==============================================================================
2019-07-25T18:43:34.3233736Z Task         : Get sources
2019-07-25T18:43:34.3233848Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
