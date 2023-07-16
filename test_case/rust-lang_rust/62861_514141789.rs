plain
2019-07-23T09:56:42.9925790Z Script contents:
2019-07-23T09:56:42.9926624Z aws s3 cp --acl public-read cpu-usage.csv s3://$DEPLOY_BUCKET/rustc-builds/$BUILD_SOURCEVERSION/cpu-$SYSTEM_JOBNAME.csv
2019-07-23T09:56:42.9953455Z ========================== Starting Command Output ===========================
2019-07-23T09:56:42.9991604Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/cdd4786c-c6ea-44c5-8271-21cd1b36867d.sh
2019-07-23T09:56:43.0100554Z /home/vsts/work/_temp/cdd4786c-c6ea-44c5-8271-21cd1b36867d.sh: line 1: aws: command not found
2019-07-23T09:56:43.0140206Z ##[error]Bash exited with code '127'.
2019-07-23T09:56:43.0175043Z ##[section]Starting: Checkout
2019-07-23T09:56:43.0176916Z ==============================================================================
2019-07-23T09:56:43.0177021Z Task         : Get sources
2019-07-23T09:56:43.0177101Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
