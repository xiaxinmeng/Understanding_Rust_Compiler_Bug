plain
2019-07-21T18:56:12.5793935Z do so (now or later) by using -b with the checkout command again. Example:
2019-07-21T18:56:12.5794601Z 
2019-07-21T18:56:12.5795189Z   git checkout -b <new-branch-name>
2019-07-21T18:56:12.5795939Z 
2019-07-21T18:56:12.5796612Z HEAD is now at 64561108a Auto merge of #62856 - pietroalbini:fix-awscli, r=Mark-Simulacrum
2019-07-21T18:56:12.5933781Z ##[section]Starting: Collect CPU-usage statistics in the background
2019-07-21T18:56:12.5937420Z ==============================================================================
2019-07-21T18:56:12.5937525Z Task         : Bash
2019-07-21T18:56:12.5937730Z Description  : Run a Bash script on macOS, Linux, or Windows
---
2019-07-21T18:58:00.1842294Z Script contents:
2019-07-21T18:58:00.1842707Z aws s3 cp --acl public-read cpu-usage.csv s3://$DEPLOY_BUCKET/rustc-builds/$BUILD_SOURCEVERSION/cpu-$SYSTEM_JOBNAME.csv
2019-07-21T18:58:00.1860908Z ========================== Starting Command Output ===========================
2019-07-21T18:58:00.1882810Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/40b38685-7bed-4a76-b10c-fb3044d15e17.sh
2019-07-21T18:58:00.1983463Z /home/vsts/work/_temp/40b38685-7bed-4a76-b10c-fb3044d15e17.sh: line 1: aws: command not found
2019-07-21T18:58:00.2020173Z ##[error]Bash exited with code '127'.
2019-07-21T18:58:00.2049296Z ##[section]Starting: Checkout
2019-07-21T18:58:00.2051846Z ==============================================================================
2019-07-21T18:58:00.2051931Z Task         : Get sources
2019-07-21T18:58:00.2052046Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
