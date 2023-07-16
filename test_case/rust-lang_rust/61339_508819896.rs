plain
2019-07-05T17:17:09.7858403Z  * [new tag]             release-0.4        -> release-0.4
2019-07-05T17:17:09.7860087Z  * [new tag]             release-0.5        -> release-0.5
2019-07-05T17:17:09.7861647Z  * [new tag]             release-0.6        -> release-0.6
2019-07-05T17:17:09.7862257Z  * [new tag]             release-0.7        -> release-0.7
2019-07-05T17:17:09.9028245Z ##[command]git checkout --progress --force d8a21c4d14accefd54b2483f4bae01ca4045fb3e
2019-07-05T17:17:09.9030350Z fatal: reference is not a tree: d8a21c4d14accefd54b2483f4bae01ca4045fb3e
2019-07-05T17:17:09.9096661Z ##[warning]Git checkout failed on shallow repository, this might because of git fetch with depth '2' doesn't include the checkout commit 'd8a21c4d14accefd54b2483f4bae01ca4045fb3e'. Please reference documentation (http://go.microsoft.com/fwlink/?LinkId=829603)
2019-07-05T17:17:09.9107826Z ##[error]Git checkout failed with exit code: 128
2019-07-05T17:17:09.9460725Z ##[section]Starting: Upload CPU usage statistics
2019-07-05T17:17:09.9463767Z ==============================================================================
2019-07-05T17:17:09.9463861Z Task         : Bash
2019-07-05T17:17:09.9463942Z Description  : Run a Bash script on macOS, Linux, or Windows
---
2019-07-05T17:17:10.0966307Z Generating script.
2019-07-05T17:17:10.0978953Z Script contents:
2019-07-05T17:17:10.0979459Z aws s3 cp --acl public-read cpu-usage.csv s3://$DEPLOY_BUCKET/rustc-builds/$BUILD_SOURCEVERSION/cpu-$SYSTEM_JOBNAME.csv
2019-07-05T17:17:10.1003639Z ========================== Starting Command Output ===========================
2019-07-05T17:17:10.1016445Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/524038d5-dfa1-4bed-91d3-2c9d52db0a17.sh
2019-07-05T17:17:10.1104860Z /home/vsts/work/_temp/524038d5-dfa1-4bed-91d3-2c9d52db0a17.sh: line 1: aws: command not found
2019-07-05T17:17:10.1138807Z ##[error]Bash exited with code '127'.
2019-07-05T17:17:10.1173237Z ##[section]Starting: Checkout
2019-07-05T17:17:10.1174902Z ==============================================================================
2019-07-05T17:17:10.1174980Z Task         : Get sources
2019-07-05T17:17:10.1175095Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
